mod issue;

use std::{collections::HashMap, fs, path::{Path, PathBuf}};

use clap::Parser;
use futures::future;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};

use crate::issue::download_issue;

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
    /// * `.*`: match anything at the start.<br/>
    /// * `(?:-|/)`: non-capturing group for page number separator.
    /// * `(\d+)`: the digits we want to extract that represent the page number.
    /// * `\.`: literal dot before the file extension.
    /// * `(?:jpg|jpeg|png|webp)`: non-capturing group for the file extension.
    /// * `$`: end of string.
    ///
    /// ### Edge case:
    /// In some cases, the image URLs may not actually end with e.g. `/number.png`.
    /// Sometimes, the URL may end with something that looks like `123abc456def.png`.
    /// If no images are found, try replacing `(\d+)` with `(\w+)`, which will match any alphanumeric character.
    /// If this cannot be converted to a number, images are assumed to appear in order.
    #[arg(long, value_parser = parse_regex, default_value = r"https://grabber.zone/wp-content/uploads/WP-manga/data/.*(?:-|/)(\d+)\.(?:jpg|jpeg|png|webp)$", verbatim_doc_comment)]
    re: Regex,

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

async fn download_collection(client: &Client, url: &str, re: &Regex, out_dir: &Path) {
    log::info!("Fetching collection {}", url);
    let text = get_html(client, url).await.unwrap();
    let links = extract_issue_links(url, &text);
    log::info!("Found {} issues", links.len());

    let futures = links.iter().map(|(issue, link)| {
        download_issue(client, link, re, issue, out_dir)
    }).collect::<Vec<_>>();

    future::join_all(futures).await;
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let Args { dry: _, issue, re, out_dir, url } = Args::parse();

    let client = Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .unwrap();

    if issue {
        let mut iter = url.rsplit('/');
        let issue_name = iter.find(|s| !s.is_empty()).unwrap_or("issue");
        let out_dir = out_dir.unwrap_or_else(|| {
            let collection_name = iter.next().unwrap_or("collection");
            PathBuf::from(collection_name)
        });
        log::info!("Writing to: {}/", out_dir.display());
        fs::create_dir_all(&out_dir).unwrap();

        download_issue(&client, &url, &re, issue_name, &out_dir).await.unwrap();
    } else {
        let out_dir = out_dir.unwrap_or_else(|| {
            let collection_name = url.rsplit('/').find(|s| !s.is_empty()).unwrap_or("collection");
            PathBuf::from(collection_name)
        });
        log::info!("Writing to: {}/", out_dir.display());
        fs::create_dir_all(&out_dir).unwrap();

        download_collection(&client, &url, &re, &out_dir).await;
    }
}
