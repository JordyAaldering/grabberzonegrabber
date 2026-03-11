mod issue;

use std::{collections::HashMap, fs, path::{Path, PathBuf}};

use clap::Parser;
use futures::future;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};

use crate::issue::download_issue;

#[derive(Parser)]
struct Args {
    /// Dry run: search for images without downloading or creating files.
    #[arg(long)]
    dry: bool,

    /// HTML class used to identify comic page images.
    #[arg(long, default_value = "wp-manga-chapter-img")]
    html_image_class: String,

    /// Output directory.
    ///
    /// If not specified, the output directory will be derived from the URL.
    #[arg(short('o'), long("out"))]
    out_dir: Option<PathBuf>,

    #[arg()]
    url: String,
}

async fn get_html(client: &Client, url: &str) -> reqwest::Result<String> {
    let resp = client.get(url).send().await?;
    resp.text().await
}

fn extract_issue_links(base: &str, html: &str) -> HashMap<String, String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(r#"a[href]:not([href^="javascript:"])"#).unwrap();

    let re = Regex::new(&format!(r"{}/?([^/]+)/?$", base)).unwrap();

    let mut links = HashMap::new();

    for el in document.select(&selector) {
        if let Some(mut url) = el.value().attr("href") {
            url = url.trim();
            if let Some(caps) = re.captures(url) {
                let issue = &caps[1];
                if let Some(prev) = links.insert(issue.to_owned(), url.to_owned()) {
                    if prev != url {
                        log::warn!("Duplicate issue {} with mismatching URLs: {} and {}", issue, prev, url);
                    }
                } else {
                    log::info!("Found issue: {}", issue);
                }
            }
        }
    }

    links
}

async fn download_collection(client: &Client, url: &str, html_image_class: &str, out_dir: &Path, dry: bool) {
    log::info!("Fetching collection {}", url);
    let text = get_html(client, url).await.unwrap();
    let links = extract_issue_links(url, &text);
    log::info!("Found {} issues", links.len());

    let futures = links.iter().map(|(issue, link)| {
        download_issue(client, link, html_image_class, issue, out_dir, dry)
    }).collect::<Vec<_>>();

    future::join_all(futures).await;
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let Args { dry, html_image_class, out_dir, url } = Args::parse();

    let client = Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .unwrap();

    let out_dir = out_dir.unwrap_or_else(|| {
        let collection_name = url.rsplit('/').find(|s| !s.is_empty()).unwrap_or("collection");
        PathBuf::from(collection_name)
    });

    if !dry {
        log::info!("Writing to: {}", out_dir.display());
        fs::create_dir_all(&out_dir).unwrap();
    }

    download_collection(&client, &url, &html_image_class, &out_dir, dry).await;
}
