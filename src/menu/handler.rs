#[cfg(target_os = "macos")]
use {
    crate::{state::AppState, AppWindow},
    objc2::{declare_class, msg_send_id, mutability, rc::Retained, runtime::NSObject, ClassType, DeclaredClass},
    objc2_foundation::MainThreadMarker,
    slint::Weak,
    std::sync::{atomic::{AtomicPtr, Ordering}, Arc},
};

#[cfg(target_os = "macos")]
static APP_HANDLE: AtomicPtr<Weak<AppWindow>> = AtomicPtr::new(std::ptr::null_mut());
#[cfg(target_os = "macos")]
static APP_STATE: AtomicPtr<Arc<AppState>> = AtomicPtr::new(std::ptr::null_mut());
#[cfg(target_os = "macos")]
static TARGET_STORAGE: AtomicPtr<ApdlMenuTarget> = AtomicPtr::new(std::ptr::null_mut());

#[cfg(target_os = "macos")]
declare_class!(
    pub struct ApdlMenuTarget;
    unsafe impl ClassType for ApdlMenuTarget {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "ApdlMenuTarget";
    }
    impl DeclaredClass for ApdlMenuTarget {}

    unsafe impl ApdlMenuTarget {
        #[method(aboutDialog:)]
        fn about_dialog(&self, _sender: *mut NSObject) {
            let ptr = APP_HANDLE.load(Ordering::SeqCst);
            if !ptr.is_null() {
                let weak_handle = unsafe { &*ptr };
                let handle_c = weak_handle.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = handle_c.upgrade() { ui.set_show_about_dialog(true); }
                });
            }
        }

        #[method(addLink:)]
        fn add_link(&self, _sender: *mut NSObject) {
            let ptr = APP_HANDLE.load(Ordering::SeqCst);
            if !ptr.is_null() {
                let weak_handle = unsafe { &*ptr };
                let handle_c = weak_handle.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = handle_c.upgrade() {
                        ui.set_validation_message("".into());
                        ui.set_show_url_modal(true);
                    }
                });
            }
        }

        #[method(openFolder:)]
        fn open_folder(&self, _sender: *mut NSObject) {
            let state_ptr = APP_STATE.load(Ordering::SeqCst);
            if !state_ptr.is_null() {
                let state = unsafe { &*state_ptr };
                let state_c = state.clone();
                tokio::spawn(async move {
                    let dir = { state_c.download_dir.read().await.clone() };
                    let _ = open::that(&dir);
                });
            }
        }

        #[method(checkUpdates:)]
        fn check_updates(&self, _sender: *mut NSObject) {
            let ptr = APP_HANDLE.load(Ordering::SeqCst);
            if !ptr.is_null() {
                let weak_handle = unsafe { &*ptr };
                crate::updater::check_for_updates(weak_handle.clone(), true);
            }
        }

        #[method(openDocs:)]
        fn open_docs(&self, _sender: *mut NSObject) { let _ = open::that("https://github.com/Mazafard/ap-dl#readme"); }
        #[method(openRepo:)]
        fn open_repo(&self, _sender: *mut NSObject) { let _ = open::that("https://github.com/Mazafard/ap-dl"); }
        #[method(openIssues:)]
        fn open_issues(&self, _sender: *mut NSObject) { let _ = open::that("https://github.com/Mazafard/ap-dl/issues"); }
    }
);

#[cfg(target_os = "macos")]
pub fn create_target(_mtm: MainThreadMarker, handle: Weak<AppWindow>, state: Arc<AppState>) -> Retained<ApdlMenuTarget> {
    let boxed_handle = Box::into_raw(Box::new(handle));
    APP_HANDLE.store(boxed_handle, Ordering::SeqCst);
    let boxed_state = Box::into_raw(Box::new(state));
    APP_STATE.store(boxed_state, Ordering::SeqCst);

    let target: Retained<ApdlMenuTarget> = unsafe { msg_send_id![ApdlMenuTarget::alloc(), init] };
    let raw = Retained::into_raw(target.clone());
    let old = TARGET_STORAGE.swap(raw as *mut _, Ordering::SeqCst);
    if !old.is_null() { unsafe { let _ = Retained::from_raw(old as *mut ApdlMenuTarget); } }
    target
}
