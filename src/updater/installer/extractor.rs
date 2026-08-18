use std::fs::File;
use std::path::{Path, PathBuf};

pub fn extract_payload(archive_path: &Path, extract_to: &Path) -> Result<PathBuf, String> {
    let _ = std::fs::create_dir_all(extract_to);
    let path_str = archive_path.to_string_lossy();

    if path_str.ends_with(".zip") {
        let file = File::open(archive_path).map_err(|e| e.to_string())?;
        let mut zip = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
        zip.extract(extract_to).map_err(|e| e.to_string())?;
    } else if path_str.ends_with(".tar.gz") || path_str.ends_with(".tgz") {
        let file = File::open(archive_path).map_err(|e| e.to_string())?;
        let tar = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(extract_to).map_err(|e| e.to_string())?;
    } else {
        return Err("Unsupported archive format".into());
    }

    find_extracted_binary(extract_to)
}

fn find_extracted_binary(dir: &Path) -> Result<PathBuf, String> {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name == "ap-dl" || name == "ap-dl.exe" || name == "APDL" {
                    return Ok(path);
                }
            } else if path.is_dir() {
                if path.extension().and_then(|s| s.to_str()) == Some("app") {
                    let inner_bin = path.join("Contents/MacOS/ap-dl");
                    if inner_bin.exists() {
                        return Ok(inner_bin);
                    }
                }
                if let Ok(found) = find_extracted_binary(&path) {
                    return Ok(found);
                }
            }
        }
    }
    Err("Could not find executable binary in extracted update payload".into())
}
