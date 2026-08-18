pub mod about_handler;
pub mod browse_handler;
pub mod clipboard_handler;
pub mod download_handler;
pub mod item_builder;
pub mod selection_handler;
pub mod status_updater;
pub mod task_runner;
pub mod update_handler;
pub mod url_handler;

use crate::state::AppState;
use crate::AppWindow;
use slint::ComponentHandle;
use std::sync::Arc;

pub struct UiAdapter;

impl UiAdapter {
    pub fn attach(app: &AppWindow, state: Arc<AppState>) {
        let app_handle = app.as_weak();

        browse_handler::bind_browse_path(app, state.clone(), app_handle.clone());
        clipboard_handler::bind_clipboard_paste(app, app_handle.clone());
        url_handler::bind_submit_url(app, state.clone(), app_handle.clone());
        selection_handler::bind_item_selection(app, state.clone(), app_handle.clone());
        selection_handler::bind_select_all(app, state.clone(), app_handle.clone());
        download_handler::bind_start_download(app, state.clone(), app_handle.clone());
        download_handler::bind_cancel_download(app, state, app_handle.clone());
        update_handler::bind_update_handlers(app, app_handle.clone());
        about_handler::bind_about_handlers(app, app_handle);
    }
}
