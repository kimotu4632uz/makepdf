use std::env;

use itertools::Itertools;
use mime_guess::mime;

use tokio::{fs, task::JoinHandle};

use pdftool::Pdf;

use crate::utils::ls;

pub async fn genpdf() -> anyhow::Result<()> {
    let mut pdf = Pdf::new();

    let files = ls()
        .await?
        .into_iter()
        .filter(|file| {
            mime_guess::from_path(file)
                .into_iter()
                .find(|m| *m == mime::IMAGE_JPEG || *m == mime::IMAGE_PNG)
                .is_some()
        })
        .collect_vec();

    let handles: Vec<JoinHandle<anyhow::Result<Vec<u8>>>> = files
        .into_iter()
        .map(|file| tokio::spawn(async move { fs::read(file).await.map_err(|e| e.into()) }))
        .collect();

    let imgs = futures::future::try_join_all(handles)
        .await?
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;

    for img in imgs {
        pdf.add_image(&img)?;
    }

    let bytes = pdf.to_bytes()?;

    if let Some(file) = env::current_dir()?.with_extension("pdf").file_name() {
        fs::write(file, bytes).await?;
    } else {
        fs::write("out.pdf", bytes).await?;
    }

    Ok(())
}
