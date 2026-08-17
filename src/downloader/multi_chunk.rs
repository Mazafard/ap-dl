use super::assembler::assemble_parts;
use super::chunk_worker::download_chunk_with_retry;
use super::progress::DownloadProgress;
use super::state::{ChunkProgress, TaskCheckpoint};
use super::ticker::spawn_progress_ticker;
use reqwest::Client;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

pub async fn run_multi_chunk<F>(
    client: &Client,
    candidates: &[String],
    total_bytes: u64,
    final_path: &Path,
    checkpoint_path: &Path,
    cancel_flag: Arc<AtomicBool>,
    progress_callback: F,
) -> Result<(), String>
where
    F: FnMut(DownloadProgress) + Send + 'static,
{
    let chunk_count = if total_bytes > 50 * 1024 * 1024 { 4 } else { 2 };
    let chunk_size = total_bytes / chunk_count as u64;

    let checkpoint = match TaskCheckpoint::load(checkpoint_path).await {
        Some(cp) if cp.total_size == total_bytes => cp,
        _ => {
            let mut chunks = Vec::new();
            for i in 0..chunk_count {
                let start_byte = i as u64 * chunk_size;
                let end_byte = if i == chunk_count - 1 { total_bytes - 1 } else { (i as u64 + 1) * chunk_size - 1 };
                chunks.push(ChunkProgress { index: i, start_byte, end_byte, downloaded_bytes: 0, is_complete: false });
            }
            TaskCheckpoint { url: candidates[0].clone(), total_size: total_bytes, chunks }
        }
    };

    let checkpoint_chunks = Arc::new(Mutex::new(checkpoint.chunks.clone()));
    let initial_downloaded: u64 = checkpoint.chunks.iter().map(|c| c.downloaded_bytes).sum();
    let overall_downloaded = Arc::new(AtomicU64::new(initial_downloaded));

    let mut join_set = JoinSet::new();
    let mut part_paths: Vec<PathBuf> = Vec::new();

    for chunk in checkpoint.chunks {
        let part_path = final_path.with_extension(format!("part{}", chunk.index));
        part_paths.push(part_path.clone());
        if chunk.is_complete { continue; }

        let cl = client.clone();
        let cands = candidates.to_vec();
        let od = overall_downloaded.clone();
        let cf = cancel_flag.clone();
        let cc = checkpoint_chunks.clone();

        join_set.spawn(async move { download_chunk_with_retry(&cl, &cands, chunk, &part_path, od, cf, cc).await });
        tokio::time::sleep(Duration::from_millis(150)).await;
    }

    let ticker_handle = spawn_progress_ticker(
        cancel_flag,
        overall_downloaded,
        checkpoint_chunks,
        checkpoint_path,
        candidates[0].clone(),
        total_bytes,
        progress_callback,
    );

    while let Some(res) = join_set.join_next().await {
        match res {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => { ticker_handle.abort(); return Err(e); }
            Err(e) => { ticker_handle.abort(); return Err(format!("Task worker join error: {}", e)); }
        }
    }
    ticker_handle.abort();

    assemble_parts(&part_paths, final_path).await?;
    TaskCheckpoint::remove(checkpoint_path).await;
    Ok(())
}
