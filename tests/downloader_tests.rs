use ap_dl::downloader::{format_bytes, DownloadTask};
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[test]
fn test_filename_sanitizer_edge_cases() {
    assert_eq!(
        DownloadTask::sanitize_filename("آموزش: درس اول / ویدیو شماره *1*?"),
        "آموزش_ درس اول _ ویدیو شماره _1"
    );
    assert_eq!(
        DownloadTask::sanitize_filename("  ...file_with_dots...  "),
        "file_with_dots"
    );
    assert_eq!(
        DownloadTask::sanitize_filename(":::***???///"),
        "aparat_video"
    );
    assert_eq!(
        DownloadTask::sanitize_filename(""),
        "aparat_video"
    );
    assert_eq!(
        DownloadTask::sanitize_filename("video\n\t\0title"),
        "video___title"
    );
}

#[test]
fn test_format_bytes_boundaries() {
    assert_eq!(format_bytes(0), "0 B");
    assert_eq!(format_bytes(500), "500 B");
    assert_eq!(format_bytes(1023), "1023 B");
    assert_eq!(format_bytes(1024), "1 KB");
    assert_eq!(format_bytes(1024 * 500), "500 KB");
    assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
    assert_eq!(format_bytes(1024 * 1024 * 5), "5.0 MB");
    assert_eq!(format_bytes(1024 * 1024 * 1024 * 2), "2.00 GB");
    assert_eq!(format_bytes((1024.0 * 1024.0 * 1024.0 * 3.5) as u64), "3.50 GB");
}

#[test]
fn test_get_candidate_urls_priority() {
    let task = DownloadTask {
        url: "https://caspian10.cdn.asset.aparat.com/aparat-video/vid.mp4?wmsAuthSign=abc".to_string(),
        candidate_urls: vec![
            "https://arvan1.cdn.asset.aparat.com/aparat-video/vid.mp4?wmsAuthSign=abc".to_string(),
            "https://persian8.cdn.asset.aparat.com/aparat-video/vid.mp4?wmsAuthSign=abc".to_string(),
        ],
        title: "Test Video".to_string(),
        destination_folder: PathBuf::from("/tmp"),
        cancel_flag: Arc::new(AtomicBool::new(false)),
    };

    let candidates = task.get_candidate_urls();
    assert!(!candidates.is_empty());
    assert!(candidates[0].contains("persian8"));
    assert!(candidates.last().unwrap().contains("caspian") || candidates.iter().position(|u| u.contains("caspian")).unwrap() > 0);
}

#[tokio::test]
async fn test_download_task_invalid_url() {
    let cancel = Arc::new(AtomicBool::new(false));
    let task = DownloadTask {
        url: "".to_string(),
        candidate_urls: Vec::new(),
        title: "Nonexistent".to_string(),
        destination_folder: std::env::temp_dir(),
        cancel_flag: cancel,
    };

    let result = task.run(|_| {}).await;
    assert!(result.is_err());
}


#[tokio::test]
async fn test_download_task_cancellation() {
    let cancel = Arc::new(AtomicBool::new(true)); // Pre-cancelled
    let task = DownloadTask {
        url: "https://caspian10.cdn.asset.aparat.com/aparat-video/vid.mp4".to_string(),
        candidate_urls: Vec::new(),
        title: "Cancelled Video".to_string(),
        destination_folder: std::env::temp_dir(),
        cancel_flag: cancel,
    };

    let result = task.run(|_| {}).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_assembler_combines_parts_correctly() {
    use ap_dl::downloader::assembler::assemble_parts;
    let temp_dir = std::env::temp_dir();
    let part0 = temp_dir.join("test_part0.part0");
    let part1 = temp_dir.join("test_part1.part1");
    let final_file = temp_dir.join("test_final.mp4");

    tokio::fs::write(&part0, b"Hello, ").await.unwrap();
    tokio::fs::write(&part1, b"World!").await.unwrap();

    let res = assemble_parts(&[part0.clone(), part1.clone()], &final_file).await;
    assert!(res.is_ok());

    let final_data = tokio::fs::read(&final_file).await.unwrap();
    assert_eq!(final_data, b"Hello, World!");

    assert!(!part0.exists());
    assert!(!part1.exists());
    let _ = tokio::fs::remove_file(&final_file).await;
}

