use crate::utils::ls;

use std::env;
use tokio::fs;

pub async fn backup() -> anyhow::Result<()> {
    let cwd = env::current_dir()?;
    fs::create_dir(cwd.join("backup")).await?;

    let files = ls().await?;

    for file in files {
        let out_file = file.parent().unwrap().join("backup").join(file.file_name().unwrap());
        fs::copy(file, out_file).await?;
    }

    Ok(())
}

