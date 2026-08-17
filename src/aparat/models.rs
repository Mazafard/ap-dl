use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoQualityOption {
    pub label: String,
    pub url: String,
    pub urls: Vec<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AparatVideoInfo {
    pub hash: String,
    pub title: String,
    pub description: Option<String>,
    pub duration_seconds: u64,
    pub formatted_duration: String,
    pub sender_name: String,
    pub poster_url: Option<String>,
    pub qualities: Vec<VideoQualityOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AparatPlaylistItem {
    pub hash: String,
    pub title: String,
    pub duration: String,
    pub sender_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AparatPlaylistInfo {
    pub id: String,
    pub title: String,
    pub items: Vec<AparatPlaylistItem>,
}

#[derive(Debug, Clone)]
pub enum FetchResult {
    SingleVideo(AparatVideoInfo),
    Playlist(AparatPlaylistInfo),
}

// Aparat API JSON schemas
#[derive(Debug, Deserialize)]
pub struct AparatApiResponse<T> {
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
pub struct AparatVideoData {
    pub attributes: Option<AparatVideoAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct AparatVideoAttributes {
    pub title: Option<String>,
    pub description: Option<String>,
    pub duration: Option<serde_json::Value>,
    pub sender_name: Option<String>,
    pub small_poster: Option<String>,
    pub big_poster: Option<String>,
    pub file_link_all: Option<Vec<AparatFileLink>>,
}

#[derive(Debug, Deserialize)]
pub struct AparatFileLink {
    pub profile: Option<String>,
    pub urls: Option<Vec<String>>,
}
