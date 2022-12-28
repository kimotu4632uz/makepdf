use std::env;

use tokio::fs;

use crate::utils::ls;

pub async fn backup() -> anyhow::Result<()> {
    let cwd = env::current_dir()?;

    let backup_dir = cwd.join("backup");
    if !backup_dir.exists() {
        fs::create_dir(backup_dir).await?;
    }

    let files = ls().await?;

    for file in files {
        let out_file = file
            .parent()
            .unwrap()
            .join("backup")
            .join(file.file_name().unwrap());
        fs::copy(file, out_file).await?;
    }

    Ok(())
}
