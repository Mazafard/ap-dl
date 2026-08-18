use crate::AppWindow;
use slint::Weak;

pub fn bind_update_handlers(app: &AppWindow, app_handle: Weak<AppWindow>) {
    let handle = app_handle;
    app.on_download_update(move || {
        if let Some(ui) = handle.upgrade() {
            let url = ui.get_release_url();
            let _ = open::that(url.as_str());
            ui.set_show_update_dialog(false);
        }
    });
}
