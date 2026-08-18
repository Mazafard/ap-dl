use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: Option<String>,
    pub html_url: String,
    pub body: Option<String>,
    pub published_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub release_name: String,
    pub release_url: String,
    pub release_notes: String,
}

#[derive(Debug, Clone)]
pub enum UpdateCheckResult {
    UpdateAvailable(UpdateInfo),
    UpToDate(String),
    Error(String),
}
