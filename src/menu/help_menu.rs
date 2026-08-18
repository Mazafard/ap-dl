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
        parent_item.setTitle(ns_string!("Help"));

        let help_menu = NSMenu::new(mtm);
        help_menu.setTitle(ns_string!("Help"));
        help_menu.setAutoenablesItems(false);

        let docs_item = NSMenuItem::new(mtm);
        docs_item.setTitle(ns_string!("APDL Documentation"));
        docs_item.setAction(Some(objc2::sel!(openDocs:)));
        docs_item.setTarget(Some(target));
        docs_item.setEnabled(true);

        let repo_item = NSMenuItem::new(mtm);
        repo_item.setTitle(ns_string!("GitHub Repository"));
        repo_item.setAction(Some(objc2::sel!(openRepo:)));
        repo_item.setTarget(Some(target));
        repo_item.setEnabled(true);

        let issues_item = NSMenuItem::new(mtm);
        issues_item.setTitle(ns_string!("Report an Issue"));
        issues_item.setAction(Some(objc2::sel!(openIssues:)));
        issues_item.setTarget(Some(target));
        issues_item.setEnabled(true);

        help_menu.addItem(&docs_item);
        help_menu.addItem(&repo_item);
        help_menu.addItem(&NSMenuItem::separatorItem(mtm));
        help_menu.addItem(&issues_item);

        parent_item.setSubmenu(Some(&help_menu));
        parent_item
    }
}
