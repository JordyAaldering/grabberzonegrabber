mod issue;

use std::{collections::HashMap, fs, path::{Path, PathBuf}};

use clap::Parser;
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
    out_dir: PathBuf,

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
                if links.contains_key(issue) {
                    // Duplicates may occur, that is no problem
                    links.insert(issue.to_owned(), url.to_owned());
                    println!("Found issue: {}", url);
                }
            }
        }
    }

    links
}

async fn download_collection(client: &Client, url: &str, re: &Regex, out_dir: &Path) {
    println!("Fetching collection {}", url);
    let text = get_html(client, url).await.unwrap();
    let links = extract_issue_links(url, &text);
    println!("Found {} issues", links.len());

    for (issue, link) in links {
        download_issue(client, &link, re, &issue, &out_dir).await.unwrap();
        break;
    }
}

#[tokio::main]
async fn main() {
    let Args { dry: _, issue, re, out_dir, url } = Args::parse();

    fs::create_dir_all(&out_dir).unwrap();

    let client = Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .unwrap();

    if issue {
        download_issue(&client, &url, &re, "issue", &out_dir).await.unwrap();
    } else {
        download_collection(&client, &url, &re, &out_dir).await;
    }
}
