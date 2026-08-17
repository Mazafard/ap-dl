use crate::state::AppState;
use crate::AppWindow;
use slint::Weak;
use std::sync::Arc;

pub fn bind_browse_path(app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
    app.on_browse_path(move || {
        let state = state.clone();
        let handle = handle.clone();
        tokio::spawn(async move {
            let current_dir = { state.download_dir.read().await.clone() };
            let dialog = rfd::AsyncFileDialog::new().set_directory(current_dir);
            if let Some(folder) = dialog.pick_folder().await {
                let path = folder.path().to_path_buf();
                log::info!("Destination folder updated to: {:?}", path);
                {
                    let mut write = state.download_dir.write().await;
                    *write = path.clone();
                }
                let path_str = path.to_string_lossy().to_string();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = handle.upgrade() {
                        ui.set_destination_path(path_str.into());
                    }
                });
            }
        });
    });
}
