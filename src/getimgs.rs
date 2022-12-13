use scraper::{Html, Selector};
use image::GenericImageView;
use mime_guess::mime;
use itertools::Itertools;

use std::sync::Arc;
use tokio::task::JoinHandle;

pub async fn getimgs(url: &str) -> anyhow::Result<()> {
    let html = reqwest::get(url)
        .await?
        .text()
        .await?;

    let selector = Selector::parse("a,img").unwrap();

    let urls = Html::parse_document(&html)
        .select(&selector)
        .filter_map(|elem| {
            match elem.value().name() {
                "a" => elem.value().attr("href"),
                "img" => elem.value().attr("src"),
                _ => None,
            }
            .map(String::from)
        })
        .filter(|url| {
            match mime_guess::from_path(url).first() {
                Some(mime) =>
                    mime == mime::IMAGE_JPEG || mime == mime::IMAGE_PNG,
                None => false,
            }
        })
        .collect_vec();

    let client = Arc::new(reqwest::Client::new());

    let handles: Vec<JoinHandle<anyhow::Result<_>>> = urls
        .into_iter()
        .map(|url| {
            let client = client.clone();

            tokio::spawn(async move {
                let result = client.get(url).send().await?.bytes().await?;
                let img = image::load_from_memory(&result)?;
                Ok(img)
            })
        })
        .collect();

    let imgs = futures::future::try_join_all(handles)
        .await?
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;

    let imgs = imgs.into_iter()
        .filter(|img| {
            let (height, _) = img.dimensions();
            height > 700
        })
        .collect_vec();

    Ok(())
}

