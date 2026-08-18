# APDL — Aparat Downloader (`ap-dl`)

<div align="center">

[![Release](https://img.shields.io/github/v/release/Mazafard/ap-dl?color=f43f5e&logo=github&label=Release)](https://github.com/Mazafard/ap-dl/releases)
[![Rust](https://img.shields.io/badge/Rust-1.80+_(2021)-orange.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Slint UI](https://img.shields.io/badge/GUI-Slint_1.9-00bcd4.svg?logo=slint&logoColor=white)](https://slint.dev/)
[![Async Runtime](https://img.shields.io/badge/Async-Tokio-blueviolet.svg?logo=tokio&logoColor=white)](https://tokio.rs/)
[![CI](https://github.com/Mazafard/ap-dl/actions/workflows/ci.yml/badge.svg)](https://github.com/Mazafard/ap-dl/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/Mazafard/ap-dl/branch/main/graph/badge.svg)](https://codecov.io/gh/Mazafard/ap-dl)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

</div>

A fast, async desktop application built in **Rust** using **Slint UI** and **Tokio** to download videos and playlists from [Aparat](https://www.aparat.com) with quality selection, multi-segment downloading, and real-time progress indicators.

<div align="center">
  <img src="assets/demo.png" alt="APDL Demo Screenshot" width="800" style="border-radius: 12px; box-shadow: 0 16px 32px rgba(0,0,0,0.4);" />
</div>

---

> [!IMPORTANT]
> ### ⚠️ Educational & Non-Commercial Disclaimer
> This project was created **strictly for learning and educational purposes** to explore Rust language fundamentals, Tokio asynchronous runtime, HTTP streaming, and Slint GUI development.
> 
> - This software is **NOT intended for commercial use**.
> - It is not affiliated with, endorsed by, or associated with Aparat or Saba Idea.
> - Users are solely responsible for ensuring their usage complies with all applicable copyright laws and the terms of service of the content provider.

---

## ✨ Features

- **Single Video Downloader**: Paste any Aparat video link (e.g., `https://www.aparat.com/v/XXXXX`), inspect metadata (title, duration, channel), and pick your desired quality (1080p, 720p, 480p, 360p, 240p, 144p).
- **Playlist Extractor**: Paste a playlist link (e.g., `https://www.aparat.com/playlist/XXXXX`) to automatically load all videos into the batch queue.
- **Resumable Chunk Streaming**: Uses HTTP `Range` requests to automatically resume dropped or interrupted downloads from where they left off.
- **Intelligent CDN Failover**: Seamlessly fails over across high-availability CDN edge mirrors if a local node times out.
- **In-App Auto Update Checker**: Background and on-demand checking against the GitHub Releases API with SemVer comparisons and release note previews.
- **Frosted Glass Startup Experience**: Smooth startup splash screen with glowing APDL branding, progressive loader track, and initialization status.
- **Custom "About APDL" Modal**: Beautiful frosted glass dialog showing author credits, project links (GitHub, Bug Tracker, Docs), and legal disclaimer.
- **Real-Time Progress Tracking**: Live speed (MB/s), downloaded bytes, and percentage progress bars.
- **Native macOS Menu Bar**: Full system integration with custom keyboard shortcuts (`⌘+N`, `⌘+O`, `⌘+H`, `⌘+Q`).

---

## 🛠 Makefile Automation Commands

A comprehensive [`Makefile`](file:///Volumes/CrucialX9/ap-dl/Makefile) is provided for easy development, testing, packaging, and CI/CD releases.

| Command | Description |
| :--- | :--- |
| **`make watch`** | Run development mode with hot-reloading (`cargo-watch` watching `src/` & `ui/`) |
| **`make run`** | Run the application locally in release mode (`cargo run --release`) |
| **`make build`** | Build the optimized local release binary |
| **`make app`** | Package the macOS native **`APDL.app`** bundle with high-res icon in `dist/` |
| **`make dmg`** | Build signed and notarized **`APDL.dmg`** disk image |
| **`make test`** | Run all unit tests and live Aparat API integration tests |
| **`make check`** | Fast compiler syntax and type checking |
| **`make tag VERSION=0.2.0`** | Create and push a git release tag (`v0.2.0`) to trigger GitHub Actions |
| **`make release VERSION=0.2.0`** | Bump version in `Cargo.toml`, commit, tag, and trigger multi-platform CI/CD |
| **`make clean`** | Clean cargo build artifacts and temporary download files |

---

## 🚀 Getting Started

### 1. Prerequisites
Make sure [Rust & Cargo](https://rustup.rs) are installed.

### 2. Running in Development
```bash
make watch
```

### 3. Running as a Native macOS App
```bash
make app
open dist/APDL.app
```

---

## 👥 Author & Maintainer

Developed with ❤️ by **Mohammadreza A. Fard** ([@Mazafard](https://github.com/Mazafard))  
- **Issues & Bug Reports**: [https://github.com/Mazafard/ap-dl/issues](https://github.com/Mazafard/ap-dl/issues)
- **Repository**: [https://github.com/Mazafard/ap-dl](https://github.com/Mazafard/ap-dl)
