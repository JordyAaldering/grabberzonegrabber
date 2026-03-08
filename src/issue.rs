use std::{fs::File, io::Write, path::{Path, PathBuf}};

use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use zip::{ZipWriter, write::SimpleFileOptions};

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

pub async fn download_issue(client: &Client, url: &str, re: &Regex, issue_name: &str, out_dir: &Path) -> zip::result::ZipResult<()> {
    println!("Fetching issue {} from {}", issue_name, url);
    let text = get_html(&client, &url).await.unwrap();
    let imgs = extract_image_urls(&text, &re);
    println!("Found {} images", imgs.len());

    let cbz_dst = out_dir.join(format!("{}.cbz", issue_name));
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
