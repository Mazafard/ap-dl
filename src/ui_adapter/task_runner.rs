use super::status_updater::update_item_status;
use crate::aparat::AparatClient;
use crate::downloader::DownloadTask;
use crate::error::AppError;
use crate::state::{AppState, InternalTask};
use crate::AppWindow;
use slint::{ModelRc, VecModel, Weak};
use std::sync::Arc;

pub async fn execute_single_download(task: InternalTask, state: Arc<AppState>, handle: Weak<AppWindow>) {
    update_item_status(&task.id, &state, &handle, 1, "Connecting...").await;
    let client = AparatClient::new();
    let (stream_url, candidate_urls) = match task.direct_url {
        Some(url) => (Ok(url), task.candidate_urls.clone()),
        None => match client.fetch_video_info(&task.video_hash).await {
            Ok(info) => {
                let first = info.qualities.first();
                (first.map(|q| q.url.clone()).ok_or_else(|| "No stream available".to_string()), first.map(|q| q.urls.clone()).unwrap_or_default())
            }
            Err(e) => (Err(e), Vec::new()),
        },
    };

    match stream_url {
        Ok(url) => {
            let out_dir = { state.download_dir.read().await.clone() };
            let download_task = DownloadTask { url, candidate_urls, title: task.title.clone(), destination_folder: out_dir, cancel_flag: state.cancel_flag.clone() };
            let s_in = state.clone();
            let h_in = handle.clone();
            let tid = task.id.clone();

            let res = download_task.run(move |p| {
                let s = s_in.clone(); let h = h_in.clone(); let t = tid.clone();
                tokio::spawn(async move {
                    let mut items = s.media_items.lock().await;
                    for item in items.iter_mut() {
                        if item.id.as_str() == t { item.progress = p.progress_ratio; item.speed_text = p.speed_formatted.clone().into(); item.file_size = p.size_formatted.clone().into(); break; }
                    }
                    let snap = items.clone(); drop(items);
                    let _ = slint::invoke_from_event_loop(move || { if let Some(ui) = h.upgrade() { ui.set_media_items(ModelRc::new(VecModel::from(snap))); ui.set_total_speed(format!("Downloading • {}", p.speed_formatted).into()); } });
                });
            }).await;

            match res {
                Ok(_) => update_item_status(&task.id, &state, &handle, 2, "Completed ✓").await,
                Err(e) if e.contains("paused") => update_item_status(&task.id, &state, &handle, 3, "Paused").await,
                Err(e) => update_item_status(&task.id, &state, &handle, 3, &AppError::from_err_string(&e).to_ui_summary()).await,
            }
        }
        Err(e) => update_item_status(&task.id, &state, &handle, 3, &AppError::from_err_string(&e).to_ui_summary()).await,
    }
}
