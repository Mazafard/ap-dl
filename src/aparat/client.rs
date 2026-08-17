use super::models::{AparatPlaylistInfo, AparatVideoInfo, FetchResult};
use super::playlist_fetcher::fetch_playlist_info;
use super::video_fetcher::fetch_video_info;
use regex::Regex;
use reqwest::Client;

pub struct AparatClient {
    pub http_client: Client,
}

impl AparatClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::builder()
                .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .build()
                .unwrap_or_default(),
        }
    }

    pub fn parse_input_url(input: &str) -> Option<(bool, String)> {
        let input = input.trim();
        if input.is_empty() {
            return None;
        }

        // 1. Playlist URL matching
        let playlist_re = Regex::new(r"https?://(?:www\.)?aparat\.com/(?:v/[^/]+(?:\?[^#]*playlist=)|playlist/)([0-9]+)").unwrap();
        if let Some(caps) = playlist_re.captures(input) {
            if let Some(m) = caps.get(1) {
                return Some((true, m.as_str().to_string()));
            }
        }

        // 2. Single Video URL matching
        let video_re = Regex::new(r"https?://(?:www\.)?aparat\.com/v/([a-zA-Z0-9]+)").unwrap();
        if let Some(caps) = video_re.captures(input) {
            if let Some(m) = caps.get(1) {
                return Some((false, m.as_str().to_string()));
            }
        }

        // 3. Raw hash
        let raw_re = Regex::new(r"^[a-zA-Z0-9]{5,10}$").unwrap();
        if raw_re.is_match(input) {
            return Some((false, input.to_string()));
        }

        None
    }

    pub async fn fetch_video_info(&self, video_hash: &str) -> Result<AparatVideoInfo, String> {
        fetch_video_info(&self.http_client, video_hash).await
    }

    pub async fn fetch_playlist_info(&self, playlist_id: &str) -> Result<AparatPlaylistInfo, String> {
        fetch_playlist_info(&self.http_client, playlist_id).await
    }

    pub async fn resolve_url(&self, input: &str) -> Result<FetchResult, String> {
        let (is_playlist, id) = Self::parse_input_url(input).ok_or_else(|| "Invalid Aparat URL pattern".to_string())?;
        if is_playlist {
            let playlist = self.fetch_playlist_info(&id).await?;
            Ok(FetchResult::Playlist(playlist))
        } else {
            let video = self.fetch_video_info(&id).await?;
            Ok(FetchResult::SingleVideo(video))
        }
    }
}
