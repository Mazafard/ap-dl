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
        parent_item.setTitle(ns_string!("Edit"));

        let edit_menu = NSMenu::new(mtm);
        edit_menu.setTitle(ns_string!("Edit"));

        let undo = NSMenuItem::new(mtm);
        undo.setTitle(ns_string!("Undo"));
        undo.setAction(Some(objc2::sel!(undo:)));
        undo.setKeyEquivalent(ns_string!("z"));

        let redo = NSMenuItem::new(mtm);
        redo.setTitle(ns_string!("Redo"));
        redo.setAction(Some(objc2::sel!(redo:)));
        redo.setKeyEquivalent(ns_string!("Z"));

        let cut = NSMenuItem::new(mtm);
        cut.setTitle(ns_string!("Cut"));
        cut.setAction(Some(objc2::sel!(cut:)));
        cut.setKeyEquivalent(ns_string!("x"));

        let copy = NSMenuItem::new(mtm);
        copy.setTitle(ns_string!("Copy"));
        copy.setAction(Some(objc2::sel!(copy:)));
        copy.setKeyEquivalent(ns_string!("c"));

        let paste = NSMenuItem::new(mtm);
        paste.setTitle(ns_string!("Paste"));
        paste.setAction(Some(objc2::sel!(paste:)));
        paste.setKeyEquivalent(ns_string!("v"));

        let select_all = NSMenuItem::new(mtm);
        select_all.setTitle(ns_string!("Select All"));
        select_all.setAction(Some(objc2::sel!(selectAll:)));
        select_all.setKeyEquivalent(ns_string!("a"));

        edit_menu.addItem(&undo);
        edit_menu.addItem(&redo);
        edit_menu.addItem(&NSMenuItem::separatorItem(mtm));
        edit_menu.addItem(&cut);
        edit_menu.addItem(&copy);
        edit_menu.addItem(&paste);
        edit_menu.addItem(&select_all);

        parent_item.setSubmenu(Some(&edit_menu));
        parent_item
    }
}
