#[cfg(target_os = "macos")]
use {
    objc2::{rc::Retained, runtime::AnyObject},
    objc2_app_kit::{NSMenu, NSMenuItem},
    objc2_foundation::{ns_string, MainThreadMarker},
};

#[cfg(target_os = "macos")]
pub fn build(mtm: MainThreadMarker, target: &AnyObject) -> Retained<NSMenuItem> {
    unsafe {
        let parent_item = NSMenuItem::new(mtm);
        parent_item.setTitle(ns_string!("APDL"));

        let app_menu = NSMenu::new(mtm);
        app_menu.setTitle(ns_string!("APDL"));
        app_menu.setAutoenablesItems(false);

        let about_item = NSMenuItem::new(mtm);
        about_item.setTitle(ns_string!("About APDL"));
        about_item.setTarget(Some(target));
        about_item.setAction(Some(objc2::sel!(aboutDialog:)));
        about_item.setEnabled(true);

        let update_item = NSMenuItem::new(mtm);
        update_item.setTitle(ns_string!("Check for Updates..."));
        update_item.setTarget(Some(target));
        update_item.setAction(Some(objc2::sel!(checkUpdates:)));
        update_item.setEnabled(true);

        let hide_item = NSMenuItem::new(mtm);
        hide_item.setTitle(ns_string!("Hide APDL"));
        hide_item.setAction(Some(objc2::sel!(hide:)));
        hide_item.setKeyEquivalent(ns_string!("h"));
        hide_item.setEnabled(true);

        let hide_others = NSMenuItem::new(mtm);
        hide_others.setTitle(ns_string!("Hide Others"));
        hide_others.setAction(Some(objc2::sel!(hideOtherApplications:)));
        hide_others.setKeyEquivalent(ns_string!("h"));
        hide_others.setEnabled(true);

        let show_all = NSMenuItem::new(mtm);
        show_all.setTitle(ns_string!("Show All"));
        show_all.setAction(Some(objc2::sel!(unhideAllApplications:)));
        show_all.setEnabled(true);

        let quit_item = NSMenuItem::new(mtm);
        quit_item.setTitle(ns_string!("Quit APDL"));
        quit_item.setAction(Some(objc2::sel!(terminate:)));
        quit_item.setKeyEquivalent(ns_string!("q"));
        quit_item.setEnabled(true);

        app_menu.addItem(&about_item);
        app_menu.addItem(&update_item);
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
