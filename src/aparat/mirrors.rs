use regex::Regex;

pub fn format_seconds(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

pub fn format_quality_label(profile: &str) -> String {
    match profile.to_lowercase().as_str() {
        "1080p" => "1080p (Full HD)".to_string(),
        "720p" => "720p (HD)".to_string(),
        "480p" => "480p (SD)".to_string(),
        "360p" => "360p".to_string(),
        "240p" => "240p".to_string(),
        "144p" => "144p".to_string(),
        _ => profile.to_string(),
    }
}

pub fn quality_rank(label: &str) -> u32 {
    if label.contains("1080p") {
        1080
    } else if label.contains("720p") {
        720
    } else if label.contains("480p") {
        480
    } else if label.contains("360p") {
        360
    } else if label.contains("240p") {
        240
    } else if label.contains("144p") {
        144
    } else {
        0
    }
}

pub fn generate_cdn_mirrors(url: &str) -> Vec<String> {
    let mut mirrors = Vec::new();
    let re = Regex::new(r"https?://([^.]+)\.cdn\.asset\.aparat\.com(/.*)").unwrap();

    if let Some(caps) = re.captures(url) {
        let current_node = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let path_and_query = caps.get(2).map(|m| m.as_str()).unwrap_or("");

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
