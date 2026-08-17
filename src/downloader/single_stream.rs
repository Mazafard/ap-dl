use super::progress::{format_bytes, DownloadProgress};
use futures_util::StreamExt;
use reqwest::Client;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn run_single_stream<F>(
    client: &Client,
    candidates: &[String],
    final_path: &Path,
    cancel_flag: Arc<AtomicBool>,
    mut progress_callback: F,
) -> Result<(), String>
where
    F: FnMut(DownloadProgress) + Send + 'static,
{
    for mirror_url in candidates {
        match execute_single_stream(client, mirror_url, final_path, cancel_flag.clone(), &mut progress_callback).await {
            Ok(_) => return Ok(()),
            Err(e) if e.contains("paused") => return Err(e),
            Err(e) => {
                log::warn!("Single stream on {} failed ({}), trying next mirror...", mirror_url, e);
            }
        }
    }
    Err("All mirror links failed during single stream download".to_string())
}

async fn execute_single_stream<F>(
    client: &Client,
    url: &str,
    final_path: &Path,
    cancel_flag: Arc<AtomicBool>,
    progress_callback: &mut F,
) -> Result<(), String>
where
    F: FnMut(DownloadProgress),
{
    let resp = client.get(url).send().await.map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("HTTP error status: {}", resp.status()));
    }

    let total_bytes = resp.content_length().unwrap_or(0);
    let mut file = File::create(final_path).await.map_err(|e| format!("File create error: {}", e))?;
    let mut stream = resp.bytes_stream();

    let mut downloaded: u64 = 0;
    let mut last_instant = Instant::now();
    let mut last_bytes: u64 = 0;

    while let Some(chunk_res) = stream.next().await {
        if cancel_flag.load(Ordering::SeqCst) {
            return Err("Download paused by user".to_string());
        }

        let data = chunk_res.map_err(|e| format!("Stream error: {}", e))?;
        let len = data.len() as u64;

        file.write_all(&data).await.map_err(|e| format!("Write error: {}", e))?;
        downloaded += len;

        let elapsed = last_instant.elapsed().as_secs_f64();
        if elapsed >= 0.5 {
            let bytes_diff = downloaded.saturating_sub(last_bytes);
            let speed = bytes_diff as f64 / elapsed;
            let speed_formatted = format!("{}/s", format_bytes(speed as u64));
            last_instant = Instant::now();
            last_bytes = downloaded;

            let ratio = if total_bytes > 0 { downloaded as f32 / total_bytes as f32 } else { 0.0 };
            progress_callback(DownloadProgress {
                downloaded_bytes: downloaded,
                total_bytes,
                progress_ratio: ratio,
                speed_formatted: speed_formatted.clone(),
                size_formatted: format!("{}/{}", format_bytes(downloaded), if total_bytes > 0 { format_bytes(total_bytes) } else { "Unknown".to_string() }),
            });
        }
    }

    file.flush().await.map_err(|e| format!("Flush error: {}", e))?;
    Ok(())
}
