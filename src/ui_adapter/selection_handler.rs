use crate::state::AppState;
use crate::AppWindow;
use slint::{ModelRc, VecModel, Weak};
use std::sync::Arc;

pub fn bind_item_selection(app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
    app.on_toggle_item(move |idx| {
        let state = state.clone();
        let handle = handle.clone();
        let index = idx as usize;

        tokio::spawn(async move {
            let mut items = state.media_items.lock().await;
            if index < items.len() {
                items[index].selected = !items[index].selected;
            }
            let snapshot = items.clone();
            drop(items);

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = handle.upgrade() {
                    ui.set_media_items(ModelRc::new(VecModel::from(snapshot)));
                }
            });
        });
    });
}

pub fn bind_select_all(app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
    app.on_toggle_select_all(move |select| {
        let state = state.clone();
        let handle = handle.clone();

        tokio::spawn(async move {
            let mut items = state.media_items.lock().await;
            for item in items.iter_mut() {
                item.selected = select;
            }
            let snapshot = items.clone();
            drop(items);

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = handle.upgrade() {
                    ui.set_media_items(ModelRc::new(VecModel::from(snapshot)));
                }
            });
        });
    });
}
