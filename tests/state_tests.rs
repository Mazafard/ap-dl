use ap_dl::state::{AppState, InternalTask};
use std::path::PathBuf;
use std::sync::atomic::Ordering;

#[tokio::test]
async fn test_app_state_operations() {
    let state = AppState::new(PathBuf::from("/downloads"));
    assert_eq!(*state.download_dir.read().await, PathBuf::from("/downloads"));

    // Update download dir
    {
        let mut write = state.download_dir.write().await;
        *write = PathBuf::from("/custom/path");
    }
    assert_eq!(*state.download_dir.read().await, PathBuf::from("/custom/path"));

    // Add task
    {
        let mut tasks = state.tasks.lock().await;
        tasks.push(InternalTask {
            id: "task_1".to_string(),
            video_hash: "v123".to_string(),
            title: "Test Title".to_string(),
            direct_url: Some("https://example.com/v.mp4".to_string()),
            candidate_urls: vec!["https://example.com/v.mp4".to_string()],
        });
        assert_eq!(tasks.len(), 1);
    }

    // Toggle cancel flag
    assert!(!state.cancel_flag.load(Ordering::SeqCst));
    state.cancel_flag.store(true, Ordering::SeqCst);
    assert!(state.cancel_flag.load(Ordering::SeqCst));
}
