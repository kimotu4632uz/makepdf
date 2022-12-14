use pdftool::img2pdf::Pdf;

use crate::utils::read_imgs;

pub async fn genpdf() -> anyhow::Result<()> {
    let mut pdf = Pdf::new();

    let imgs = read_imgs("").await?;

    for img in imgs {
        pdf.add_image(&img)?;
    }

    pdf.pdf.save("out.pdf")?;

    Ok(())
}

