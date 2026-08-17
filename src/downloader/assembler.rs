use std::path::{Path, PathBuf};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn assemble_parts(part_paths: &[PathBuf], final_path: &Path) -> Result<(), String> {
    let mut out_file = File::create(final_path).await.map_err(|e| format!("Failed to create final file: {}", e))?;
    let mut buffer = vec![0u8; 64 * 1024];

    for part in part_paths {
        if let Ok(mut pf) = OpenOptions::new().read(true).open(part).await {
            loop {
                let n = pf.read(&mut buffer).await.map_err(|e| format!("Read part error: {}", e))?;
                if n == 0 { break; }
                out_file.write_all(&buffer[..n]).await.map_err(|e| format!("Write assembled error: {}", e))?;
            }
        }
        let _ = tokio::fs::remove_file(part).await;
    }
    out_file.flush().await.map_err(|e| format!("Flush final error: {}", e))?;
    Ok(())
}
