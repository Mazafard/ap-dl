use crate::updater::types::GitHubRelease;
use std::time::Duration;

const GITHUB_API_URL: &str = "https://api.github.com/repos/Mazafard/ap-dl/releases/latest";
const USER_AGENT: &str = "APDL-App/0.1";

pub async fn fetch_latest_release() -> Result<GitHubRelease, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(6))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {e}"))?;

    let response = client
        .get(GITHUB_API_URL)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| format!("Network request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API returned status: {}", response.status()));
    }

    response
        .json::<GitHubRelease>()
        .await
        .map_err(|e| format!("Failed to parse release JSON: {e}"))
}
