use crate::AppWindow;
use slint::Weak;

pub fn bind_clipboard_paste(app: &AppWindow, handle: Weak<AppWindow>) {
    app.on_paste_from_clipboard(move || {
        let handle = handle.clone();
        tokio::spawn(async move {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if let Ok(text) = clipboard.get_text() {
                    let trimmed = text.trim().to_string();
                    if trimmed.contains("aparat.com") || trimmed.len() >= 5 {
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = handle.upgrade() {
                                ui.set_modal_url_input(trimmed.into());
                            }
                        });
                    }
                }
            }
        });
    });
}
