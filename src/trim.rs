use image::{DynamicImage, GenericImageView};
use itertools::Itertools;

use tokio::{fs, task::JoinHandle};

use crate::utils::ls;

pub async fn trim() -> anyhow::Result<()> {
    let files = ls().await?;

    let handles: Vec<JoinHandle<anyhow::Result<()>>> = files
        .into_iter()
        .map(|file| {
            tokio::spawn(async move {
                let img = fs::read(&file).await?;
                let img = image::load_from_memory(&img)?;

                let bg = img.get_pixel(0, 0);
                let (height, width) = img.dimensions();

                let yrange = (0..width)
                    .map(|x| {
                        (0..height)
                            .find(|&y| img.get_pixel(x, y) != bg)
                            .and_then(|top| {
                                (0..height)
                                    .rev()
                                    .find(|&y| img.get_pixel(x, y) != bg)
                                    .map(|bottom| (top, bottom))
                            })
                    })
                    .collect_vec();

                let x1 = yrange
                    .iter()
                    .find_position(|x| x.is_some())
                    .map(|(i, _)| i)
                    .unwrap_or(0) as u32;
                let y1 = yrange
                    .iter()
                    .filter_map(|x| x.map(|(top, _)| top))
                    .min()
                    .unwrap_or(0);
                let x2 = width
                    - 1
                    - yrange
                        .iter()
                        .rev()
                        .find_position(|x| x.is_some())
                        .map(|(i, _)| i)
                        .unwrap_or(0) as u32;
                let y2 = yrange
                    .iter()
                    .filter_map(|x| x.map(|(_, bottom)| bottom))
                    .max()
                    .unwrap_or(height - 1);

                let img_out =
                    DynamicImage::ImageRgb8(img.crop_imm(x1, y1, x2 - x1, y2 - y1).to_rgb8())
                        .into_bytes();
                fs::write(file, img_out).await?;
                Ok(())
            })
        })
        .collect();

    let _ = futures::future::try_join_all(handles)
        .await?
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(())
}
