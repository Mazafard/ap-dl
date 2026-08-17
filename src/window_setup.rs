#[cfg(target_os = "macos")]
pub fn setup_macos_window() {
    use objc2_app_kit::{
        NSApplication, NSAutoresizingMaskOptions, NSColor, NSVisualEffectBlendingMode,
        NSVisualEffectMaterial, NSVisualEffectState, NSVisualEffectView, NSWindowOrderingMode,
        NSWindowStyleMask, NSWindowTitleVisibility,
    };
    use objc2_foundation::MainThreadMarker;

    if let Some(mtm) = MainThreadMarker::new() {
        let app = NSApplication::sharedApplication(mtm);
        for window in app.windows().iter() {
            window.setTitlebarAppearsTransparent(true);
            window.setTitleVisibility(NSWindowTitleVisibility::NSWindowTitleHidden);
            let mut style = window.styleMask();
            style.insert(NSWindowStyleMask::FullSizeContentView);
            window.setStyleMask(style);
            window.setMovableByWindowBackground(true);
            window.setOpaque(false);

            let clear_color = unsafe { NSColor::clearColor() };
            window.setBackgroundColor(Some(&clear_color));

            if let Some(content_view) = window.contentView() {
                let bounds = content_view.bounds();
                let effect_view = unsafe {
                    let view = NSVisualEffectView::new(mtm);
                    view.setFrame(bounds);
                    view.setMaterial(NSVisualEffectMaterial::HUDWindow);
                    view.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
                    view.setState(NSVisualEffectState::Active);
                    view.setAutoresizingMask(
                        NSAutoresizingMaskOptions::NSViewWidthSizable
                            | NSAutoresizingMaskOptions::NSViewHeightSizable,
                    );
                    view
                };

                unsafe {
                    if let Some(superview) = content_view.superview() {
                        superview.addSubview_positioned_relativeTo(&effect_view, NSWindowOrderingMode::NSWindowBelow, Some(&content_view));
                    } else {
                        content_view.addSubview_positioned_relativeTo(&effect_view, NSWindowOrderingMode::NSWindowBelow, None);
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
pub fn set_macos_dock_icon() {
    use objc2::ClassType;
    use objc2_app_kit::{NSApplication, NSImage};
    use objc2_foundation::{MainThreadMarker, NSData};

    if let Some(mtm) = MainThreadMarker::new() {
        let icon_bytes = include_bytes!("../assets/icon.png");
        let data = NSData::with_bytes(icon_bytes);
        if let Some(image) = NSImage::initWithData(NSImage::alloc(), &data) {
            let app = NSApplication::sharedApplication(mtm);
            unsafe {
                app.setApplicationIconImage(Some(&image));
            }
        }
    }
}
