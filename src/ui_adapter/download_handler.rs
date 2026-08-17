use super::task_runner::execute_single_download;
use crate::state::{AppState, InternalTask};
use crate::AppWindow;
use slint::Weak;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::task::JoinSet;

pub fn bind_start_download(app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
    app.on_start_download(move || {
        let state = state.clone();
        let handle = handle.clone();
        state.cancel_flag.store(false, Ordering::SeqCst);

        let h_start = handle.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = h_start.upgrade() {
                ui.set_is_downloading(true);
                ui.set_total_speed("Starting downloads...".into());
            }
        });

        tokio::spawn(async move { run_download_pipeline(state, handle).await; });
    });
}

pub fn bind_cancel_download(app: &AppWindow, state: Arc<AppState>, handle: Weak<AppWindow>) {
    app.on_cancel_download(move || {
        state.cancel_flag.store(true, Ordering::SeqCst);
        let handle_ui = handle.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = handle_ui.upgrade() {
                ui.set_is_downloading(false);
                ui.set_total_speed("Pausing downloads...".into());
            }
        });
    });
}

async fn run_download_pipeline(state: Arc<AppState>, handle: Weak<AppWindow>) {
    let tasks_to_run: Vec<InternalTask> = {
        let items = state.media_items.lock().await;
        let tasks = state.tasks.lock().await;
        tasks.iter().filter(|t| items.iter().any(|i| i.id.as_str() == t.id && i.selected && i.status_code != 2)).cloned().collect()
    };

    let mut join_set = JoinSet::new();
    let semaphore = Arc::new(tokio::sync::Semaphore::new(2));

    for task in tasks_to_run {
        let sem = semaphore.clone();
        let state_c = state.clone();
        let handle_c = handle.clone();

        join_set.spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            if state_c.cancel_flag.load(Ordering::SeqCst) { return; }
            execute_single_download(task, state_c, handle_c).await;
        });
    }

    while join_set.join_next().await.is_some() {}
    let is_cancelled = state.cancel_flag.load(Ordering::SeqCst);
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(ui) = handle.upgrade() {
            ui.set_is_downloading(false);
            ui.set_total_speed(if is_cancelled { "Downloads paused".into() } else { "Downloads complete ✓".into() });
        }
    });
}
