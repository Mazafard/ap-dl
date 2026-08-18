fn main() {
    slint_build::compile("ui/appwindow.slint").expect("Slint build failed");

    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "APDL");
        res.set("FileDescription", "Aparat Video & Playlist Downloader");
        res.set("LegalCopyright", "Copyright (c) Mohammadreza A. Fard");
        let _ = res.compile();
    }
}
