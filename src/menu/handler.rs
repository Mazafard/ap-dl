#[cfg(target_os = "macos")]
use crate::state::AppState;
#[cfg(target_os = "macos")]
use crate::AppWindow;
#[cfg(target_os = "macos")]
use objc2::rc::Retained;
#[cfg(target_os = "macos")]
use objc2::runtime::NSObject;
#[cfg(target_os = "macos")]
use objc2::{declare_class, msg_send_id, mutability, ClassType, DeclaredClass};
#[cfg(target_os = "macos")]
use objc2_foundation::MainThreadMarker;
#[cfg(target_os = "macos")]
use slint::Weak;
#[cfg(target_os = "macos")]
use std::sync::atomic::{AtomicPtr, Ordering};
#[cfg(target_os = "macos")]
use std::sync::Arc;

#[cfg(target_os = "macos")]
static APP_HANDLE: AtomicPtr<Weak<AppWindow>> = AtomicPtr::new(std::ptr::null_mut());
#[cfg(target_os = "macos")]
static APP_STATE: AtomicPtr<Arc<AppState>> = AtomicPtr::new(std::ptr::null_mut());

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

        #[method(openDocs:)]
        fn open_docs(&self, _sender: *mut NSObject) {
            let _ = open::that("https://github.com/ap-dl/ap-dl#readme");
        }

        #[method(openRepo:)]
        fn open_repo(&self, _sender: *mut NSObject) {
            let _ = open::that("https://github.com/ap-dl/ap-dl");
        }

        #[method(openIssues:)]
        fn open_issues(&self, _sender: *mut NSObject) {
            let _ = open::that("https://github.com/ap-dl/ap-dl/issues");
        }
    }
);

#[cfg(target_os = "macos")]
pub fn create_target(
    _mtm: MainThreadMarker,
    handle: Weak<AppWindow>,
    state: Arc<AppState>,
) -> Retained<ApdlMenuTarget> {
    let boxed_handle = Box::into_raw(Box::new(handle));
    APP_HANDLE.store(boxed_handle, Ordering::SeqCst);

    let boxed_state = Box::into_raw(Box::new(state));
    APP_STATE.store(boxed_state, Ordering::SeqCst);

    unsafe { msg_send_id![ApdlMenuTarget::alloc(), init] }
}
