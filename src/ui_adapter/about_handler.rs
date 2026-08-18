use crate::AppWindow;
use slint::Weak;

pub fn bind_about_handlers(app: &AppWindow, app_handle: Weak<AppWindow>) {
    app.on_open_repo(move || {
        let _ = open::that("https://github.com/Mazafard/ap-dl");
    });

    app.on_open_issues(move || {
        let _ = open::that("https://github.com/Mazafard/ap-dl/issues");
    });

    app.on_open_docs(move || {
        let _ = open::that("https://github.com/Mazafard/ap-dl#readme");
    });

    let handle_check = app_handle;
    app.on_check_updates_from_about(move || {
        if let Some(ui) = handle_check.upgrade() {
            ui.set_show_about_dialog(false);
            crate::updater::check_for_updates(handle_check.clone(), true);
        }
    });
}
