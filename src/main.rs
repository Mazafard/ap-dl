mod aparat;
mod downloader;
mod error;
mod logger;
mod menu;
mod state;
mod ui_adapter;

use state::AppState;
use ui_adapter::UiAdapter;

use std::path::PathBuf;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();
    let app = AppWindow::new()?;

    #[cfg(target_os = "macos")]
    {
        set_macos_dock_icon();
        setup_macos_window();
        let _ = slint::invoke_from_event_loop(|| {
            set_macos_dock_icon();
            setup_macos_window();
        });
    }

    // Default download directory: User's Downloads folder
    let default_download_dir = dirs::download_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Aparat");
    let _ = std::fs::create_dir_all(&default_download_dir);
    app.set_destination_path(default_download_dir.to_string_lossy().to_string().into());

    // Centralized Application State
    let state = AppState::new(default_download_dir);

    // Initialize Native Application Menu Bar
    menu::MenuManager::setup(&app, state.clone());

    #[cfg(target_os = "macos")]
    {
        let state_c = state.clone();
        let app_weak = app.as_weak();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = app_weak.upgrade() {
                menu::MenuManager::setup(&ui, state_c);
            }
        });
    }

    // Attach UI Event Adapter
    UiAdapter::attach(&app, state);

    // Run Slint Event Loop
    app.run()?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn setup_macos_window() {
    use objc2_app_kit::{
        NSApplication, NSAutoresizingMaskOptions, NSColor, NSVisualEffectBlendingMode,
        NSVisualEffectMaterial, NSVisualEffectState, NSVisualEffectView, NSWindowOrderingMode,
        NSWindowStyleMask, NSWindowTitleVisibility,
    };
    use objc2_foundation::MainThreadMarker;

    if let Some(mtm) = MainThreadMarker::new() {
        let app = NSApplication::sharedApplication(mtm);
        let windows = app.windows();
        for window in windows.iter() {
            window.setTitlebarAppearsTransparent(true);
            window.setTitleVisibility(NSWindowTitleVisibility::NSWindowTitleHidden);
            let mut style = window.styleMask();
            style.insert(NSWindowStyleMask::FullSizeContentView);
            window.setStyleMask(style);
            window.setMovableByWindowBackground(true);
            window.setOpaque(false);

            let clear_color = unsafe { NSColor::clearColor() };
            window.setBackgroundColor(Some(&clear_color));

            // Attach native full-window frosted glass NSVisualEffectView behind the Slint view
            if let Some(content_view) = window.contentView() {
                let bounds = content_view.bounds();
                let effect_view = unsafe {
                    let view = NSVisualEffectView::new(mtm);
                    view.setFrame(bounds);
                    view.setMaterial(NSVisualEffectMaterial::HUDWindow);
                    view.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
                    view.setState(NSVisualEffectState::Active);
                    view.setAutoresizingMask(
                        NSAutoresizingMaskOptions::NSViewWidthSizable
                            | NSAutoresizingMaskOptions::NSViewHeightSizable,
                    );
                    view
                };

                unsafe {
                    if let Some(superview) = content_view.superview() {
                        superview.addSubview_positioned_relativeTo(
                            &effect_view,
                            NSWindowOrderingMode::NSWindowBelow,
                            Some(&content_view),
                        );
                    } else {
                        content_view.addSubview_positioned_relativeTo(
                            &effect_view,
                            NSWindowOrderingMode::NSWindowBelow,
                            None,
                        );
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn set_macos_dock_icon() {
    use objc2::ClassType;
    use objc2_app_kit::{NSApplication, NSImage};
    use objc2_foundation::{MainThreadMarker, NSData};

    if let Some(mtm) = MainThreadMarker::new() {
        let icon_bytes = include_bytes!("../assets/icon.png");
        let data = NSData::with_bytes(icon_bytes);
        if let Some(image) = NSImage::initWithData(NSImage::alloc(), &data) {
            let app = NSApplication::sharedApplication(mtm);
            unsafe {
                app.setApplicationIconImage(Some(&image));
            }
        }
    }
}
