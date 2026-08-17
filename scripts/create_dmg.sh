#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
VERSION="${1:-0.1.0}"
DIST_DIR="$ROOT_DIR/dist"
APP_BUNDLE="$DIST_DIR/APDL.app"
DMG_STAGING="$DIST_DIR/dmg_staging"
OUTPUT_DMG="$DIST_DIR/APDL-${VERSION}.dmg"
FINAL_DMG="$DIST_DIR/APDL.dmg"

AUTO_IDENTITY="$(security find-identity -p codesigning -v 2>/dev/null | grep "Developer ID Application" | head -n 1 | awk -F'"' '{print $2}' || true)"
SIGNING_IDENTITY="${SIGNING_IDENTITY:-$AUTO_IDENTITY}"

# Always re-bundle if needed or if signed
echo "Packaging APDL.app bundle..."
"$SCRIPT_DIR/bundle_macos.sh" "$VERSION"

echo "Staging DMG contents..."
rm -rf "$DMG_STAGING" "$OUTPUT_DMG" "$FINAL_DMG"
mkdir -p "$DMG_STAGING"

# Copy App into staging
cp -R "$APP_BUNDLE" "$DMG_STAGING/APDL.app"

# Create /Applications symlink for drag-and-drop installer UX
ln -s /Applications "$DMG_STAGING/Applications"

echo "Creating compressed DMG at $OUTPUT_DMG..."
hdiutil create \
    -volname "APDL" \
    -srcfolder "$DMG_STAGING" \
    -ov \
    -format UDZO \
    "$OUTPUT_DMG"

cp "$OUTPUT_DMG" "$FINAL_DMG"
rm -rf "$DMG_STAGING"

# Sign DMG if identity found
if [[ -n "$SIGNING_IDENTITY" ]]; then
    echo "Signing DMG with identity: $SIGNING_IDENTITY"
    codesign --force --sign "$SIGNING_IDENTITY" "$FINAL_DMG"
fi

echo "APDL DMG created successfully at $FINAL_DMG!"
