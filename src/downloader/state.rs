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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_checkpoint_roundtrip() {
        let temp_dir = std::env::temp_dir();
        let test_path = temp_dir.join("apdl_test_checkpoint.apdl");

        let checkpoint = TaskCheckpoint {
            url: "https://example.com/stream.mp4".to_string(),
            total_size: 104857600,
            chunks: vec![
                ChunkProgress {
                    index: 0,
                    start_byte: 0,
                    end_byte: 52428799,
                    downloaded_bytes: 52428800,
                    is_complete: true,
                },
                ChunkProgress {
                    index: 1,
                    start_byte: 52428800,
                    end_byte: 104857599,
                    downloaded_bytes: 12000000,
                    is_complete: false,
                },
            ],
        };

        checkpoint.save(&test_path).await.unwrap();
        let loaded = TaskCheckpoint::load(&test_path).await.unwrap();

        assert_eq!(loaded.url, "https://example.com/stream.mp4");
        assert_eq!(loaded.total_size, 104857600);
        assert_eq!(loaded.chunks.len(), 2);
        assert!(loaded.chunks[0].is_complete);
        assert!(!loaded.chunks[1].is_complete);

        TaskCheckpoint::remove(&test_path).await;
        assert!(!test_path.exists());
    }
}
