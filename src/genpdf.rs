use pdftool::img2pdf::Pdf;

use tokio::fs;

use std::path::Path;

async fn ls<P: AsRef<Path>>(base: P) -> anyhow::Result<()> {
    let mut entries = fs::read_dir(base).await?;

    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        files.push(entry.file_name());
    }

    Ok(())
}

pub async fn genpdf() -> anyhow::Result<()> {
    let pdf = Pdf::new();

    pdf.add_image();

    Ok(())
}
