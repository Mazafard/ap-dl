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

A fast, lightweight desktop application built with **Rust**, **Slint UI**, and **Tokio** for high-speed video and batch playlist downloading from [Aparat](https://www.aparat.com).

<div align="center">
  <img src="assets/demo.png" alt="APDL Demo Screenshot" width="800" style="border-radius: 12px; box-shadow: 0 16px 32px rgba(0,0,0,0.4);" />
</div>

---

## 📥 Downloads (Latest Release)

| Platform | Architecture | Format | Download Link |
| :--- | :--- | :--- | :--- |
| ![macOS](https://img.shields.io/badge/macOS-000000?logo=apple&logoColor=white) | **Apple Silicon (ARM64)** | `.dmg` | [⬇️ **Download for macOS**](https://github.com/Mazafard/ap-dl/releases/latest/download/ap-dl-macos-arm64.dmg) |
| ![Windows](https://img.shields.io/badge/Windows-0078D6?logo=windows&logoColor=white) | **Windows (x64)** | `.zip` | [⬇️ **Download for Windows**](https://github.com/Mazafard/ap-dl/releases/latest/download/ap-dl-windows-x64.zip) |
| ![Linux](https://img.shields.io/badge/Linux-FCC624?logo=linux&logoColor=black) | **Linux (x86_64)** | `.tar.gz` | [⬇️ **Download for Linux**](https://github.com/Mazafard/ap-dl/releases/latest/download/ap-dl-linux-x64.tar.gz) |

> 💡 *Download links automatically point to the latest stable release.*

---

> [!IMPORTANT]
> ### ⚠️ Educational & Non-Commercial Disclaimer
> This project was developed strictly for learning and educational purposes (Rust async patterns, Tokio runtime, HTTP range streaming, and Slint GUI). It is not affiliated with, endorsed by, or associated with Aparat or Saba Idea.

---

## 🚀 Key Features

- ⚡ **Multi-Segment Turbo Downloader**: Parallel chunk streaming via HTTP `Range` requests for maximum bandwidth saturation.
- 🔁 **Resumable Downloads**: Automatically recovers interrupted or paused downloads from the exact byte without restarting.
- 📋 **Batch Playlist Extraction**: Resolves entire playlist URLs into queued batch downloads with one click.
- 🎯 **Quality Selector**: Full resolution selection (1080p, 720p, 480p, 360p, 240p, 144p) with live metadata preview.
- 🌐 **Intelligent CDN Failover**: Auto-routes through alternative CDN edge mirrors if an endpoint experiences throttling or timeouts.
- 🔄 **In-App Self-Updater**: Background update checking and atomic in-place binary upgrades (`self-replace`) without manual reinstalling.
- 🖥️ **Native Cross-Platform GUI**: High-performance GPU-rendered desktop interface built with Slint for macOS, Windows, and Linux.

---

## 🛠 Makefile Automation

| Command | Description |
| :--- | :--- |
| **`make watch`** | Run development mode with hot-reloading |
| **`make run`** | Run local release build (`cargo run --release`) |
| **`make build`** | Build optimized release binary |
| **`make app`** | Package macOS native `.app` bundle in `dist/` |
| **`make dmg`** | Create macOS `.dmg` installer |
| **`make test`** | Run full test suite |
| **`make release VERSION=0.3.1`** | Bump version, tag, and trigger multi-platform CI/CD release |

---

## 👥 Author

Developed with ❤️ by **Mohammadreza A. Fard** ([@Mazafard](https://github.com/Mazafard))  
- **Issues**: [https://github.com/Mazafard/ap-dl/issues](https://github.com/Mazafard/ap-dl/issues)
- **Repository**: [https://github.com/Mazafard/ap-dl](https://github.com/Mazafard/ap-dl)
