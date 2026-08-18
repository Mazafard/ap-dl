pub mod aparat;
pub mod downloader;
pub mod error;
pub mod logger;
pub mod menu;
pub mod startup;
pub mod state;
pub mod ui_adapter;
pub mod updater;
pub mod window_setup;

use state::AppState;
use ui_adapter::UiAdapter;
use std::path::PathBuf;

slint::include_modules!();

pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();
    let app = AppWindow::new()?;

    #[cfg(target_os = "macos")]
    {
        window_setup::set_macos_dock_icon();
        window_setup::setup_macos_window();
        let _ = slint::invoke_from_event_loop(|| {
            window_setup::set_macos_dock_icon();
            window_setup::setup_macos_window();
        });
    }

    let default_download_dir = dirs::download_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Aparat");
    let _ = std::fs::create_dir_all(&default_download_dir);
    app.set_destination_path(default_download_dir.to_string_lossy().to_string().into());

    let state = AppState::new(default_download_dir);
    menu::MenuManager::setup(&app, state.clone());

    UiAdapter::attach(&app, state.clone());
    startup::StartupCoordinator::start_sequence(&app, state);
    app.run()?;
    Ok(())
}
