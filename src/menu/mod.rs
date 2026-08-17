pub mod app_menu;
pub mod edit_menu;
pub mod file_menu;
pub mod handler;
pub mod help_menu;
pub mod view_menu;
pub mod window_menu;

use crate::state::AppState;
use crate::AppWindow;
use slint::ComponentHandle;
use std::sync::Arc;

#[cfg(target_os = "macos")]
static MENUBAR_STORAGE: std::sync::atomic::AtomicPtr<objc2_app_kit::NSMenu> =
    std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

pub struct MenuManager;

impl MenuManager {
    pub fn setup(app: &AppWindow, state: Arc<AppState>) {
        #[cfg(target_os = "macos")]
        {
            use objc2::rc::Retained;
            use objc2_app_kit::{NSApplication, NSMenu};
            use objc2_foundation::MainThreadMarker;

            if let Some(mtm) = MainThreadMarker::new() {
                let ns_app = NSApplication::sharedApplication(mtm);
                let menubar = NSMenu::new(mtm);

                let target = handler::create_target(mtm, app.as_weak(), state);

                let app_item = app_menu::build(mtm);
                let file_item = file_menu::build(mtm, &target);
                let edit_item = edit_menu::build(mtm);
                let view_item = view_menu::build(mtm);
                let window_item = window_menu::build(mtm);
                let help_item = help_menu::build(mtm, &target);

                menubar.addItem(&app_item);
                menubar.addItem(&file_item);
                menubar.addItem(&edit_item);
                menubar.addItem(&view_item);
                menubar.addItem(&window_item);
                menubar.addItem(&help_item);

                unsafe {
                    if let Some(win_sub) = window_item.submenu() {
                        ns_app.setWindowsMenu(Some(&win_sub));
                    }
                    if let Some(help_sub) = help_item.submenu() {
                        ns_app.setHelpMenu(Some(&help_sub));
                    }
                    ns_app.setMainMenu(Some(&menubar));
                }

                // Prevent Rust from dropping menubar at function exit
                let raw = Retained::into_raw(menubar);
                let old = MENUBAR_STORAGE.swap(raw as *mut _, std::sync::atomic::Ordering::SeqCst);
                if !old.is_null() {
                    unsafe {
                        let _ = Retained::from_raw(old as *mut NSMenu);
                    }
                }

                log::info!("Native macOS menu bar attached and persisted permanently");
            }
        }
    }
}
