#[cfg(target_os = "macos")]
use {
    objc2::rc::Retained,
    objc2_app_kit::{NSMenu, NSMenuItem},
    objc2_foundation::{ns_string, MainThreadMarker},
};

#[cfg(target_os = "macos")]
pub fn build(mtm: MainThreadMarker) -> Retained<NSMenuItem> {
    unsafe {
        let parent_item = NSMenuItem::new(mtm);
        parent_item.setTitle(ns_string!("Window"));

        let window_menu = NSMenu::new(mtm);
        window_menu.setTitle(ns_string!("Window"));
        window_menu.setAutoenablesItems(false);

        let minimize = NSMenuItem::new(mtm);
        minimize.setTitle(ns_string!("Minimize"));
        minimize.setAction(Some(objc2::sel!(performMiniaturize:)));
        minimize.setKeyEquivalent(ns_string!("m"));
        minimize.setEnabled(true);

        let zoom = NSMenuItem::new(mtm);
        zoom.setTitle(ns_string!("Zoom"));
        zoom.setAction(Some(objc2::sel!(performZoom:)));
        zoom.setEnabled(true);

        let bring_all = NSMenuItem::new(mtm);
        bring_all.setTitle(ns_string!("Bring All to Front"));
        bring_all.setAction(Some(objc2::sel!(arrangeInFront:)));
        bring_all.setEnabled(true);

        window_menu.addItem(&minimize);
        window_menu.addItem(&zoom);
        window_menu.addItem(&NSMenuItem::separatorItem(mtm));
        window_menu.addItem(&bring_all);

        parent_item.setSubmenu(Some(&window_menu));
        parent_item
    }
}
