use std::path::Path;

use tokio::fs;


pub async fn ls<P: AsRef<Path>>(base: P) -> anyhow::Result<Vec<String>> {
    let mut entries = fs::read_dir(base).await?;

    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        files.push(entry.file_name().into_string().unwrap());
    }

    Ok(files)
}

