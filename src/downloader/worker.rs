use crate::downloader::state::{ChunkProgress, TaskCheckpoint};
use futures_util::StreamExt;
use reqwest::header::{ACCEPT_RANGES, CONTENT_LENGTH, RANGE, REFERER, USER_AGENT};
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;

const DEFAULT_CHUNKS: usize = 2;
const LARGE_FILE_CHUNKS: usize = 4;
const CHUNK_SPLIT_THRESHOLD: u64 = 64 * 1024 * 1024; // 64 MB

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub progress_ratio: f32,
    pub speed_formatted: String,
    pub size_formatted: String,
}

pub struct DownloadTask {
    pub url: String,
    pub candidate_urls: Vec<String>,
    pub title: String,
    pub destination_folder: PathBuf,
    pub cancel_flag: Arc<AtomicBool>,
}

impl DownloadTask {
    pub fn sanitize_filename(name: &str) -> String {
        let cleaned: String = name
            .chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                c if c.is_control() => '_',
                c => c,
            })
            .collect();
        let trimmed = cleaned.trim().trim_matches('.');
        if trimmed.is_empty() {
            "aparat_video".to_string()
        } else {
            trimmed.to_string()
        }
    }

    pub fn get_candidate_urls(&self) -> Vec<String> {
        let mut candidates = Vec::new();
        if !self.url.is_empty() {
            candidates.push(self.url.clone());
        }
        for u in &self.candidate_urls {
            if !candidates.contains(u) && !u.is_empty() {
                candidates.push(u.clone());
            }
        }
        // If needed, generate dynamic mirror fallbacks
        if let Some(first) = candidates.first().cloned() {
            for mirror in crate::aparat::api::generate_cdn_mirrors(&first) {
                if !candidates.contains(&mirror) {
                    candidates.push(mirror);
                }
            }
        }

        // Prioritize fast, reliable Persian & Arvan clusters over Caspian nodes
        candidates.sort_by_key(|u| {
            if u.contains("persian8") || u.contains("persian9") || u.contains("persian14") {
                0
            } else if u.contains("persian") {
                1
            } else if u.contains(".as") {
                2
            } else if u.contains("arvan") {
                3
            } else if u.contains(".m") {
                4
            } else if u.contains("caspian") {
                10 // Demote caspian to the end to avoid initial connection hangs
            } else {
                5
            }
        });

        candidates
    }

    fn build_http_client() -> Result<reqwest::Client, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
                .parse()
                .unwrap(),
        );
        headers.insert(REFERER, "https://www.aparat.com/".parse().unwrap());
        headers.insert("Origin", "https://www.aparat.com".parse().unwrap());
        headers.insert("Accept", "*/*".parse().unwrap());

        reqwest::Client::builder()
            .default_headers(headers)
            .tcp_nodelay(true)
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .pool_max_idle_per_host(4)
            .pool_idle_timeout(Some(Duration::from_secs(60)))
            .connect_timeout(Duration::from_secs(5))
            .build()
    }

    pub async fn run<F>(&self, on_progress: F) -> Result<(), String>
    where
        F: Fn(DownloadProgress) + Send + Sync + 'static,
    {
        let client = Self::build_http_client().map_err(|e| format!("Client build error: {}", e))?;
        let sanitized = Self::sanitize_filename(&self.title);
        let final_path = self.destination_folder.join(format!("{}.mp4", sanitized));
        let part_path = self.destination_folder.join(format!("{}.mp4.part", sanitized));
        let apdl_path = self.destination_folder.join(format!("{}.mp4.apdl", sanitized));

        // 1. Verify if final file already exists and is non-empty
        if final_path.exists() {
            if let Ok(metadata) = tokio::fs::metadata(&final_path).await {
                if metadata.len() > 0 {
                    let size_str = format_bytes(metadata.len());
                    on_progress(DownloadProgress {
                        progress_ratio: 1.0,
                        speed_formatted: "Completed ✓".to_string(),
                        size_formatted: size_str,
                    });
                    return Ok(());
                }
            }
        }

        tokio::fs::create_dir_all(&self.destination_folder)
            .await
            .map_err(|e| format!("Failed to create destination dir: {}", e))?;

        // 2. Check for existing checkpoint
        let mut checkpoint = TaskCheckpoint::load(&apdl_path).await;
        let mut total_size = checkpoint.as_ref().map(|c| c.total_size).unwrap_or(0);
        let candidate_urls = self.get_candidate_urls();

        // 3. If no checkpoint, probe server for Content-Length and Accept-Ranges across candidate mirrors
        let mut supports_ranges = true;
        let mut active_working_url = self.url.clone();

        if checkpoint.is_none() {
            let mut probe_succeeded = false;

            for (idx, candidate_url) in candidate_urls.iter().enumerate() {
                if self.cancel_flag.load(Ordering::SeqCst) {
                    return Err("Download paused".to_string());
                }

                let probe_res = client
                    .get(candidate_url)
                    .header(RANGE, "bytes=0-0")
                    .send()
                    .await;

                match probe_res {
                    Ok(probe) if probe.status().is_success() || probe.status().as_u16() == 206 => {
                        if let Some(cr) = probe.headers().get("content-range") {
                            if let Ok(cr_str) = cr.to_str() {
                                if let Some(pos) = cr_str.rfind('/') {
                                    if let Ok(len) = cr_str[pos + 1..].trim().parse::<u64>() {
                                        total_size = len;
                                    }
                                }
                            }
                        }

                        if total_size == 0 {
                            if let Some(cl) = probe.headers().get(CONTENT_LENGTH) {
                                if let Ok(cl_str) = cl.to_str() {
                                    if let Ok(len) = cl_str.trim().parse::<u64>() {
                                        total_size = len;
                                    }
                                }
                            }
                        }

                        if probe.status().as_u16() != 206 {
                            let has_accept_ranges = probe
                                .headers()
                                .get(ACCEPT_RANGES)
                                .and_then(|v| v.to_str().ok())
                                .map(|v| v.contains("bytes"))
                                .unwrap_or(false);
                            if !has_accept_ranges {
                                supports_ranges = false;
                            }
                        }

                        active_working_url = candidate_url.clone();
                        probe_succeeded = true;
                        if idx > 0 {
                            log::info!("Successfully failed over to working CDN mirror: {}", candidate_url);
                        }
                        break;
                    }
                    Ok(probe) => {
                        log::warn!("Range probe to CDN mirror {} returned HTTP {}, attempting next mirror...", candidate_url, probe.status());
                    }
                    Err(e) => {
                        log::warn!("Range probe to CDN mirror {} failed ({}), attempting next mirror...", candidate_url, e);
                    }
                }
            }

            if !probe_succeeded {
                log::warn!("All CDN range probes failed, falling back to direct stream attempt");
                supports_ranges = false;
            }

            // Create initial checkpoint if range supported and total size known
            if supports_ranges && total_size > 0 {
                let num_chunks = if total_size >= CHUNK_SPLIT_THRESHOLD {
                    LARGE_FILE_CHUNKS
                } else {
                    DEFAULT_CHUNKS
                };

                let chunk_size = total_size / (num_chunks as u64);
                let mut chunks = Vec::new();

                for i in 0..num_chunks {
                    let start = i as u64 * chunk_size;
                    let end = if i == num_chunks - 1 {
                        total_size - 1
                    } else {
                        (i as u64 + 1) * chunk_size - 1
                    };

                    chunks.push(ChunkProgress {
                        index: i,
                        start_byte: start,
                        end_byte: end,
                        downloaded_bytes: 0,
                        is_complete: false,
                    });
                }

                // Pre-allocate part file on disk
                if let Ok(file) = tokio::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(&part_path)
                    .await
                {
                    let _ = file.set_len(total_size).await;
                }

                let cp = TaskCheckpoint {
                    url: active_working_url.clone(),
                    total_size,
                    chunks,
                };
                let _ = cp.save(&apdl_path).await;
                checkpoint = Some(cp);
            }
        }

        // 4. Multi-part chunked download if checkpoint exists
        if let Some(cp) = checkpoint {
            let res = self
                .download_multipart(
                    client.clone(),
                    cp,
                    &candidate_urls,
                    &part_path,
                    &apdl_path,
                    &final_path,
                    &on_progress,
                )
                .await;

            match res {
                Ok(_) => Ok(()),
                Err(e) if e.contains("paused") => Err(e),
                Err(e) => {
                    log::warn!("Multi-part stream failed ({}), falling back to single stream across candidate mirrors", e);
                    self.download_single_stream(
                        client,
                        &candidate_urls,
                        total_size,
                        &part_path,
                        &final_path,
                        on_progress,
                    )
                    .await
                }
            }
        } else {
            // Fallback single stream download
            self.download_single_stream(
                client,
                &candidate_urls,
                total_size,
                &part_path,
                &final_path,
                on_progress,
            )
            .await
        }
    }

    async fn download_multipart<F>(
        &self,
        client: reqwest::Client,
        checkpoint: TaskCheckpoint,
        candidate_urls: &[String],
        part_path: &Path,
        apdl_path: &Path,
        final_path: &Path,
        on_progress: &F,
    ) -> Result<(), String>
    where
        F: Fn(DownloadProgress) + Send + Sync + 'static,
    {
        let total_size = checkpoint.total_size;
        let mut initial_downloaded = 0u64;
        for c in &checkpoint.chunks {
            initial_downloaded += c.downloaded_bytes;
        }

        let downloaded_counter = Arc::new(AtomicU64::new(initial_downloaded));
        let checkpoint_state = Arc::new(Mutex::new(checkpoint));

        let chunks = {
            let cp = checkpoint_state.lock().await;
            cp.chunks.clone()
        };

        let mut workers = Vec::new();
        let all_urls = Arc::new(candidate_urls.to_vec());

        for (idx, chunk) in chunks.into_iter().enumerate() {
            if chunk.is_complete || chunk.downloaded_bytes >= (chunk.end_byte - chunk.start_byte + 1) {
                continue;
            }

            if idx > 0 {
                tokio::time::sleep(Duration::from_millis(150)).await;
            }

            let client_c = client.clone();
            let urls_c = all_urls.clone();
            let part_path_c = part_path.to_path_buf();
            let cancel_c = self.cancel_flag.clone();
            let downloaded_c = downloaded_counter.clone();
            let checkpoint_c = checkpoint_state.clone();

            let worker = tokio::spawn(async move {
                let mut current_downloaded = chunk.downloaded_bytes;
                let mut start_pos = chunk.start_byte + current_downloaded;
                let end_pos = chunk.end_byte;

                if start_pos > end_pos {
                    return Ok(());
                }

                let mut last_error = String::new();
                let mut chunk_succeeded = false;

                // Try downloading chunk with retries and failover across candidate CDN mirrors
                'mirror_loop: for url_candidate in urls_c.iter() {
                    for attempt in 1..=3 {
                        if cancel_c.load(Ordering::SeqCst) {
                            return Err("Download paused".to_string());
                        }

                        let range_header = format!("bytes={}-{}", start_pos, end_pos);
                        let req_res = client_c
                            .get(url_candidate)
                            .header(RANGE, range_header)
                            .send()
                            .await;

                        let response = match req_res {
                            Ok(r) if r.status().is_success() || r.status().as_u16() == 206 => r,
                            Ok(r) => {
                                last_error = format!("HTTP {}", r.status());
                                tokio::time::sleep(Duration::from_millis(300 * attempt)).await;
                                continue;
                            }
                            Err(e) => {
                                last_error = format!("Connection error: {}", e);
                                tokio::time::sleep(Duration::from_millis(300 * attempt)).await;
                                continue;
                            }
                        };

                        let mut file = match tokio::fs::OpenOptions::new()
                            .write(true)
                            .open(&part_path_c)
                            .await
                        {
                            Ok(f) => f,
                            Err(e) => return Err(format!("Failed to open part file: {}", e)),
                        };

                        if let Err(e) = file.seek(SeekFrom::Start(start_pos)).await {
                            return Err(format!("Failed to seek file: {}", e));
                        }

                        let mut stream = response.bytes_stream();
                        let mut stream_error = false;

                        while let Some(item) = stream.next().await {
                            if cancel_c.load(Ordering::SeqCst) {
                                return Err("Download paused".to_string());
                            }

                            match item {
                                Ok(bytes) => {
                                    if let Err(e) = file.write_all(&bytes).await {
                                        return Err(format!("Chunk write error: {}", e));
                                    }
                                    let len = bytes.len() as u64;
                                    current_downloaded += len;
                                    start_pos += len;
                                    downloaded_c.fetch_add(len, Ordering::Relaxed);

                                    // Update chunk progress in checkpoint
                                    {
                                        let mut cp = checkpoint_c.lock().await;
                                        if let Some(c) = cp.chunks.get_mut(chunk.index) {
                                            c.downloaded_bytes = current_downloaded;
                                            if current_downloaded >= (c.end_byte - c.start_byte + 1) {
                                                c.is_complete = true;
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    last_error = format!("Stream read error: {}", e);
                                    stream_error = true;
                                    break;
                                }
                            }
                        }

                        let _ = file.flush().await;

                        if !stream_error && (current_downloaded >= (chunk.end_byte - chunk.start_byte + 1) || start_pos > end_pos) {
                            chunk_succeeded = true;
                            break 'mirror_loop;
                        }
                    }
                }

                if chunk_succeeded {
                    Ok::<(), String>(())
                } else {
                    Err(format!("Chunk {} failed across all mirrors: {}", chunk.index, last_error))
                }
            });

            workers.push(worker);
        }

        // Progress Reporter Loop
        let progress_cancel = self.cancel_flag.clone();
        let progress_downloaded = downloaded_counter.clone();
        let apdl_path_buf = apdl_path.to_path_buf();
        let checkpoint_report = checkpoint_state.clone();

        let progress_loop = tokio::spawn(async move {
            let mut last_bytes = progress_downloaded.load(Ordering::Relaxed);
            let mut last_instant = Instant::now();

            while !progress_cancel.load(Ordering::SeqCst) {
                tokio::time::sleep(Duration::from_millis(300)).await;

                let current_bytes = progress_downloaded.load(Ordering::Relaxed);
                let now = Instant::now();
                let elapsed = now.duration_since(last_instant).as_secs_f64();

                let _speed = if elapsed > 0.0 && current_bytes >= last_bytes {
                    (current_bytes - last_bytes) as f64 / elapsed
                } else {
                    0.0
                };

                last_bytes = current_bytes;
                last_instant = now;

                // Periodically save checkpoint
                if let Ok(cp) = checkpoint_report.try_lock() {
                    let _ = cp.save(&apdl_path_buf).await;
                }

                if current_bytes >= total_size {
                    break;
                }
            }
        });

        // Await all workers
        let mut failed = false;
        let mut error_message = String::new();

        for worker in workers {
            match worker.await {
                Ok(Ok(_)) => {}
                Ok(Err(e)) => {
                    failed = true;
                    error_message = e;
                }
                Err(e) => {
                    failed = true;
                    error_message = format!("Worker join error: {}", e);
                }
            }
        }

        // Cancel progress loop
        let _ = progress_loop.abort();

        // Save final checkpoint state
        {
            let cp = checkpoint_state.lock().await;
            let _ = cp.save(apdl_path).await;
        }

        if self.cancel_flag.load(Ordering::SeqCst) {
            return Err("Download paused".to_string());
        }

        if failed {
            return Err(error_message);
        }

        let final_downloaded = downloaded_counter.load(Ordering::Relaxed);
        if final_downloaded >= total_size && total_size > 0 {
            tokio::fs::rename(part_path, final_path)
                .await
                .map_err(|e| format!("Failed to finalize file: {}", e))?;
            TaskCheckpoint::remove(apdl_path).await;

            on_progress(DownloadProgress {
                progress_ratio: 1.0,
                speed_formatted: "Completed ✓".to_string(),
                size_formatted: format_bytes(total_size),
            });

            return Ok(());
        }

        Err("Incomplete download".to_string())
    }

    async fn download_single_stream<F>(
        &self,
        client: reqwest::Client,
        candidate_urls: &[String],
        total_size: u64,
        part_path: &Path,
        final_path: &Path,
        on_progress: F,
    ) -> Result<(), String>
    where
        F: Fn(DownloadProgress) + Send + Sync + 'static,
    {
        let mut last_error = String::new();

        for candidate_url in candidate_urls.iter() {
            if self.cancel_flag.load(Ordering::SeqCst) {
                return Err("Download paused".to_string());
            }

            let mut existing_bytes = 0u64;
            if let Ok(meta) = tokio::fs::metadata(part_path).await {
                existing_bytes = meta.len();
            }

            let mut req = client.get(candidate_url);
            if existing_bytes > 0 {
                req = req.header(RANGE, format!("bytes={}-", existing_bytes));
            }

            let response = match req.send().await {
                Ok(r) if r.status().is_success() || r.status().as_u16() == 206 => r,
                Ok(r) => {
                    last_error = format!("Server returned HTTP {}", r.status());
                    log::warn!("Single stream on {} failed ({}), trying next mirror...", candidate_url, last_error);
                    continue;
                }
                Err(e) => {
                    last_error = format!("Request failed: {}", e);
                    log::warn!("Single stream on {} failed ({}), trying next mirror...", candidate_url, last_error);
                    continue;
                }
            };

            let mut file = match tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(part_path)
                .await
            {
                Ok(f) => f,
                Err(e) => return Err(format!("Failed to open part file: {}", e)),
            };

            let mut stream = response.bytes_stream();
            let mut downloaded = existing_bytes;
            let mut last_bytes = downloaded;
            let mut last_instant = Instant::now();
            let mut stream_failed = false;

            while let Some(chunk_res) = stream.next().await {
                if self.cancel_flag.load(Ordering::SeqCst) {
                    return Err("Download paused".to_string());
                }

                let chunk = match chunk_res {
                    Ok(c) => c,
                    Err(e) => {
                        last_error = format!("Stream chunk error: {}", e);
                        stream_failed = true;
                        break;
                    }
                };

                if let Err(e) = file.write_all(&chunk).await {
                    return Err(format!("Write error: {}", e));
                }

                downloaded += chunk.len() as u64;

                let now = Instant::now();
                let elapsed = now.duration_since(last_instant).as_secs_f64();

                if elapsed >= 0.25 {
                    let speed = if elapsed > 0.0 {
                        (downloaded - last_bytes) as f64 / elapsed
                    } else {
                        0.0
                    };

                    let ratio = if total_size > 0 {
                        (downloaded as f32 / total_size as f32).min(1.0)
                    } else {
                        0.0
                    };

                    on_progress(DownloadProgress {
                        progress_ratio: ratio,
                        speed_formatted: format!("{}/s", format_bytes(speed as u64)),
                        size_formatted: if total_size > 0 {
                            format!("{} / {}", format_bytes(downloaded), format_bytes(total_size))
                        } else {
                            format_bytes(downloaded)
                        },
                    });

                    last_bytes = downloaded;
                    last_instant = now;
                }
            }

            let _ = file.flush().await;

            if !stream_failed && (total_size == 0 || downloaded >= total_size) {
                tokio::fs::rename(part_path, final_path)
                    .await
                    .map_err(|e| format!("Failed to finalize file: {}", e))?;
                return Ok(());
            }
        }

        Err(if last_error.is_empty() {
            "Incomplete download across all CDN mirrors".to_string()
        } else {
            last_error
        })
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.1} MB", b / MB)
    } else if b >= KB {
        format!("{:.0} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_sanitizer() {
        assert_eq!(
            DownloadTask::sanitize_filename("آموزش: درس اول / ویدیو شماره *1*?"),
            "آموزش_ درس اول _ ویدیو شماره _1__"
        );
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024 * 1024 * 5), "5.0 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 2), "2.00 GB");
    }
}
