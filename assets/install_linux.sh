#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="${HOME}/.local/bin"
APP_DIR="${HOME}/.local/share/applications"
ICON_DIR="${HOME}/.local/share/icons/hicolor/256x256/apps"

mkdir -p "$BIN_DIR" "$APP_DIR" "$ICON_DIR"

if [[ -f "${SCRIPT_DIR}/ap-dl" ]]; then
    cp "${SCRIPT_DIR}/ap-dl" "${BIN_DIR}/ap-dl"
    chmod +x "${BIN_DIR}/ap-dl"
fi

if [[ -f "${SCRIPT_DIR}/assets/icon.png" ]]; then
    cp "${SCRIPT_DIR}/assets/icon.png" "${ICON_DIR}/ap-dl.png"
elif [[ -f "${SCRIPT_DIR}/icon.png" ]]; then
    cp "${SCRIPT_DIR}/icon.png" "${ICON_DIR}/ap-dl.png"
fi

if [[ -f "${SCRIPT_DIR}/assets/ap-dl.desktop" ]]; then
    cp "${SCRIPT_DIR}/assets/ap-dl.desktop" "${APP_DIR}/ap-dl.desktop"
elif [[ -f "${SCRIPT_DIR}/ap-dl.desktop" ]]; then
    cp "${SCRIPT_DIR}/ap-dl.desktop" "${APP_DIR}/ap-dl.desktop"
fi

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "${APP_DIR}" 2>/dev/null || true
fi

echo "APDL installed successfully to ~/.local/bin/ap-dl and added to Linux desktop apps!"
