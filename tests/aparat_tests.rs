use ap_dl::aparat::{format_quality_label, format_seconds, generate_cdn_mirrors, quality_rank, AparatClient, FetchResult};

#[test]
fn test_url_parser_variants() {
    assert_eq!(
        AparatClient::parse_input_url("https://www.aparat.com/playlist/26824651"),
        Some((true, "26824651".to_string()))
    );
    assert_eq!(
        AparatClient::parse_input_url("https://www.aparat.com/v/sample?playlist=26824651"),
        Some((true, "26824651".to_string()))
    );
    assert_eq!(
        AparatClient::parse_input_url("https://aparat.com/v/hbq2608"),
        Some((false, "hbq2608".to_string()))
    );
    assert_eq!(
        AparatClient::parse_input_url("https://www.aparat.com/v/hbq2608/sample_video_title"),
        Some((false, "hbq2608".to_string()))
    );
    assert_eq!(
        AparatClient::parse_input_url("hbq2608"),
        Some((false, "hbq2608".to_string()))
    );
    assert_eq!(AparatClient::parse_input_url("https://youtube.com/watch?v=1234"), None);
    assert_eq!(AparatClient::parse_input_url(""), None);
}

#[test]
fn test_format_seconds() {
    assert_eq!(format_seconds(0), "00:00");
    assert_eq!(format_seconds(45), "00:45");
    assert_eq!(format_seconds(125), "02:05");
    assert_eq!(format_seconds(3600), "01:00:00");
    assert_eq!(format_seconds(3665), "01:01:05");
}

#[test]
fn test_format_quality_label() {
    assert_eq!(format_quality_label("1080p"), "1080p (Full HD)");
    assert_eq!(format_quality_label("720p"), "720p (HD)");
    assert_eq!(format_quality_label("480p"), "480p (SD)");
    assert_eq!(format_quality_label("360p"), "360p");
    assert_eq!(format_quality_label("240p"), "240p");
    assert_eq!(format_quality_label("144p"), "144p");
    assert_eq!(format_quality_label("custom_profile"), "custom_profile");
}

#[test]
fn test_quality_rank() {
    assert!(quality_rank("1080p (Full HD)") > quality_rank("720p (HD)"));
    assert!(quality_rank("720p (HD)") > quality_rank("480p (SD)"));
    assert!(quality_rank("480p (SD)") > quality_rank("360p"));
    assert!(quality_rank("360p") > quality_rank("240p"));
    assert!(quality_rank("240p") > quality_rank("144p"));
    assert_eq!(quality_rank("Unknown"), 0);
}

#[test]
fn test_generate_cdn_mirrors() {
    let original_url = "https://caspian10.cdn.asset.aparat.com/aparat-video/sample.mp4?wmsAuthSign=token123";
    let mirrors = generate_cdn_mirrors(original_url);
    assert!(!mirrors.is_empty());
    assert!(mirrors.iter().any(|m| m.contains("persian8.cdn.asset.aparat.com")));
    assert!(mirrors.iter().any(|m| m.contains("persian9.cdn.asset.aparat.com")));
    assert!(mirrors.iter().all(|m| m.contains("sample.mp4?wmsAuthSign=token123")));
}

#[tokio::test]
async fn test_fetch_playlist() {
    let client = AparatClient::new();
    let playlist = client.fetch_playlist_info("26824651").await.unwrap();
    assert!(!playlist.items.is_empty(), "Playlist items should not be empty");
    assert_eq!(playlist.id, "26824651");
}

#[tokio::test]
async fn test_resolve_url_playlist() {
    let client = AparatClient::new();
    let res = client.resolve_url("https://www.aparat.com/playlist/26824651").await.unwrap();
    match res {
        FetchResult::Playlist(p) => {
            assert!(!p.items.is_empty());
            let (items, tasks) = ap_dl::ui_adapter::item_builder::build_playlist_items(&p);
            assert_eq!(items.len(), p.items.len());
            assert_eq!(tasks.len(), p.items.len());
        }
        FetchResult::SingleVideo(_) => panic!("Expected playlist result"),
    }
}

