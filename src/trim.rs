use crate::utils::ls;

use tokio::task::JoinHandle;
use tokio::fs;

use image::{GenericImageView, DynamicImage};
use itertools::Itertools;

pub async fn trim() -> anyhow::Result<()> {
    let files = ls("").await?;
    let handles: Vec<JoinHandle<anyhow::Result<()>>> = files.into_iter().map(|file| {
        tokio::spawn(async move {
            let img = fs::read(&file).await?;
            let img = image::load_from_memory(&img)?;

            let color = img.get_pixel(0, 0);
            let (height, width) = img.dimensions();

            let xbottom = (0..width).map(|x| {
                (0..height).find(|y| img.get_pixel(x, *y) != color)
            })
            .collect_vec();

            let x1 = xbottom.iter().find(|x| x.is_some());
            let y1 = xbottom.iter().filter(|x| x.is_some()).min();
            let x2 = xbottom.iter().rev().find(|x| x.is_some());
            let y2 = xbottom.iter().filter(|x| x.is_some()).max();

            let img_out = DynamicImage::ImageRgb8(img.crop_imm(x1, y1, x2 - x1, y2 - y1).to_rgb8()).into_bytes();
            fs::write(file, Vec::new()).await?;
            Ok(())
        })
    }).collect();

    let imgs = futures::future::try_join_all(handles)
        .await?
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(())
}

