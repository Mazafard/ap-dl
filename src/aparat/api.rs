use super::models::*;
use regex::Regex;
use std::error::Error;

pub struct AparatClient {
    client: reqwest::Client,
}

impl AparatClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .danger_accept_invalid_certs(true)
            .connect_timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { client }
    }

    /// Extracts video hash or playlist ID from an input URL or raw string.
    pub fn parse_input_url(input: &str) -> Option<(bool, String)> {
        let input = input.trim();

        // 1. Check if it's a playlist URL (e.g. aparat.com/playlist/12345 or ?playlist=12345)
        let playlist_regex = Regex::new(r"(?:playlist/|playlist=)([a-zA-Z0-9]+)").ok()?;
        if let Some(caps) = playlist_regex.captures(input) {
            return Some((true, caps.get(1)?.as_str().to_string()));
        }

        // 2. Check if it's a video URL (e.g. aparat.com/v/XXXXX or aparat.com/v/XXXXX/title)
        let video_regex = Regex::new(r"(?:aparat\.com/v/|/v/)([a-zA-Z0-9]+)").ok()?;
        if let Some(caps) = video_regex.captures(input) {
            return Some((false, caps.get(1)?.as_str().to_string()));
        }

        // 3. Raw hash (alphanumeric 5-10 chars)
        let raw_hash_regex = Regex::new(r"^[a-zA-Z0-9]{4,15}$").ok()?;
        if raw_hash_regex.is_match(input) {
            return Some((false, input.to_string()));
        }

        None
    }

    /// Fetches single video metadata and direct download qualities from Aparat API.
    pub async fn fetch_video_info(&self, hash: &str) -> Result<AparatVideoInfo, Box<dyn Error + Send + Sync>> {
        let api_url = format!("https://www.aparat.com/api/fa/v1/video/video/show/videohash/{}", hash);
        
        let response = self.client
            .get(&api_url)
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<AparatApiResponse<AparatVideoData>>()
            .await?;

        let video_data = response.data.ok_or("No data found in Aparat response")?;
        let attrs = video_data.attributes.ok_or("No attributes found in video data")?;

        let title = attrs.title.unwrap_or_else(|| format!("aparat_{}", hash));
        let sender_name = attrs.sender_name.unwrap_or_else(|| "Unknown".to_string());
        let poster_url = attrs.big_poster.or(attrs.small_poster);

        let duration_seconds = match attrs.duration {
            Some(serde_json::Value::Number(num)) => num.as_u64().unwrap_or(0),
            Some(serde_json::Value::String(s)) => s.parse::<u64>().unwrap_or(0),
            _ => 0,
        };

        let formatted_duration = format_seconds(duration_seconds);

        let mut qualities = Vec::new();
        if let Some(links) = attrs.file_link_all {
            for link in links {
                let profile = link.profile.unwrap_or_else(|| "default".to_string());
                if let Some(urls) = link.urls {
                    if let Some(direct_url) = urls.first() {
                        let mut all_urls = urls.clone();
                        // Generate known fallback CDN mirrors if not already in list
                        for fallback in generate_cdn_mirrors(direct_url) {
                            if !all_urls.contains(&fallback) {
                                all_urls.push(fallback);
                            }
                        }

                        qualities.push(VideoQualityOption {
                            label: format_quality_label(&profile),
                            url: direct_url.clone(),
                            urls: all_urls,
                            profile: Some(profile),
                        });
                    }
                }
            }
        }

        // Sort qualities from highest to lowest (1080p -> 720p -> 480p -> 360p -> 240p -> 144p)
        qualities.sort_by(|a, b| {
            quality_rank(&b.label).cmp(&quality_rank(&a.label))
        });

        Ok(AparatVideoInfo {
            hash: hash.to_string(),
            title,
            description: attrs.description,
            duration_seconds,
            formatted_duration,
            sender_name,
            poster_url,
            qualities,
        })
    }

    /// Fetches playlist metadata from Aparat API.
    pub async fn fetch_playlist_info(&self, playlist_id: &str) -> Result<AparatPlaylistInfo, Box<dyn Error + Send + Sync>> {
        let api_url = format!("https://www.aparat.com/api/fa/v1/video/playlist/one/playlist_id/{}", playlist_id);

        let response = self.client
            .get(&api_url)
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let title = response.pointer("/data/attributes/title")
            .and_then(|v| v.as_str())
            .unwrap_or("Aparat Playlist")
            .to_string();

        let mut items = Vec::new();
        if let Some(included_array) = response.get("included").and_then(|v| v.as_array()) {
            for item in included_array {
                let item_type = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
                if item_type.eq_ignore_ascii_case("video") || item_type.is_empty() {
                    let hash = item.pointer("/attributes/uid")
                        .or_else(|| item.pointer("/attributes/videohash"))
                        .or_else(|| item.get("id"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    let item_title = item.pointer("/attributes/title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Untitled Video")
                        .to_string();

                    let sender = item.pointer("/attributes/sender_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Aparat")
                        .to_string();

                    let duration_raw = match item.pointer("/attributes/duration") {
                        Some(serde_json::Value::Number(num)) => num.as_u64().unwrap_or(0),
                        Some(serde_json::Value::String(s)) => s.parse::<u64>().unwrap_or(0),
                        _ => 0,
                    };

                    if !hash.is_empty() {
                        items.push(AparatPlaylistItem {
                            hash,
                            title: item_title,
                            duration: format_seconds(duration_raw),
                            sender_name: sender,
                        });
                    }
                }
            }
        }

        Ok(AparatPlaylistInfo {
            id: playlist_id.to_string(),
            title,
            items,
        })
    }

    /// High-level resolver for arbitrary URL strings
    pub async fn resolve_url(&self, input: &str) -> Result<FetchResult, Box<dyn Error + Send + Sync>> {
        let (is_playlist, id) = Self::parse_input_url(input)
            .ok_or("Invalid Aparat URL or Video ID format")?;

        if is_playlist {
            let playlist = self.fetch_playlist_info(&id).await?;
            Ok(FetchResult::Playlist(playlist))
        } else {
            let video = self.fetch_video_info(&id).await?;
            Ok(FetchResult::SingleVideo(video))
        }
    }
}

fn format_seconds(secs: u64) -> String {
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let seconds = secs % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, mins, seconds)
    } else {
        format!("{:02}:{:02}", mins, seconds)
    }
}

