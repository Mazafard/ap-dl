use std::time::Duration;

const RAW_CHANGELOG_URL: &str = "https://raw.githubusercontent.com/Mazafard/ap-dl/main/CHANGELOG.md";

pub async fn fetch_raw_changelog() -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let res = client
        .get(RAW_CHANGELOG_URL)
        .header("User-Agent", "APDL-App/0.3")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        res.text().await.map_err(|e| e.to_string())
    } else {
        Err(format!("HTTP {}", res.status()))
    }
}

pub fn extract_section_for_version<'a>(changelog: &'a str, version: &str) -> Option<String> {
    let clean_ver = version.trim_start_matches('v').trim();
    let header_prefix = format!("## [{}]", clean_ver);

    let start = changelog.find(&header_prefix)?;
    let content_after = &changelog[start..];

    // Find the end of this version section (next ## [ or EOF)
    let end = content_after[header_prefix.len()..]
        .find("\n## [")
        .map(|idx| header_prefix.len() + idx)
        .unwrap_or(content_after.len());

    let section = content_after[..end].trim();
    Some(section.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_section() {
        let text = "# Changelog\n\n## [0.3.0] - 2026-08-18\n### Added\n- Feature A\n\n## [0.2.0]\n- Feature B";
        let sec = extract_section_for_version(text, "v0.3.0");
        assert!(sec.is_some());
        let val = sec.unwrap();
        assert!(val.contains("Feature A"));
        assert!(!val.contains("Feature B"));
    }
}
