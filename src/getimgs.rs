use std::sync::Arc;

use image::GenericImageView;
use mime_guess::mime;
use scraper::{Html, Selector};

use tokio::{fs, task::JoinHandle};

pub async fn getimgs(url: &str) -> anyhow::Result<()> {
    let html = reqwest::get(url).await?.text().await?;

    let selector = Selector::parse("a,img").unwrap();
    let parsed = Html::parse_document(&html);

    let urls = parsed
        .select(&selector)
        .filter_map(|elem| {
            match elem.value().name() {
                "a" => elem.value().attr("href"),
                "img" => elem.value().attr("src"),
                _ => None,
            }
            .map(String::from)
        })
        .filter(|url| match mime_guess::from_path(url).first() {
            Some(mime) => mime == mime::IMAGE_JPEG || mime == mime::IMAGE_PNG,
            None => false,
        });

    let client = Arc::new(reqwest::Client::new());

    let handles: Vec<JoinHandle<anyhow::Result<_>>> = urls
        .map(|url| {
            let client = client.clone();

            tokio::spawn(async move {
                let fname = url.split("/").last().unwrap().to_string();
                let result = client.get(&url).send().await?.bytes().await?;
                Ok((fname, result))
            })
        })
        .collect();

    let imgs = futures::future::try_join_all(handles)
        .await?
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;

    let handles: Vec<JoinHandle<anyhow::Result<_>>> = imgs
        .into_iter()
        .filter(|(_, img)| {
            if let Some(img) = image::load_from_memory(&img).ok() {
                let (width, _) = img.dimensions();
                return width > 700;
            } else {
                return false;
            }
        })
        .map(|(fname, img)| {
            tokio::spawn(async move { fs::write(fname, img).await.map_err(|e| e.into()) })
        })
        .collect();

    let _ = futures::future::try_join_all(handles)
        .await?
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(())
}
