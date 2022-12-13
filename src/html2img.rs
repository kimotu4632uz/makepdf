use scraper::{Html, Selector};
use image::GenericImageView;
use mime_guess::mime;
use itertools::Itertools;

pub fn get_urls(url: &str) -> anyhow::Result<Vec<String>> {
    let html = ureq::get(url)
        .call()?
        .into_string()?;

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

    Ok(urls)
}

pub fn filter_img(imgs: Vec<&[u8]>) -> anyhow::Result<Vec<&[u8]>> {
    let mut result = Vec::new();

    for img in imgs {
        let (height, _) = image::load_from_memory(img)?.dimensions();
        if height > 700 {
            result.push(img);
        }
    }

    Ok(result)
}

