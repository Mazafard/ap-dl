use super::item_builder::{build_playlist_items, build_single_video_item};
use crate::aparat::models::FetchResult;
use crate::aparat::AparatClient;
use crate::error::AppError;
use crate::state::AppState;
use crate::AppWindow;
use slint::{ModelRc, VecModel, Weak};
use std::sync::Arc;

pub fn bind_submit_url(app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
    app.on_submit_url(move |url| {
        let url_str = url.to_string();
        let state = state.clone();
        let handle = handle.clone();

        log::info!("Resolving Aparat URL: {}", url_str);
        let h_load = handle.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = h_load.upgrade() {
                ui.set_is_analyzing(true);
                ui.set_validation_message("".into());
            }
        });

        tokio::spawn(async move {
            let client = AparatClient::new();
            match client.resolve_url(&url_str).await {
                Ok(FetchResult::Playlist(playlist)) => {
                    let (new_items, new_tasks) = build_playlist_items(&playlist);
                    let p_title = playlist.title.clone();
                    let p_details = format!("{} videos in playlist", new_items.len());

                    *state.media_items.lock().await = new_items.clone();
                    *state.tasks.lock().await = new_tasks;

                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = handle.upgrade() {
                            ui.set_media_items(ModelRc::new(VecModel::from(new_items)));
                            ui.set_collection_title(p_title.into());
                            ui.set_collection_details(p_details.into());
                            ui.set_is_analyzing(false);
                            ui.set_show_url_modal(false);
                        }
                    });
                }
                Ok(FetchResult::SingleVideo(video)) => {
                    let (single_item, single_task) = build_single_video_item(&video);
                    let title = video.title.clone();

                    *state.media_items.lock().await = vec![single_item.clone()];
                    *state.tasks.lock().await = vec![single_task];

                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = handle.upgrade() {
                            ui.set_media_items(ModelRc::new(VecModel::from(vec![single_item])));
                            ui.set_collection_title(title.into());
                            ui.set_collection_details("Single Video Stream".into());
                            ui.set_is_analyzing(false);
                            ui.set_show_url_modal(false);
                        }
                    });
                }
                Err(err) => {
                    let ui_error = AppError::from_err_string(&err).to_ui_summary();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = handle.upgrade() {
                            ui.set_is_analyzing(false);
                            ui.set_validation_message(ui_error.into());
                        }
                    });
                }
            }
        });
    });
}
