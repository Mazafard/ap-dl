use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

pub async fn download_archive<F>(url: &str, dest_dir: &Path, on_progress: F) -> Result<PathBuf, String>
where
    F: Fn(f32, u64, u64) + Send + 'static,
{
    let client = reqwest::Client::builder()
        .user_agent("APDL-Updater/0.2")
        .build()
        .map_err(|e| e.to_string())?;

    let res = client.get(url).send().await.map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("Download failed with status: {}", res.status()));
    }

    let total_size = res.content_length().unwrap_or(0);
    let filename = url.split('/').last().unwrap_or("update_payload.bin");
    let file_path = dest_dir.join(filename);

    let mut file = tokio::fs::File::create(&file_path).await.map_err(|e| e.to_string())?;
    let mut stream = res.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(|e| e.to_string())?;
        file.write_all(&bytes).await.map_err(|e| e.to_string())?;
        downloaded += bytes.len() as u64;

        let pct = if total_size > 0 {
            downloaded as f32 / total_size as f32
        } else {
            0.5
        };
        on_progress(pct, downloaded, total_size);
    }

    file.flush().await.map_err(|e| e.to_string())?;
    Ok(file_path)
}
