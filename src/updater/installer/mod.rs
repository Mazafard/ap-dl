pub mod asset_matcher;
pub mod downloader;
pub mod extractor;
pub mod replacer;

use crate::AppWindow;

pub async fn run_install_pipeline(
    url: String,
    app_weak: slint::Weak<AppWindow>,
) -> Result<(), String> {
    let temp_dir = std::env::temp_dir().join("apdl_update_staging");
    let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    tokio::fs::create_dir_all(&temp_dir).await.map_err(|e| e.to_string())?;

    let weak_prog = app_weak.clone();
    let on_progress = move |pct: f32, _down: u64, _total: u64| {
        let _ = weak_prog.upgrade_in_event_loop(move |app| {
            app.set_update_progress(pct);
            let msg = format!("Downloading update ({:.0}%)...", pct * 100.0);
            app.set_update_status(msg.into());
        });
    };

    let archive = downloader::download_archive(&url, &temp_dir, on_progress).await?;

    let _ = app_weak.upgrade_in_event_loop(|app| {
        app.set_update_status("Extracting update payload...".into());
        app.set_update_progress(0.95);
    });

    let extract_dir = temp_dir.join("extracted");
    let new_bin = extractor::extract_payload(&archive, &extract_dir)?;

    let _ = app_weak.upgrade_in_event_loop(|app| {
        app.set_update_status("Installing & Restarting...".into());
        app.set_update_progress(1.0);
    });

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    replacer::replace_and_relaunch(&new_bin)
}