fn format_quality_label(profile: &str) -> String {
    let lower = profile.to_lowercase();
    if lower.contains("1080") {
        "1080p (Full HD)".to_string()
    } else if lower.contains("720") {
        "720p (HD)".to_string()
    } else if lower.contains("480") {
        "480p (SD)".to_string()
    } else if lower.contains("360") {
        "360p".to_string()
    } else if lower.contains("240") {
        "240p".to_string()
    } else if lower.contains("144") {
        "144p".to_string()
    } else {
        profile.to_string()
    }
}

fn quality_rank(label: &str) -> u32 {
    if label.contains("1080") {
        1080
    } else if label.contains("720") {
        720
    } else if label.contains("480") {
        480
    } else if label.contains("360") {
        360
    } else if label.contains("240") {
        240
    } else if label.contains("144") {
        144
    } else {
        0
    }
}

pub fn generate_cdn_mirrors(url: &str) -> Vec<String> {
    let mut mirrors = Vec::new();
    let re = match Regex::new(r"https?://([a-zA-Z0-9]+)\.cdn\.asset\.aparat\.com(/.*)") {
        Ok(r) => r,
        Err(_) => return mirrors,
    };

    if let Some(caps) = re.captures(url) {
        let current_node = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let path_and_query = caps.get(2).map(|m| m.as_str()).unwrap_or("");

        // High-availability Iranian CDN edge clusters ordered by reliability & speed
        let fallback_clusters = [
            "persian8", "persian9", "persian14", "persian1", "persian2",
            "as1", "as2", "arvan1", "arvan2", "m1", "m2",
            "caspian1", "caspian2", "caspian12", "caspian20",
        ];

        for cluster in fallback_clusters {
            if cluster != current_node {
                mirrors.push(format!("https://{}.cdn.asset.aparat.com{}", cluster, path_and_query));
            }
        }
    }

    mirrors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_parser() {
        assert_eq!(
            AparatClient::parse_input_url("https://www.aparat.com/playlist/26824651"),
            Some((true, "26824651".to_string()))
        );
        assert_eq!(
            AparatClient::parse_input_url("https://www.aparat.com/v/hbq2608"),
            Some((false, "hbq2608".to_string()))
        );
    }

    #[tokio::test]
    async fn test_fetch_playlist() {
        let client = AparatClient::new();
        let playlist = client.fetch_playlist_info("26824651").await.unwrap();
        assert!(!playlist.items.is_empty(), "Playlist items should not be empty");
        println!("Playlist '{}' fetched {} items", playlist.title, playlist.items.len());
    }
}
