use crate::AppWindow;
use slint::Weak;

pub fn bind_update_handlers(app: &AppWindow, app_handle: Weak<AppWindow>) {
    let handle_dl = app_handle.clone();
    app.on_download_update(move || {
        if let Some(ui) = handle_dl.upgrade() {
            let url = ui.get_release_url();
            let _ = open::that(url.as_str());
            ui.set_show_update_dialog(false);
        }
    });

    let handle_inst = app_handle;
    app.on_start_install_update(move || {
        if let Some(ui) = handle_inst.upgrade() {
            let url = ui.get_release_url().to_string();
            ui.set_is_installing_update(true);
            ui.set_update_status("Starting update download...".into());
            ui.set_update_progress(0.05);

            let weak_inst = handle_inst.clone();
            tokio::spawn(async move {
                if let Err(e) = crate::updater::installer::run_install_pipeline(url, weak_inst.clone()).await {
                    let _ = weak_inst.upgrade_in_event_loop(move |u| {
                        u.set_is_installing_update(false);
                        let msg = format!("Update installation failed: {}", e);
                        u.set_toast_message(msg.into());
                        u.set_show_toast(true);
                    });
                }
            });
        }
    });
}
