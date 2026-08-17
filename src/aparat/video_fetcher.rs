use super::mirrors::{format_quality_label, format_seconds, quality_rank};
use super::models::{AparatApiResponse, AparatVideoAttributes, AparatVideoData, AparatVideoInfo, VideoQualityOption};
use reqwest::Client;

pub async fn fetch_video_info(http_client: &Client, video_hash: &str) -> Result<AparatVideoInfo, String> {
    let url = format!("https://www.aparat.com/api/fa/v1/video/video/show/videohash/{}", video_hash);
    let resp = http_client.get(&url).send().await.map_err(|e| format!("Failed to connect to Aparat API: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Aparat API returned error status: {}", resp.status()));
    }

    let api_res: AparatApiResponse<AparatVideoData> = resp.json().await.map_err(|e| format!("Failed to parse video response: {}", e))?;
    let attributes = api_res.data.and_then(|d| d.attributes).unwrap_or(AparatVideoAttributes {
        title: None,
        description: None,
        duration: None,
        sender_name: None,
        small_poster: None,
        big_poster: None,
        file_link_all: None,
    });

    let duration_secs = attributes.duration.and_then(|d| match d {
        serde_json::Value::Number(n) => n.as_u64(),
        serde_json::Value::String(s) => s.parse::<u64>().ok(),
        _ => None,
    }).unwrap_or(0);

    let mut qualities = Vec::new();
    if let Some(links) = attributes.file_link_all {
        for link in links {
            if let (Some(profile), Some(urls)) = (link.profile, link.urls) {
                if let Some(first_url) = urls.first() {
                    qualities.push(VideoQualityOption {
                        label: format_quality_label(&profile),
                        url: first_url.clone(),
                        urls: urls.clone(),
                        profile: Some(profile),
                    });
                }
            }
        }
    }

    qualities.sort_by(|a, b| quality_rank(&b.label).cmp(&quality_rank(&a.label)));

    Ok(AparatVideoInfo {
        hash: video_hash.to_string(),
        title: attributes.title.unwrap_or_else(|| "Unknown Video".to_string()),
        description: attributes.description,
        duration_seconds: duration_secs,
        formatted_duration: format_seconds(duration_secs),
        sender_name: attributes.sender_name.unwrap_or_else(|| "Aparat Channel".to_string()),
        poster_url: attributes.big_poster.or(attributes.small_poster),
        qualities,
    })
}
