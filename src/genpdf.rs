use crate::utils::ls;

use tokio::task::JoinHandle;
use tokio::fs;

use pdftool::img2pdf::Pdf;


pub async fn genpdf() -> anyhow::Result<()> {
    let mut pdf = Pdf::new();

    let files = ls().await?;

    let handles: Vec<JoinHandle<anyhow::Result<Vec<u8>>>> = files.into_iter().map(|file| {
        tokio::spawn(async move {
            fs::read(file).await.map_err(|e| e.into())
        })
    }).collect();

    let imgs = futures::future::try_join_all(handles)
        .await?
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;

    for img in imgs {
        pdf.add_image(&img)?;
    }

    pdf.pdf.save("out.pdf")?;

    Ok(())
}

