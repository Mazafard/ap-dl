use crate::aparat::api::AparatClient;
use crate::aparat::models::FetchResult;
use crate::downloader::worker::DownloadTask;
use crate::error::AppError;
use crate::state::{AppState, InternalTask};
use crate::{AppWindow, MediaItem};

use slint::{ComponentHandle, ModelRc, VecModel, Weak};
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub struct UiAdapter;

impl UiAdapter {
    pub fn attach(app: &AppWindow, state: Arc<AppState>) {
        let app_handle = app.as_weak();

        // 1. Browse Destination Folder
        Self::bind_browse_path(app, state.clone(), app_handle.clone());

        // 2. Clipboard Paste
        Self::bind_clipboard_paste(app, app_handle.clone());

        // 3. Submit URL
        Self::bind_submit_url(app, state.clone(), app_handle.clone());

        // 4. Item Selection
        Self::bind_item_selection(app, state.clone(), app_handle.clone());

        // 5. Select All / Deselect All
        Self::bind_select_all(app, state.clone(), app_handle.clone());

        // 6. Start Download Process
        Self::bind_start_download(app, state.clone(), app_handle.clone());

        // 7. Cancel Download Pipeline
        Self::bind_cancel_download(app, state, app_handle);
    }

    fn bind_browse_path(app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
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
                        let mut lock = state.download_dir.write().await;
                        *lock = path.clone();
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

    fn bind_clipboard_paste(app: &AppWindow, handle: Weak<AppWindow>) {
        app.on_paste_from_clipboard(move || {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if let Ok(text) = clipboard.get_text() {
                    let trimmed = text.trim().to_string();
                    let handle_ui = handle.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = handle_ui.upgrade() {
                            ui.set_modal_url_input(trimmed.into());
                        }
                    });
                }
            }
        });
    }

    fn bind_submit_url(app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
        app.on_submit_url(move |input_url| {
            let state = state.clone();
            let handle = handle.clone();
            let url_str = input_url.to_string().trim().to_string();

            if url_str.is_empty() {
                return;
            }

            log::info!("Resolving Aparat URL: {}", url_str);

            let handle_ui = handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = handle_ui.upgrade() {
                    ui.set_is_analyzing(true);
                    ui.set_validation_message("".into());
                }
            });

            tokio::spawn(async move {
                let client = AparatClient::new();
                match client.resolve_url(&url_str).await {
                    Ok(FetchResult::Playlist(playlist)) => {
                        let mut new_items = Vec::new();
                        let mut new_tasks = Vec::new();

                        for (idx, video) in playlist.items.into_iter().enumerate() {
                            let item_id = format!("{}_{}", video.hash, idx);
                            let index_label = format!("{:02}", idx + 1);
                            let resolution = "Auto".to_string();

                            new_items.push(MediaItem {
                                id: item_id.clone().into(),
                                index_label: index_label.into(),
                                title: video.title.clone().into(),
                                duration: video.duration.into(),
                                resolution: resolution.into(),
                                file_size: "Auto".into(),
                                progress: 0.0,
                                speed_text: "Queued".into(),
                                status_code: 0,
                                selected: true,
                            });

                            new_tasks.push(InternalTask {
                                id: item_id,
                                video_hash: video.hash,
                                title: video.title,
                                direct_url: None,
                                candidate_urls: Vec::new(),
                            });
                        }

                        let count = new_items.len();
                        log::info!("Loaded playlist '{}' with {} videos", playlist.title, count);

                        {
                            let mut items_lock = state.media_items.lock().await;
                            *items_lock = new_items;

                            let mut tasks_lock = state.tasks.lock().await;
                            *tasks_lock = new_tasks;
                        }

                        let snapshot = state.media_items.lock().await.clone();
                        let title = playlist.title.clone();
                        let details = format!("{} videos ready for batch download", count);

                        let handle_ui = handle.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = handle_ui.upgrade() {
                                ui.set_is_analyzing(false);
                                ui.set_show_url_modal(false);
                                ui.set_modal_url_input("".into());
                                ui.set_collection_title(title.into());
                                ui.set_collection_details(details.into());
                                ui.set_total_speed(format!("{} items loaded", count).into());
                                ui.set_media_items(ModelRc::new(VecModel::from(snapshot)));
                            }
                        });
                    }
                    Ok(FetchResult::SingleVideo(video)) => {
                        let best_quality = video.qualities.first();
                        let quality_label = best_quality.map(|q| q.label.clone()).unwrap_or_else(|| "Default".to_string());
                        let direct_url = best_quality.map(|q| q.url.clone());
                        let candidate_urls = best_quality.map(|q| q.urls.clone()).unwrap_or_default();
                        let duration = video.formatted_duration.clone();
                        let author = video.sender_name.clone();

                        let item = MediaItem {
                            id: video.hash.clone().into(),
                            index_label: "01".into(),
                            title: video.title.clone().into(),
                            duration: duration.clone().into(),
                            resolution: quality_label.into(),
                            file_size: "Auto".into(),
                            progress: 0.0,
                            speed_text: "Ready".into(),
                            status_code: 0,
                            selected: true,
                        };

                        let task = InternalTask {
                            id: video.hash.clone(),
                            video_hash: video.hash.clone(),
                            title: video.title.clone(),
                            direct_url,
                            candidate_urls,
                        };

                        log::info!("Loaded single video: '{}'", video.title);

                        {
                            let mut items_lock = state.media_items.lock().await;
                            items_lock.clear();
                            items_lock.push(item);

                            let mut tasks_lock = state.tasks.lock().await;
                            tasks_lock.clear();
                            tasks_lock.push(task);
                        }

                        let snapshot = state.media_items.lock().await.clone();
                        let title = video.title.clone();
                        let details = format!("Channel: {} • Duration: {}", author, duration);

                        let handle_ui = handle.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = handle_ui.upgrade() {
                                ui.set_is_analyzing(false);
                                ui.set_show_url_modal(false);
                                ui.set_modal_url_input("".into());
                                ui.set_collection_title(title.into());
                                ui.set_collection_details(details.into());
                                ui.set_total_speed("1 item ready".into());
                                ui.set_media_items(ModelRc::new(VecModel::from(snapshot)));
                            }
                        });
                    }
                    Err(e) => {
                        let err_string = e.to_string();
                        log::error!("Failed to resolve Aparat link: {}", err_string);
                        let ui_err = AppError::from_err_string(&err_string).to_ui_summary();
                        let handle_ui = handle.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = handle_ui.upgrade() {
                                ui.set_is_analyzing(false);
                                ui.set_validation_message(ui_err.into());
                            }
                        });
                    }
                }
            });
        });
    }

    fn bind_item_selection(app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
        app.on_toggle_item(move |idx| {
            let state = state.clone();
            let handle = handle.clone();
            let idx = idx as usize;
            tokio::spawn(async move {
                let mut items = state.media_items.lock().await;
                if idx < items.len() {
                    items[idx].selected = !items[idx].selected;
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

    fn bind_select_all(app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
        app.on_toggle_select_all(move |select_all| {
            let state = state.clone();
            let handle = handle.clone();
            tokio::spawn(async move {
                let mut items = state.media_items.lock().await;
                for item in items.iter_mut() {
                    item.selected = select_all;
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

    fn bind_start_download(app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
        app.on_start_download(move || {
            let state = state.clone();
            let handle = handle.clone();

            state.cancel_flag.store(false, Ordering::SeqCst);
            log::info!("Starting download pipeline");

            let handle_ui = handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = handle_ui.upgrade() {
                    ui.set_is_downloading(true);
                    ui.set_total_speed("Starting download pipeline...".into());
                }
            });

            tokio::spawn(async move {
                let tasks = { state.tasks.lock().await.clone() };
                let items = { state.media_items.lock().await.clone() };

                // Concurrent Semaphore: 2 parallel video downloads
                let semaphore = Arc::new(tokio::sync::Semaphore::new(2));
                let mut join_set = tokio::task::JoinSet::new();

                for (idx, task) in tasks.into_iter().enumerate() {
                    if state.cancel_flag.load(Ordering::SeqCst) {
                        break;
                    }

                    let is_selected = items.get(idx).map(|it| it.selected).unwrap_or(false);
                    let is_already_completed = items.get(idx).map(|it| it.status_code == 2).unwrap_or(false);

                    if !is_selected || is_already_completed {
                        continue;
                    }

                    // Enforce strict FIFO ordering: acquire permit before spawning task in queue
                    let permit = match semaphore.clone().acquire_owned().await {
                        Ok(p) => p,
                        Err(_) => break,
                    };

                    if state.cancel_flag.load(Ordering::SeqCst) {
                        break;
                    }

                    let state_c = state.clone();
                    let handle_c = handle.clone();

                    join_set.spawn(async move {
                        let _permit = permit;

                        if state_c.cancel_flag.load(Ordering::SeqCst) {
                            return;
                        }

                        Self::update_item_status(&task.id, &state_c, &handle_c, 1, "Connecting...").await;

                        let client = AparatClient::new();
                        let (stream_url, candidate_urls) = match (task.direct_url, task.candidate_urls) {
                            (Some(u), c) if !c.is_empty() => (Ok(u), c),
                            (Some(u), _) => {
                                let mirrors = crate::aparat::api::generate_cdn_mirrors(&u);
                                (Ok(u), mirrors)
                            }
                            (None, _) => match client.fetch_video_info(&task.video_hash).await {
                                Ok(v) => {
                                    let best = v.qualities.first();
                                    let primary = best.map(|q| q.url.clone());
                                    let candidates = best.map(|q| q.urls.clone()).unwrap_or_default();
                                    (primary.ok_or_else(|| "No video quality found".to_string()), candidates)
                                }
                                Err(e) => (Err(e.to_string()), Vec::new()),
                            },
                        };

                        match stream_url {
                            Ok(url) => {
                                let out_dir = { state_c.download_dir.read().await.clone() };
                                let download_task = DownloadTask {
                                    url,
                                    candidate_urls,
                                    title: task.title.clone(),
                                    destination_folder: out_dir,
                                    cancel_flag: state_c.cancel_flag.clone(),
                                };

                                let state_inner = state_c.clone();
                                let handle_inner = handle_c.clone();
                                let task_id = task.id.clone();

                                let res = download_task.run(move |progress| {
                                    let s_inner = state_inner.clone();
                                    let h_inner = handle_inner.clone();
                                    let tid = task_id.clone();

                                    tokio::spawn(async move {
                                        let mut items_lock = s_inner.media_items.lock().await;
                                        for item in items_lock.iter_mut() {
                                            if item.id.as_str() == tid {
                                                item.progress = progress.progress_ratio;
                                                item.speed_text = progress.speed_formatted.clone().into();
                                                item.file_size = progress.size_formatted.clone().into();
                                                break;
                                            }
                                        }
                                        let snapshot = items_lock.clone();
                                        drop(items_lock);

                                        let _ = slint::invoke_from_event_loop(move || {
                                            if let Some(ui) = h_inner.upgrade() {
                                                ui.set_media_items(ModelRc::new(VecModel::from(snapshot)));
                                                ui.set_total_speed(format!("Downloading • {}", progress.speed_formatted).into());
                                            }
                                        });
                                    });
                                }).await;

                                match res {
                                    Ok(_) => {
                                        log::info!("Video completed: '{}'", task.title);
                                        Self::update_item_status(&task.id, &state_c, &handle_c, 2, "Completed ✓").await;
                                    }
                                    Err(e) if e.contains("paused") => {
                                        log::info!("Video paused: '{}'", task.title);
                                        Self::update_item_status(&task.id, &state_c, &handle_c, 3, "Paused ⏸").await;
                                    }
                                    Err(e) => {
                                        log::error!("Video download error for '{}': {}", task.title, e);
                                        let ui_err = AppError::from_err_string(&e).to_ui_summary();
                                        Self::update_item_status(&task.id, &state_c, &handle_c, 3, &ui_err).await;
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to get stream url for '{}': {}", task.title, e);
                                let ui_err = AppError::from_err_string(&e).to_ui_summary();
                                Self::update_item_status(&task.id, &state_c, &handle_c, 3, &ui_err).await;
                            }
                        }
                    });
                }

                while join_set.join_next().await.is_some() {}

                let is_cancelled = state.cancel_flag.load(Ordering::SeqCst);
                let handle_ui = handle.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = handle_ui.upgrade() {
                        ui.set_is_downloading(false);
                        if is_cancelled {
                            ui.set_total_speed("Downloads paused ⏸".into());
                        } else {
                            ui.set_total_speed("Downloads complete ✓".into());
                        }
                    }
                });
            });
        });
    }

    fn bind_cancel_download(_app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
        _app.on_cancel_download(move || {
            state.cancel_flag.store(true, Ordering::SeqCst);
            log::info!("Pause/Cancel requested by user");
            let handle_ui = handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = handle_ui.upgrade() {
                    ui.set_is_downloading(false);
                    ui.set_total_speed("Pausing downloads...".into());
                }
            });
        });
    }

    async fn update_item_status(id: &str, state: &Arc<AppState>, handle: &Weak<AppWindow>, status_code: i32, text: &str) {
        let mut items = state.media_items.lock().await;
        for item in items.iter_mut() {
            if item.id.as_str() == id {
                item.status_code = status_code;
                item.speed_text = text.to_string().into();
                if status_code == 2 {
                    item.progress = 1.0;
                }
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
}
