pub mod checker;
pub mod github;
pub mod types;
pub mod version;

use crate::AppWindow;

pub fn check_for_updates(app_weak: slint::Weak<AppWindow>, is_manual: bool) {
    let current_version = env!("CARGO_PKG_VERSION");
    tokio::spawn(async move {
        let result = checker::check_update(current_version).await;
        checker::apply_update_result(app_weak, result, is_manual);
    });
}
