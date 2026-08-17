# APDL — Aparat Downloader (`ap-dl`)

A fast, async desktop application built in **Rust** using **Slint UI** and **Tokio** to download videos and playlists from [Aparat](https://www.aparat.com) with quality selection and real-time progress indicators.

---

> [!IMPORTANT]
> ### ⚠️ Educational & Non-Commercial Disclaimer
> This project was created **strictly for learning and educational purposes** to explore Rust language fundamentals, Tokio asynchronous runtime, HTTP streaming, and Slint GUI development.
> 
> - This software is **NOT intended for commercial use**.
> - It is not affiliated with, endorsed by, or associated with Aparat or Saba Idea.
> - Users are solely responsible for ensuring their usage complies with all applicable copyright laws and the terms of service of the content provider.

---

## Features

- **Single Video Downloader**: Paste any Aparat video link (e.g., `https://www.aparat.com/v/XXXXX`), inspect metadata (title, duration, channel), and pick your desired quality (1080p, 720p, 480p, 360p, 240p, 144p).
- **Playlist Extractor**: Paste a playlist link (e.g., `https://www.aparat.com/playlist/XXXXX`) to automatically load all videos into the batch queue.
- **Resumable Chunk Streaming**: Uses HTTP `Range` requests to automatically resume dropped or interrupted downloads from where they left off.
- **Intelligent CDN Failover**: Seamlessly fails over across high-availability CDN edge mirrors if a local node times out.
- **Real-Time Progress Tracking**: Live speed (MB/s), downloaded bytes, and percentage progress bars.
- **Directory Picker**: Easily choose or change the output download destination folder (`rfd` native file picker).
- **Concurrent & Non-Blocking**: Built on Tokio async tasks with smooth Slint UI event loop synchronization.
- **Cancel & Batch Management**: Start individual downloads, batch download all, cancel running streams, or clear completed items.

---

## Project Structure

```text
ap-dl/
├── Cargo.toml               # Project dependencies and configuration
├── Makefile                 # Build, test, packaging & release automation
├── build.rs                 # Slint UI compiler build script
├── assets/                  # App icon assets (PNG, ICNS, SVG)
├── .github/workflows/       # Multi-platform CI/CD release workflow
│   └── release.yml
├── ui/
│   └── appwindow.slint      # Modern Slint UI layout and components
└── src/
    ├── main.rs              # Tokio runtime, Slint callbacks, and state handling
    ├── aparat/
    │   ├── mod.rs
    │   ├── api.rs           # Aparat API client (Video & Playlist resolver)
    │   └── models.rs        # Data structures for Aparat JSON responses
    └── downloader/
        ├── mod.rs
        └── worker.rs        # Auto-resumable async downloader with CDN failover
```

---

## Makefile Automation Commands

A comprehensive [`Makefile`](file:///Volumes/CrucialX9/ap-dl/Makefile) is provided for easy development, testing, packaging, and CI/CD releases.

| Command | Description |
| :--- | :--- |
| **`make watch`** | Run development mode with hot-reloading (`cargo-watch` watching `src/` & `ui/`) |
| **`make run`** | Run the application locally in release mode (`cargo run --release`) |
| **`make build`** | Build the optimized local release binary |
| **`make app`** | Package the macOS native **`APDL.app`** bundle with high-res icon in `dist/` |
| **`make test`** | Run all unit tests and live Aparat API integration tests |
| **`make check`** | Fast compiler syntax and type checking |
| **`make tag VERSION=0.0.1`** | Create and push a git release tag (`v0.0.1`) to trigger GitHub Actions |
| **`make release VERSION=0.0.1`** | Bump version in `Cargo.toml`, commit, tag, and trigger multi-platform CI/CD |
| **`make clean`** | Clean cargo build artifacts and temporary download files |
| **`make help`** | Display all available Makefile commands |

---

## Getting Started

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

## CI/CD Multi-Platform Releases

When you push a version tag (e.g. via `make release VERSION=0.0.1`), GitHub Actions automatically builds release archives for all major platforms:

- **Windows** (`x86_64`): `ap-dl-v0.0.1-windows-x86_64.zip`
- **Linux** (`x86_64`): `ap-dl-v0.0.1-linux-x86_64.tar.gz`
- **macOS Apple Silicon** (`ARM64`): `ap-dl-v0.0.1-macos-arm64.zip` (containing `APDL.app`)
- **macOS Intel** (`x86_64`): `ap-dl-v0.0.1-macos-x86_64.zip` (containing `APDL.app`)

---

## Usage Guide

1. **Set Destination**: Click **Browse...** to pick a destination folder, or use the default system Downloads folder (`~/Downloads`).
2. **Fetch Metadata**: Paste an Aparat video or playlist URL into the input field and click **Fetch Info** (or press Enter).
3. **Select Quality**: For single videos, select the desired resolution from the dropdown menu and click **⬇ Download**.
4. **Download**: Click **Download** on individual queue items or click **Download All** to download all queued videos in batch.

---

## ⚖️ Legal Disclaimer & Terms

> [!IMPORTANT]
> **Please read this disclaimer carefully before using this software:**
>
> 1. **Educational & Archival Purpose**: This project is developed solely for educational, research, and personal media backup/archival purposes to demonstrate modern asynchronous GUI programming in Rust with Slint.
> 2. **No DRM Circumvention**: This software only accesses publicly exposed media endpoints and direct progressive download streams provided by the host. It does **not** bypass or crack any digital rights management (DRM), access control mechanisms, or encryption technologies.
> 3. **Copyright & Content Rights**: All videos, audio, titles, and media downloaded using this tool remain the exclusive intellectual property of their respective copyright holders and content creators. The developers of this tool do not host, store, or distribute any copyrighted media.
> 4. **User Responsibility**: Users of this software are solely and fully responsible for ensuring that their downloading activities comply with applicable national and international copyright laws, intellectual property regulations, and the terms of service of the third-party platforms.
> 5. **Trademark Notice**: "Aparat" and associated logos/branding are trademarks or registered trademarks of Saba Idea Co. This project is an independent open-source tool and is neither affiliated with, maintained by, nor officially endorsed by Saba Idea or Aparat.
> 6. **Limitation of Liability**: This software is provided "as is", without warranty of any kind, express or implied. In no event shall the authors or copyright holders be liable for any claim, damages, or other liability arising from the use of this software.

---

## 📄 License

This project is open-source and licensed under the [MIT License](LICENSE).

