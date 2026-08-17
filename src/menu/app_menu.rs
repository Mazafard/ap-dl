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
        parent_item.setTitle(ns_string!("APDL"));

        let app_menu = NSMenu::new(mtm);
        app_menu.setTitle(ns_string!("APDL"));

        let about_item = NSMenuItem::new(mtm);
        about_item.setTitle(ns_string!("About APDL"));
        about_item.setAction(Some(objc2::sel!(orderFrontStandardAboutPanel:)));

        let hide_item = NSMenuItem::new(mtm);
        hide_item.setTitle(ns_string!("Hide APDL"));
        hide_item.setAction(Some(objc2::sel!(hide:)));
        hide_item.setKeyEquivalent(ns_string!("h"));

        let hide_others = NSMenuItem::new(mtm);
        hide_others.setTitle(ns_string!("Hide Others"));
        hide_others.setAction(Some(objc2::sel!(hideOtherApplications:)));
        hide_others.setKeyEquivalent(ns_string!("h"));

        let show_all = NSMenuItem::new(mtm);
        show_all.setTitle(ns_string!("Show All"));
        show_all.setAction(Some(objc2::sel!(unhideAllApplications:)));

        let quit_item = NSMenuItem::new(mtm);
        quit_item.setTitle(ns_string!("Quit APDL"));
        quit_item.setAction(Some(objc2::sel!(terminate:)));
        quit_item.setKeyEquivalent(ns_string!("q"));

        app_menu.addItem(&about_item);
        app_menu.addItem(&NSMenuItem::separatorItem(mtm));
        app_menu.addItem(&hide_item);
        app_menu.addItem(&hide_others);
        app_menu.addItem(&show_all);
        app_menu.addItem(&NSMenuItem::separatorItem(mtm));
        app_menu.addItem(&quit_item);

        parent_item.setSubmenu(Some(&app_menu));
        parent_item
    }
}
