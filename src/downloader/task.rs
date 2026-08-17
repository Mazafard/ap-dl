use super::client::{build_http_client, probe_range_support};
use super::multi_chunk::run_multi_chunk;
use super::progress::DownloadProgress;
use super::single_stream::run_single_stream;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

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
        let trimmed = cleaned.trim().trim_matches('.').trim_matches('_').trim();
        if trimmed.is_empty() {
            "aparat_video".to_string()
        } else {
            trimmed.to_string()
        }
    }

    pub fn get_candidate_urls(&self) -> Vec<String> {
        let mut candidates = Vec::new();
        if !self.url.is_empty() { candidates.push(self.url.clone()); }
        for u in &self.candidate_urls {
            if !candidates.contains(u) && !u.is_empty() { candidates.push(u.clone()); }
        }
        if let Some(first) = candidates.first().cloned() {
            for mirror in crate::aparat::generate_cdn_mirrors(&first) {
                if !candidates.contains(&mirror) { candidates.push(mirror); }
            }
        }
        candidates.sort_by_key(|u| {
            if u.contains("persian8") || u.contains("persian9") || u.contains("persian14") { 0 }
            else if u.contains("persian") { 1 }
            else if u.contains(".as") { 2 }
            else if u.contains("arvan") { 3 }
            else if u.contains(".m") { 4 }
            else if u.contains("caspian") { 10 }
            else { 5 }
        });
        candidates
    }

    pub async fn run<F>(&self, progress_callback: F) -> Result<(), String>
    where
        F: FnMut(DownloadProgress) + Send + 'static,
    {
        if self.cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
            return Err("Download paused by user".to_string());
        }
        let safe_title = Self::sanitize_filename(&self.title);
        let final_path = self.destination_folder.join(format!("{}.mp4", safe_title));
        let checkpoint_path = self.destination_folder.join(format!("{}.apdl", safe_title));
        let client = build_http_client().map_err(|e| format!("HTTP Client Init Error: {}", e))?;
        let candidates = self.get_candidate_urls();

        if candidates.is_empty() {
            return Err("No valid stream URLs provided".to_string());
        }

        if let Some((total_bytes, supports_range)) = probe_range_support(&client, &candidates).await {
            if supports_range && total_bytes > 0 {
                return run_multi_chunk(&client, &candidates, total_bytes, &final_path, &checkpoint_path, self.cancel_flag.clone(), progress_callback).await;
            }
        }

        log::warn!("All CDN range probes failed, falling back to direct stream attempt");
        run_single_stream(&client, &candidates, &final_path, self.cancel_flag.clone(), progress_callback).await
    }
}
