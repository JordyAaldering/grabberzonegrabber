use std::{collections::HashMap, path::Path};

use futures::future;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};

use crate::issue::download_issue;

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

pub async fn download_series(client: &Client, url: &str, html_image_class: &str, out_dir: &Path, dry: bool) {
    log::info!("Fetching series {}", url);
    let text = get_html(client, url).await.unwrap();
    let links = extract_issue_links(url, &text);
    log::info!("Found {} issues", links.len());

    let futures = links.iter().map(|(issue, link)| {
        download_issue(client, link, html_image_class, issue, out_dir, dry)
    }).collect::<Vec<_>>();

    future::join_all(futures).await;
}
