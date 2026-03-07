use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}};

use clap::Parser;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use zip::{ZipWriter, write::SimpleFileOptions};

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
    #[arg(long, default_value = r".*(?:-|/)(\d+)\.(?:jpg|jpeg|png|webp)$")]
    re: String,

    /// Output directory.
    #[arg(long, default_value = "downloads")]
    out: PathBuf,

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

async fn download_image(client: &Client, url: &str, path: &Path) -> reqwest::Result<()> {
    let resp = client.get(url).send().await?;
    let bytes = resp.bytes().await?;
    fs::write(path, &bytes).unwrap();
    Ok(())
}

fn create_cbz(imgs_dir: &Path, cbz_dst: &Path) -> zip::result::ZipResult<()> {
    let entries: Vec<_> = fs::read_dir(imgs_dir)?
        .map(|e| e.unwrap().path())
        .collect();
    //entries.sort();

    // TODO: currently read_dir MUST go before this, because it is places in the same directory...
    // Can be resolved once we use a temp directory for downloaded images
    let file = File::create(cbz_dst)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default();

    for entry in entries {
        println!("Writing {} to cbz", entry.display());
        // TODO: this can be safer by using the path extracted from the regex
        let name = entry.file_name().unwrap().to_string_lossy();
        let data = fs::read(&entry)?;
        zip.start_file(name, options)?;
        zip.write_all(&data)?;
    }

    zip.finish()?;
    Ok(())
}

fn extract_issue_links(base: &str, html: &str) -> Vec<(String, String)> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(r#"a[href]:not([href^="javascript:"])"#).unwrap();

    let re = Regex::new(&format!(r"{base}/?([^/]+)/?$")).unwrap();

    let mut links = Vec::new();

    for el in document.select(&selector) {
        if let Some(mut url) = el.value().attr("href") {
            url = url.trim();
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

async fn download_issue(client: &Client, url: &str, re: &Regex, out: &Path, dry: bool) {
    println!("Fetching issue {}", url);
    let text = get_html(&client, &url).await.unwrap();
    let imgs = extract_image_urls(&text, &re);
    println!("Found {} images", imgs.len());

    if dry {
        return;
    }

    // TODO: is there a generic way to create a temp directory?
    fs::create_dir_all(&out).unwrap();

    for (page, img) in imgs {
        let imp_path = out.join(format!("{:03}.jpg", page));
        println!("Downloading {} to {}", img, imp_path.display());
        download_image(&client, &img, &imp_path).await.unwrap();
    }

    let cbz_path = out.join(format!("out.cbz"));
    println!("Creating cbz {}", cbz_path.display());
    create_cbz(&out, &cbz_path).unwrap();
}

async fn download_collection(client: &Client, url: &str, re: &Regex, out: &Path, dry: bool) {
    println!("Fetching collection {}", url);
    let text = get_html(client, url).await.unwrap();
    let links = extract_issue_links(url, &text);
    println!("Found {} issues", links.len());

    for (issue, link) in links {
        let issue_out = out.join(&issue);
        println!("Downloading issue {} to {}", issue, issue_out.display());
        download_issue(client, &link, re, &issue_out, dry).await;
    }
}

#[tokio::main]
async fn main() {
    let Args { dry, issue, re, out, url } = Args::parse();
    // TODO: args.re should be Regex, not String. Needs a custom value_parser
    let re = Regex::new(&re).expect("Invalid regex");

    let client = Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .unwrap();

    if issue {
        download_issue(&client, &url, &re, &out, dry).await;
    } else {
        download_collection(&client, &url, &re, &out, dry).await;
    }
}
