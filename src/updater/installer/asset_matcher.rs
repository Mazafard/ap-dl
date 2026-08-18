use crate::updater::types::GitHubAsset;

pub fn find_matching_asset<'a>(assets: &'a [GitHubAsset]) -> Option<&'a GitHubAsset> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let target_pattern = match (os, arch) {
        ("macos", "aarch64") => "macos-arm64",
        ("macos", "x86_64") => "macos-x86_64",
        ("linux", "x86_64") => "linux-x86_64",
        ("windows", "x86_64") => "windows-x86_64",
        _ => return None,
    };

    assets.iter().find(|a| {
        let name = a.name.to_lowercase();
        name.contains(target_pattern) && (name.ends_with(".zip") || name.ends_with(".tar.gz"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_matching() {
        let assets = vec![
            GitHubAsset { name: "ap-dl-v0.2.0-macos-arm64.zip".into(), browser_download_url: "url1".into(), size: 100 },
            GitHubAsset { name: "ap-dl-v0.2.0-linux-x86_64.tar.gz".into(), browser_download_url: "url2".into(), size: 100 },
            GitHubAsset { name: "ap-dl-v0.2.0-windows-x86_64.zip".into(), browser_download_url: "url3".into(), size: 100 },
        ];

        let matched = find_matching_asset(&assets);
        assert!(matched.is_some());
    }
}
