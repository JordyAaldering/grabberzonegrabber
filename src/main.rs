use std::{fs, path::{Path, PathBuf}};

use clap::Parser;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};

#[derive(Parser)]
struct Args {
    /// Dry run: search for images without downloading.
    #[arg(long)]
    dry: bool,

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
                println!("Found page {}: {}", page, src);
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

#[tokio::main]
async fn main() {
    let Args { dry, re, out, url } = Args::parse();
    let re = Regex::new(&re).expect("Invalid regex");
    if !dry {
        fs::create_dir_all(&out).unwrap();
    }

    let client = Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .unwrap();

    println!("Fetching {}", url);
    let text = get_html(&client, &url).await.unwrap();
    let imgs = extract_image_urls(&text, &re);
    println!("Found {} images", imgs.len());

    for (page, img) in imgs {
        let path = out.join(format!("{:03}.jpg", page));
        println!("Downloading {} to {}", img, path.display());

        if !dry {
            download_image(&client, &img, &path).await.unwrap();
        }
    }
}
