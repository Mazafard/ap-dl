# Changelog

All notable changes to **APDL** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.3.0] - 2026-08-18

### Added
- **In-App Self-Updater & Binary Replacer**: One-click in-app update downloading, archive extraction, atomic replacement (`self-replace`), and automatic application relaunch.
- **In-Window Menubar for Windows & Linux**: Sleek frosted glass top menubar with `File`, `Edit`, `View`, and `Help` dropdown menus (automatically hidden on macOS).
- **Windows Platform Polish**: Suppressed background console terminal (`windows_subsystem = "windows"`) and embedded native high-resolution taskbar icon (`icon.ico`).
- **Linux Desktop Integration**: Added standard freedesktop `ap-dl.desktop` entry (`Terminal=false`) and automated `install_linux.sh` installer script.
- **Centralized Singleton AppInfo**: Dynamic runtime version injection across UI components with zero duplication.
- **In-App "What's New" Changelog Viewer**: Interactive frosted glass dialog showcasing version release highlights.

---

## [0.2.0] - 2026-08-18

### Added
- **GitHub Release Update Checker**: Background and manual update notifications with SemVer comparison and release notes preview.
- **Frosted Glass Startup Loader**: Progressive 100% startup track with glowing APDL branding.
- **Custom About Dialog**: Frosted glass modal with author credits (*Mohammadreza A. Fard*), repository links, and disclaimer.
- **Permanent macOS Menu Bar**: In-place native `NSMenu` mutator with enabled actions and shortcut triggers.
- **Vector Icons & UI Polish**: Crisp vector `CloseIcon` replacing missing glyph boxes, and smart auto-toggling paste/clear inputs.

---

## [0.1.0] - 2026-08-17

### Added
- **Single Video Downloader**: Video resolution picker (1080p, 720p, 480p, 360p, 240p, 144p) with metadata inspection.
- **Playlist Batch Extractor**: Automatic playlist expansion into batch queue.
- **Turbo Multi-Segment Downloader**: Parallel chunk streaming with HTTP `Range` requests and auto-resume.
- **Intelligent CDN Failover**: Edge mirror routing across Caspian nodes.
- **Real-Time Speed & Progress Tracking**: Download speed (MB/s) and live progress bars.
