use crate::updater::github::fetch_latest_release;
use crate::updater::types::{UpdateCheckResult, UpdateInfo};
use crate::updater::version::is_newer_version;
use crate::AppWindow;

pub async fn check_update(current_version: &str) -> UpdateCheckResult {
    if std::env::var("APDL_SIMULATE_UPDATE").is_ok() {
        return UpdateCheckResult::UpdateAvailable(UpdateInfo {
            current_version: current_version.to_string(),
            latest_version: "v0.2.0".to_string(),
            release_name: "APDL v0.2.0 — Major Performance Boost".to_string(),
            release_url: "https://github.com/Mazafard/ap-dl/releases".to_string(),
            release_notes: "• Enhanced Multi-Segment Turbo Download Engine\n• Instant Resume & Integrity Verifier\n• Sleek Frosted Glass macOS UI".to_string(),
        });
    }

    match fetch_latest_release().await {
        Ok(release) => {
            if is_newer_version(&release.tag_name, current_version) {
                UpdateCheckResult::UpdateAvailable(UpdateInfo {
                    current_version: current_version.to_string(),
                    latest_version: release.tag_name.clone(),
                    release_name: release.name.unwrap_or(release.tag_name),
                    release_url: release.html_url,
                    release_notes: release.body.unwrap_or_default(),
                })
            } else {
                UpdateCheckResult::UpToDate(current_version.to_string())
            }
        }
        Err(e) => UpdateCheckResult::Error(e),
    }
}

pub fn apply_update_result(app_weak: slint::Weak<AppWindow>, result: UpdateCheckResult, is_manual: bool) {
    let _ = app_weak.upgrade_in_event_loop(move |app| match result {
        UpdateCheckResult::UpdateAvailable(info) => {
            app.set_update_available(true);
            app.set_latest_version(info.latest_version.into());
            app.set_release_notes(info.release_notes.into());
            app.set_release_url(info.release_url.into());
            app.set_show_update_dialog(true);
        }
        UpdateCheckResult::UpToDate(v) => {
            if is_manual {
                let msg = format!("You're using the latest version (v{})", v);
                app.set_toast_message(msg.into());
                app.set_show_toast(true);
            }
        }
        UpdateCheckResult::Error(err) => {
            log::warn!("Update check error: {}", err);
            if is_manual {
                let msg = format!("Could not check updates: {}", err);
                app.set_toast_message(msg.into());
                app.set_show_toast(true);
            }
        }
    });
}
