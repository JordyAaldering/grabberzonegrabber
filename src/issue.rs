use std::{fs::File, io::{Cursor, Write}, path::{Path, PathBuf}};

use reqwest::Client;
use scraper::{CaseSensitivity, Html, Selector};
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

async fn get_html(client: &Client, url: &str) -> reqwest::Result<String> {
    let resp = client.get(url).send().await?;
    resp.text().await
}

fn extract_image_urls(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("img").unwrap();

    // Images are assumed to appear in order
    let mut imgs = Vec::new();

    for el in document.select(&selector) {
        let el = el.value();
        if el.has_class("wp-manga-chapter-img", CaseSensitivity::AsciiCaseInsensitive) {
            if let Some(src) = el.attr("data-src").or(el.attr("src")) {
                log::trace!("Page {}: {}", imgs.len() + 1, src.trim());
                imgs.push(src.trim().to_owned());
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
    img.write_to(&mut Cursor::new(&mut img_bytes), image::ImageFormat::WebP).unwrap();
    Ok(img_bytes)
}

pub async fn download_issue(client: &Client, url: &str, issue_name: &str, out_dir: &Path, dry: bool) -> zip::result::ZipResult<()> {
    log::info!("Fetching issue {} from {}", issue_name, url);
    let text = get_html(&client, &url).await.unwrap();
    let imgs = extract_image_urls(&text);
    log::info!("Found {} images for {}", imgs.len(), url);

    if imgs.is_empty() {
        log::warn!("No images found for {}", url);
        return Ok(());
    }

    if dry {
        return Ok(());
    }

    let cbz_dst = out_dir.join(format!("{}.cbz", issue_name));
    if cbz_dst.exists() {
        log::warn!("File {} already exists, skipping", cbz_dst.display());
        return Ok(());
    }

    log::info!("Merging images of {}", issue_name);
    // Write to buffer first, only write to disk if successful to avoid partial files
    let buffer = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(buffer);
    // WebP images are already compressed, so create the cbz without compression
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored);

    for (page, img) in imgs.into_iter().enumerate() {
        log::trace!("Downloading {}", img);
        let img_data = download_image(&client, &img).await.unwrap();

        let name = format!("{:04}.webp", page);
        log::trace!("Writing {} to {}", name, cbz_dst.display());
        zip.start_file(name, options)?;
        zip.write_all(&img_data)?;
    }

    log::info!("Writing to cbz {}", cbz_dst.display());
    let buffer = zip.finish()?;
    let mut file = File::create(&cbz_dst)?;
    file.write_all(&buffer.into_inner())?;

    Ok(())
}
