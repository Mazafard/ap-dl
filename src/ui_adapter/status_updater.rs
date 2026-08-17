use crate::state::AppState;
use crate::AppWindow;
use slint::{ModelRc, VecModel, Weak};
use std::sync::Arc;

pub async fn update_item_status(id: &str, state: &Arc<AppState>, handle: &Weak<AppWindow>, status_code: i32, text: &str) {
    let mut items = state.media_items.lock().await;
    for item in items.iter_mut() {
        if item.id.as_str() == id {
            item.status_code = status_code;
            item.speed_text = text.to_string().into();
            if status_code == 2 { item.progress = 1.0; }
            break;
        }
    }
    let snapshot = items.clone();
    drop(items);

    let handle_ui = handle.clone();
    let text_str = text.to_string();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(ui) = handle_ui.upgrade() {
            ui.set_media_items(ModelRc::new(VecModel::from(snapshot)));
            ui.set_total_speed(text_str.into());
        }
    });
}
