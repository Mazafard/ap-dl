#[cfg(target_os = "macos")]
use objc2::rc::Retained;
#[cfg(target_os = "macos")]
use objc2_app_kit::{NSMenu, NSMenuItem};
#[cfg(target_os = "macos")]
use objc2_foundation::{ns_string, MainThreadMarker};

#[cfg(target_os = "macos")]
pub fn build(mtm: MainThreadMarker) -> Retained<NSMenuItem> {
    unsafe {
        let parent_item = NSMenuItem::new(mtm);
        parent_item.setTitle(ns_string!("View"));

        let view_menu = NSMenu::new(mtm);
        view_menu.setTitle(ns_string!("View"));

        let fullscreen = NSMenuItem::new(mtm);
        fullscreen.setTitle(ns_string!("Toggle Full Screen"));
        fullscreen.setAction(Some(objc2::sel!(toggleFullScreen:)));
        fullscreen.setKeyEquivalent(ns_string!("f"));

        view_menu.addItem(&fullscreen);
        parent_item.setSubmenu(Some(&view_menu));
        parent_item
    }
}
