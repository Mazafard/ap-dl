use crate::updater::types::GitHubAsset;

pub fn find_matching_asset<'a>(assets: &'a [GitHubAsset]) -> Option<&'a GitHubAsset> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    assets.iter().find(|a| {
        let name = a.name.to_lowercase();
        let is_archive = name.ends_with(".zip") || name.ends_with(".tar.gz");
        if !is_archive {
            return false;
        }

        match (os, arch) {
            ("macos", "aarch64") => name.contains("macos") && (name.contains("arm64") || name.contains("aarch64")),
            ("linux", "x86_64") => name.contains("linux") && (name.contains("x64") || name.contains("x86_64")),
            ("windows", "x86_64") => name.contains("windows") || name.contains("win") && (name.contains("x64") || name.contains("x86_64")),
            _ => false,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_matching() {
        let assets = vec![
            GitHubAsset { name: "ap-dl-macos-arm64.zip".into(), browser_download_url: "url1".into(), size: 100 },
            GitHubAsset { name: "ap-dl-linux-x64.tar.gz".into(), browser_download_url: "url2".into(), size: 100 },
            GitHubAsset { name: "ap-dl-windows-x64.zip".into(), browser_download_url: "url3".into(), size: 100 },
        ];

        let matched = find_matching_asset(&assets);
        assert!(matched.is_some());
    }
}
