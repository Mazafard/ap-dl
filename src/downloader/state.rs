use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkProgress {
    pub index: usize,
    pub start_byte: u64,
    pub end_byte: u64,
    pub downloaded_bytes: u64,
    pub is_complete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCheckpoint {
    pub url: String,
    pub total_size: u64,
    pub chunks: Vec<ChunkProgress>,
}

impl TaskCheckpoint {
    pub async fn load(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }
        let data = tokio::fs::read(path).await.ok()?;
        serde_json::from_slice(&data).ok()
    }

    pub async fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_vec(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        tokio::fs::write(path, json).await
    }

    pub async fn remove(path: &Path) {
        if path.exists() {
            let _ = tokio::fs::remove_file(path).await;
        }
    }
}
