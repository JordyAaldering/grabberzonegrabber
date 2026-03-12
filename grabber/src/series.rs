use std::{collections::HashMap, path::Path};

use futures::future;
use reqwest::Client;
use sanitize_filename::sanitize;
use scraper::{Html, Selector};

use crate::issue::download_issue;

async fn get_html(client: &Client, url: &str) -> reqwest::Result<String> {
    let resp = client.get(url).send().await?;
    resp.text().await
}

/// The series webpage is expected to contain a list of all issues of the comic book.
/// These issues are expected to have a unique `li` class.
///
/// If possible, we extract the human-readable issue name.
///
/// ### Example
///
/// ```html
/// <li class="wp-manga-chapter has-thumb">
///   <div class="chapter-thumbnail">
///     <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-247/">
///       <img class="thumb"
///            src="https://grabber.zone/wp-content/uploads/thumb-1414-75x106.jpg">
///     </a>
///   </div>
///   <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-247/">
///   </a>
///   <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-247/">
///     Sonic the Hedgehog 247
///   </a>
///   <span class="chapter-release-date">
///     <i>November 24, 2021</i>
///   </span>
/// </li>
/// ```
fn extract_issue_links(html: &str, html_issue_class: &str) -> HashMap<String, String> {
    let document = Html::parse_document(html);
    let issue_selector = Selector::parse(&format!("li.{html_issue_class}")).unwrap();
    let href_selector = Selector::parse(r#"a[href]:not([href^="javascript:"])"#).unwrap();

    document.select(&issue_selector)
        .filter_map(|li| {
            li.select(&href_selector)
                .find_map(|href| {
                    href.text().find_map(|t| {
                        if t.trim().is_empty() {
                            None
                        } else {
                            let title = t.trim().to_string();
                            // The `href_selector` only matches on `href`; unwrap should be okay
                            let url = href.value().attr("href").unwrap().to_string();
                            Some((title, url))
                        }
                    })
                })
        })
        .map(|(issue, url)| (sanitize(issue), url))
        .inspect(|(issue, url)| log::trace!("{}: {}", issue, url))
        .collect()
}

pub async fn download_series(client: &Client, url: &str, html_issue_class: &str, html_image_class: &str, out_dir: &Path, dry: bool) {
    log::info!("Fetching series {}", url);
    let text = get_html(client, url).await.unwrap();
    let links = extract_issue_links(&text, html_issue_class);
    log::info!("Found {} issues", links.len());

    let futures = links.iter().map(|(issue, link)| {
        download_issue(client, link, html_image_class, issue, out_dir, dry)
    }).collect::<Vec<_>>();

    future::join_all(futures).await;
}
