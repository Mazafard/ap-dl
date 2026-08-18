pub fn parse_version(v: &str) -> (u32, u32, u32) {
    let clean = v.trim().trim_start_matches('v').trim_start_matches('V');
    let parts: Vec<&str> = clean.split('.').collect();
    let major = parts.first().and_then(|p| p.parse().ok()).unwrap_or(0);
    let minor = parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(0);
    let patch = parts.get(2).and_then(|p| {
        // Strip any pre-release suffix e.g. "1-beta" -> "1"
        p.split('-').next().and_then(|s| s.parse().ok())
    }).unwrap_or(0);
    (major, minor, patch)
}

pub fn is_newer_version(latest: &str, current: &str) -> bool {
    let (l_maj, l_min, l_pat) = parse_version(latest);
    let (c_maj, c_min, c_pat) = parse_version(current);

    if l_maj != c_maj {
        l_maj > c_maj
    } else if l_min != c_min {
        l_min > c_min
    } else {
        l_pat > c_pat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        assert_eq!(parse_version("v0.1.2"), (0, 1, 2));
        assert_eq!(parse_version("0.2.0"), (0, 2, 0));
        assert_eq!(parse_version("1.0.0-rc1"), (1, 0, 0));
    }

    #[test]
    fn test_is_newer_version() {
        assert!(is_newer_version("v0.2.0", "0.1.2"));
        assert!(is_newer_version("0.1.3", "0.1.2"));
        assert!(is_newer_version("1.0.0", "0.9.9"));
        assert!(!is_newer_version("0.1.2", "0.1.2"));
        assert!(!is_newer_version("0.1.1", "0.1.2"));
        assert!(!is_newer_version("v0.1.2", "0.1.2"));
    }
}
