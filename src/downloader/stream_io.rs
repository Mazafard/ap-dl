use super::state::ChunkProgress;
use futures_util::StreamExt;
use reqwest::header::RANGE;
use reqwest::Client;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::sync::Mutex;

pub async fn execute_chunk_stream(
    client: &Client,
    url: &str,
    chunk: &ChunkProgress,
    part_path: &Path,
    overall_downloaded: Arc<AtomicU64>,
    cancel_flag: Arc<AtomicBool>,
    checkpoint_chunks: Arc<Mutex<Vec<ChunkProgress>>>,
) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(part_path)
        .await
        .map_err(|e| format!("Disk error creating chunk file: {}", e))?;

    let initial_offset = file.seek(SeekFrom::End(0)).await.unwrap_or(0);
    let start = chunk.start_byte + initial_offset;
    let end = chunk.end_byte;

    if start > end { return Ok(()); }

    let resp = client.get(url).header(RANGE, format!("bytes={}-{}", start, end)).send().await
        .map_err(|e| format!("Network error on chunk request: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Server returned error status on chunk: {}", resp.status()));
    }

    let mut stream = resp.bytes_stream();
    let mut current_downloaded = initial_offset;

    while let Some(chunk_res) = stream.next().await {
        if cancel_flag.load(Ordering::SeqCst) {
            return Err("Download paused by user".to_string());
        }

        let data = chunk_res.map_err(|e| format!("Stream read error: {}", e))?;
        let len = data.len() as u64;

        file.write_all(&data).await.map_err(|e| format!("Disk write error: {}", e))?;
        overall_downloaded.fetch_add(len, Ordering::Relaxed);
        current_downloaded += len;

        let mut lock = checkpoint_chunks.lock().await;
        if let Some(cp) = lock.iter_mut().find(|c| c.index == chunk.index) {
            cp.downloaded_bytes = current_downloaded;
        }
    }

    file.flush().await.map_err(|e| format!("Disk flush error: {}", e))?;
    let mut lock = checkpoint_chunks.lock().await;
    if let Some(cp) = lock.iter_mut().find(|c| c.index == chunk.index) {
        cp.is_complete = true;
    }
    Ok(())
}
