use ap_dl::downloader::{ChunkProgress, TaskCheckpoint};

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

    // Non-existent load returns None
    assert!(TaskCheckpoint::load(&test_path).await.is_none());

    // Removing non-existent file should not panic
    TaskCheckpoint::remove(&test_path).await;
}
