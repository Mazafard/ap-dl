use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

#[derive(Clone)]
pub struct InternalTask {
    pub id: String,
    pub video_hash: String,
    pub title: String,
    pub direct_url: Option<String>,
    pub candidate_urls: Vec<String>,
}

pub struct AppState {
    pub download_dir: RwLock<PathBuf>,
    pub media_items: Arc<Mutex<Vec<crate::MediaItem>>>,
    pub tasks: Arc<Mutex<Vec<InternalTask>>>,
    pub cancel_flag: Arc<AtomicBool>,
}

impl AppState {
    pub fn new(download_dir: PathBuf) -> Arc<Self> {
        Arc::new(Self {
            download_dir: RwLock::new(download_dir),
            media_items: Arc::new(Mutex::new(Vec::new())),
            tasks: Arc::new(Mutex::new(Vec::new())),
            cancel_flag: Arc::new(AtomicBool::new(false)),
        })
    }
}
