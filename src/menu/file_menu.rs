#[cfg(target_os = "macos")]
use crate::menu::handler::ApdlMenuTarget;
#[cfg(target_os = "macos")]
use objc2::rc::Retained;
#[cfg(target_os = "macos")]
use objc2_app_kit::{NSMenu, NSMenuItem};
#[cfg(target_os = "macos")]
use objc2_foundation::{ns_string, MainThreadMarker};

#[cfg(target_os = "macos")]
pub fn build(mtm: MainThreadMarker, target: &ApdlMenuTarget) -> Retained<NSMenuItem> {
    unsafe {
        let parent_item = NSMenuItem::new(mtm);
        parent_item.setTitle(ns_string!("File"));

        let file_menu = NSMenu::new(mtm);
        file_menu.setTitle(ns_string!("File"));

        let add_item = NSMenuItem::new(mtm);
        add_item.setTitle(ns_string!("Add Stream / Playlist..."));
        add_item.setAction(Some(objc2::sel!(addLink:)));
        add_item.setKeyEquivalent(ns_string!("n"));
        add_item.setTarget(Some(target));

        let open_item = NSMenuItem::new(mtm);
        open_item.setTitle(ns_string!("Open Downloads Folder"));
        open_item.setAction(Some(objc2::sel!(openFolder:)));
        open_item.setKeyEquivalent(ns_string!("o"));
        open_item.setTarget(Some(target));

        let close_item = NSMenuItem::new(mtm);
        close_item.setTitle(ns_string!("Close Window"));
        close_item.setAction(Some(objc2::sel!(performClose:)));
        close_item.setKeyEquivalent(ns_string!("w"));

        file_menu.addItem(&add_item);
        file_menu.addItem(&open_item);
        file_menu.addItem(&NSMenuItem::separatorItem(mtm));
        file_menu.addItem(&close_item);

        parent_item.setSubmenu(Some(&file_menu));
        parent_item
    }
}
