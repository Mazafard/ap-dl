#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
VERSION="${1:-0.1.0}"
DIST_DIR="$ROOT_DIR/dist"
APP_BUNDLE="$DIST_DIR/APDL.app"
CONTENTS_DIR="$APP_BUNDLE/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

# Auto-detect Developer ID Application certificate if not explicitly set
AUTO_IDENTITY="$(security find-identity -p codesigning -v 2>/dev/null | grep "Developer ID Application" | head -n 1 | awk -F'"' '{print $2}' || true)"
SIGNING_IDENTITY="${SIGNING_IDENTITY:-$AUTO_IDENTITY}"

echo "Building release binary..."
cd "$ROOT_DIR"
cargo build --release

echo "Preparing bundle directory at $APP_BUNDLE..."
rm -rf "$APP_BUNDLE"
mkdir -p "$MACOS_DIR" "$RESOURCES_DIR"

# Copy binary
cp "$ROOT_DIR/target/release/ap-dl" "$MACOS_DIR/ap-dl"
chmod +x "$MACOS_DIR/ap-dl"

# Copy Icon
if [[ -f "$ROOT_DIR/assets/AppIcon.icns" ]]; then
    cp "$ROOT_DIR/assets/AppIcon.icns" "$RESOURCES_DIR/AppIcon.icns"
elif [[ -f "$ROOT_DIR/assets/icon.icns" ]]; then
    cp "$ROOT_DIR/assets/icon.icns" "$RESOURCES_DIR/AppIcon.icns"
fi

# Copy Info.plist and update version if specified
cp "$ROOT_DIR/assets/Info.plist" "$CONTENTS_DIR/Info.plist"
if command -v /usr/libexec/PlistBuddy > /dev/null 2>&1; then
    /usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $VERSION" "$CONTENTS_DIR/Info.plist" 2>/dev/null || true
    /usr/libexec/PlistBuddy -c "Set :CFBundleVersion $VERSION" "$CONTENTS_DIR/Info.plist" 2>/dev/null || true
fi

# Write PkgInfo
echo -n "APPL????" > "$CONTENTS_DIR/PkgInfo"

# Sign bundle with Developer ID or fallback to ad-hoc
if [[ -n "$SIGNING_IDENTITY" ]]; then
    echo "Signing APDL.app with identity: $SIGNING_IDENTITY"
    codesign --force --deep --options runtime --entitlements "$ROOT_DIR/assets/entitlements.plist" --sign "$SIGNING_IDENTITY" "$APP_BUNDLE"
else
    echo "Applying ad-hoc signature for local testing..."
    codesign --force --deep --sign - "$APP_BUNDLE" 2>/dev/null || true
fi

echo "APDL.app successfully created at $APP_BUNDLE!"
