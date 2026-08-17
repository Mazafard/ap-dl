use super::mirrors::format_seconds;
use super::models::{AparatPlaylistInfo, AparatPlaylistItem};
use reqwest::Client;

pub async fn fetch_playlist_info(http_client: &Client, playlist_id: &str) -> Result<AparatPlaylistInfo, String> {
    let url = format!("https://www.aparat.com/api/fa/v1/video/playlist/one/playlist_id/{}", playlist_id);
    let resp = http_client.get(&url).send().await.map_err(|e| format!("Failed to connect to Aparat Playlist API: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Aparat Playlist API returned error status: {}", resp.status()));
    }

    let json_val: serde_json::Value = resp.json().await.map_err(|e| format!("Failed to parse playlist JSON: {}", e))?;

    let playlist_title = json_val
        .get("data")
        .and_then(|d| d.get("attributes"))
        .and_then(|a| a.get("title"))
        .and_then(|t| t.as_str())
        .unwrap_or("Aparat Playlist")
        .to_string();

    let mut items = Vec::new();
    if let Some(included) = json_val.get("included").and_then(|inc| inc.as_array()) {
        for item in included {
            if item.get("type").and_then(|t| t.as_str()) == Some("Video") {
                let attrs = item.get("attributes");
                let hash = attrs.and_then(|a| a.get("uid")).and_then(|u| u.as_str()).unwrap_or("").to_string();
                let title = attrs.and_then(|a| a.get("title")).and_then(|t| t.as_str()).unwrap_or("Untitled").to_string();
                let sender = attrs.and_then(|a| a.get("sender_name")).and_then(|s| s.as_str()).unwrap_or("Aparat Channel").to_string();

                let duration_str = attrs.and_then(|a| a.get("duration")).and_then(|d| {
                    if let Some(n) = d.as_u64() {
                        Some(format_seconds(n))
                    } else if let Some(s) = d.as_str() {
                        if let Ok(n) = s.parse::<u64>() {
                            Some(format_seconds(n))
                        } else {
                            Some(s.to_string())
                        }
                    } else {
                        None
                    }
                }).unwrap_or_else(|| "00:00".to_string());

                if !hash.is_empty() {
                    items.push(AparatPlaylistItem {
                        hash,
                        title,
                        duration: duration_str,
                        sender_name: sender,
                    });
                }
            }
        }
    }

    Ok(AparatPlaylistInfo {
        id: playlist_id.to_string(),
        title: playlist_title,
        items,
    })
}
