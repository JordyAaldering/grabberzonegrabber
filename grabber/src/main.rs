mod issue;
mod series;

use std::{fs, path::PathBuf};

use clap::Parser;
use reqwest::Client;

#[derive(Parser)]
struct Args {
    /// Dry run: search for images without downloading or creating files.
    #[arg(long)]
    dry: bool,

    /// HTML class used to identify issues from the series page.
    #[arg(long, default_value = "wp-manga-chapter")]
    html_issue_class: String,

    /// HTML class used to identify pages from the issue page.
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

#[tokio::main]
async fn main() {
    env_logger::init();

    let Args { dry, html_issue_class, html_image_class, out_dir, url } = Args::parse();

    let client = Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .unwrap();

    let out_dir = out_dir.unwrap_or_else(|| {
        let series_name = url.rsplit('/').find(|s| !s.is_empty()).unwrap_or("series");
        PathBuf::from(series_name)
    });

    if !dry {
        log::info!("Writing to: {}", out_dir.display());
        fs::create_dir_all(&out_dir).unwrap();
    }

    series::download_series(&client, &url, &html_issue_class, &html_image_class, &out_dir, dry).await;
}
