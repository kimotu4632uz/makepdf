use std::env;
use std::path::PathBuf;
use tokio::fs;


pub async fn ls() -> anyhow::Result<Vec<PathBuf>> {
    let cwd = env::current_dir()?;
    let mut entries = fs::read_dir(&cwd).await?;

    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        files.push(cwd.join(entry.file_name()));
    }

    Ok(files)
}

