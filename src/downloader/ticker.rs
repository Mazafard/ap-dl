use super::progress::{format_bytes, DownloadProgress};
use super::state::{ChunkProgress, TaskCheckpoint};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub fn spawn_progress_ticker<F>(
    cancel_flag: Arc<AtomicBool>,
    overall_downloaded: Arc<AtomicU64>,
    checkpoint_chunks: Arc<Mutex<Vec<ChunkProgress>>>,
    checkpoint_path: &Path,
    first_candidate: String,
    total_bytes: u64,
    mut progress_callback: F,
) -> JoinHandle<()>
where
    F: FnMut(DownloadProgress) + Send + 'static,
{
    let cp_path: PathBuf = checkpoint_path.to_path_buf();
    tokio::spawn(async move {
        let mut last_instant = Instant::now();
        let mut last_bytes = overall_downloaded.load(Ordering::Relaxed);

        while !cancel_flag.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(500)).await;
            let current = overall_downloaded.load(Ordering::Relaxed);
            let elapsed = last_instant.elapsed().as_secs_f64();
            let speed = (current.saturating_sub(last_bytes)) as f64 / elapsed.max(0.1);
            let sp_fmt = format!("{}/s", format_bytes(speed as u64));
            last_instant = Instant::now();
            last_bytes = current;

            let ratio = current as f32 / total_bytes as f32;
            progress_callback(DownloadProgress {
                downloaded_bytes: current,
                total_bytes,
                progress_ratio: ratio.min(1.0),
                speed_formatted: sp_fmt,
                size_formatted: format!("{}/{}", format_bytes(current), format_bytes(total_bytes)),
            });

            let chunks_snap = checkpoint_chunks.lock().await.clone();
            let _ = (TaskCheckpoint { url: first_candidate.clone(), total_size: total_bytes, chunks: chunks_snap }).save(&cp_path).await;
        }
    })
}
