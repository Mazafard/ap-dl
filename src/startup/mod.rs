use crate::state::AppState;
use crate::AppWindow;
use slint::ComponentHandle;
use std::sync::Arc;
use std::time::Duration;

pub struct StartupCoordinator;

impl StartupCoordinator {
    pub fn start_sequence(app: &AppWindow, state: Arc<AppState>) {
        let app_weak = app.as_weak();
        let state_c = state;
        app.set_show_splash(true);
        app.set_splash_status("Initializing download engines...".into());
        app.set_splash_progress(0.15);

        tokio::spawn(async move {
            // Stage 1: Engine initialization (15% -> 50%)
            tokio::time::sleep(Duration::from_millis(300)).await;
            let weak1 = app_weak.clone();
            let _ = weak1.upgrade_in_event_loop(|w| {
                w.set_splash_status("Loading checkpoint storage...".into());
                w.set_splash_progress(0.50);
            });

            // Stage 2: Background update check (50% -> 80%)
            tokio::time::sleep(Duration::from_millis(300)).await;
            let weak2 = app_weak.clone();
            let _ = weak2.upgrade_in_event_loop(|w| {
                w.set_splash_status("Connecting to network services...".into());
                w.set_splash_progress(0.80);
            });

            crate::updater::check_for_updates(app_weak.clone(), false);

            // Stage 3: Reaching 100% completion (80% -> 100%)
            tokio::time::sleep(Duration::from_millis(300)).await;
            let weak3 = app_weak.clone();
            let _ = weak3.upgrade_in_event_loop(|w| {
                w.set_splash_status("Ready".into());
                w.set_splash_progress(1.0);
            });

            // Stage 4: Reveal Dashboard & Lock Custom Menubar
            tokio::time::sleep(Duration::from_millis(350)).await;
            let weak4 = app_weak.clone();
            let state_final = state_c.clone();
            let _ = weak4.upgrade_in_event_loop(move |w| {
                w.set_show_splash(false);
                crate::menu::MenuManager::setup(&w, state_final);
            });
        });
    }
}
