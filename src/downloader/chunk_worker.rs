use super::state::ChunkProgress;
use super::stream_io::execute_chunk_stream;
use reqwest::Client;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub async fn download_chunk_with_retry(
    client: &Client,
    candidates: &[String],
    chunk: ChunkProgress,
    part_path: &Path,
    overall_downloaded: Arc<AtomicU64>,
    cancel_flag: Arc<AtomicBool>,
    checkpoint_chunks: Arc<Mutex<Vec<ChunkProgress>>>,
) -> Result<(), String> {
    let mut retry_count = 0;
    const MAX_RETRIES: usize = 3;

    loop {
        let candidate_index = retry_count % candidates.len();
        let active_url = &candidates[candidate_index];

        match execute_chunk_stream(
            client,
            active_url,
            &chunk,
            part_path,
            overall_downloaded.clone(),
            cancel_flag.clone(),
            checkpoint_chunks.clone(),
        )
        .await
        {
            Ok(_) => return Ok(()),
            Err(e) if e.contains("paused") => return Err(e),
            Err(e) => {
                retry_count += 1;
                if retry_count >= MAX_RETRIES {
                    return Err(format!("Chunk {} failed after {} retries. Last error: {}", chunk.index, MAX_RETRIES, e));
                }
                let backoff_ms = 400 * (1 << retry_count);
                tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
            }
        }
    }
}
