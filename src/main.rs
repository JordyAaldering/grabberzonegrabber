use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}};

use clap::Parser;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use zip::{ZipWriter, write::SimpleFileOptions};

fn parse_regex(re: &str) -> Result<Regex, String> {
    Regex::new(re).map_err(|e| e.to_string())
}

#[derive(Parser)]
struct Args {
    /// Dry run: search for images without downloading.
    #[arg(long)]
    dry: bool,

    /// Fetch only a single issue, not a whole collection.
    #[arg(long)]
    issue: bool,

    /// Regex to match image URLs.
    ///
    /// ### Example:
    /// * `.*`: match anything at the start.
    /// * `(?:-|/)`: non-capturing group for page number separator.
    /// * `(\d+)`: the digits we want to extract that represent the page number.
    /// * `\.`: literal dot before the file extension.
    /// * `(?:jpg|jpeg|png|webp)`: non-capturing group for the file extension.
    /// * `$`: end of string.
    #[arg(long, value_parser = parse_regex, default_value = r".*(?:-|/)(\d+)\.(?:jpg|jpeg|png|webp)$")]
    re: Regex,

    /// Output directory.
    #[arg(short('o'), long("out"), default_value = "downloads")]
    cbz_out_dir: PathBuf,

    #[arg()]
    url: String,
}

async fn get_html(client: &Client, url: &str) -> reqwest::Result<String> {
    let resp = client.get(url).send().await?;
    resp.text().await
}

fn extract_image_urls(html: &str, re: &Regex) -> Vec<(usize, String)> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("img").unwrap();

    let mut imgs = Vec::new();

    for el in document.select(&selector) {
        if let Some(mut src) = el.value().attr("data-src").or(el.value().attr("src")) {
            src = src.trim();
            if let Some(caps) = re.captures(src) {
                let page = caps[1].parse().unwrap();
                println!("Page {}: {}", page, src);
                imgs.push((page, src.to_owned()));
            }
        }
    }

    imgs
}

async fn download_image(client: &Client, url: &str) -> reqwest::Result<Vec<u8>> {
    let resp = client.get(url).send().await?;
    let bytes = resp.bytes().await?;

    let img = if let Ok(ext) = image::ImageFormat::from_path(PathBuf::from(url)) {
        image::load_from_memory_with_format(&bytes, ext).unwrap()
    } else {
        image::load_from_memory(&bytes).unwrap()
    };

    let mut img_bytes = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut img_bytes), image::ImageFormat::WebP).unwrap();
    Ok(img_bytes)
}

fn extract_issue_links(base: &str, html: &str) -> Vec<(String, String)> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(r#"a[href]:not([href^="javascript:"])"#).unwrap();

    let re = Regex::new(&format!(r"{}/?([^/]+)/?$", base)).unwrap();

    let mut links = Vec::new();

    for el in document.select(&selector) {
        if let Some(mut url) = el.value().attr("href") {
            url = url.trim();
            println!("Found link: {}", url);
            if let Some(caps) = re.captures(url) {
                let issue = &caps[1];
                // println!("Issue {}: {}", &issue, url);
                links.push((issue.to_owned(), url.to_owned()));
            }
        }
    }

    links.sort();
    links.dedup();
    links
}

async fn download_issue(client: &Client, url: &str, re: &Regex, issue_name: &str, cbz_out_dir: &Path) -> zip::result::ZipResult<()> {
    println!("Fetching issue {} from {}", issue_name, url);
    let text = get_html(&client, &url).await.unwrap();
    let imgs = extract_image_urls(&text, &re);
    println!("Found {} images", imgs.len());

    fs::create_dir_all(cbz_out_dir).unwrap();
    let cbz_dst = cbz_out_dir.join(format!("{}.cbz", issue_name));
    println!("Creating cbz {}", cbz_dst.display());

    let file = File::create(&cbz_dst)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default();

    for (page, img) in imgs {
        println!("Downloading {}", img);
        let img_data = download_image(&client, &img).await.unwrap();

        let name = format!("{:03}.webp", page);
        println!("Writing {} to {}", name, cbz_dst.display());
        zip.start_file(name, options)?;
        zip.write_all(&img_data)?;
    }

    zip.finish()?;
    Ok(())
}

async fn download_collection(client: &Client, url: &str, re: &Regex, cbz_out_dir: &Path) {
    println!("Fetching collection {}", url);
    let text = get_html(client, url).await.unwrap();
    let links = extract_issue_links(url, &text);
    println!("Found {} issues", links.len());

    for (issue, link) in links {
        download_issue(client, &link, re, &issue, &cbz_out_dir).await.unwrap();
        break;
    }
}

#[tokio::main]
async fn main() {
    let Args { dry: _, issue, re, cbz_out_dir, url } = Args::parse();

    let client = Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .unwrap();

    if issue {
        download_issue(&client, &url, &re, "issue", &cbz_out_dir).await.unwrap();
    } else {
        download_collection(&client, &url, &re, &cbz_out_dir).await;
    }
}
