#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$ROOT_DIR/dist"
DMG_PATH="${1:-$DIST_DIR/APDL.dmg}"
KEYCHAIN_PROFILE="${KEYCHAIN_PROFILE:-APDL_NOTARY}"

if [[ ! -f "$DMG_PATH" ]]; then
    echo "Error: DMG not found at $DMG_PATH"
    echo "Usage: ./scripts/notarize_dmg.sh [path/to/APDL.dmg]"
    exit 1
fi

echo "Submitting $DMG_PATH to Apple Notary Service using profile '$KEYCHAIN_PROFILE'..."
if [[ -n "$KEYCHAIN_PROFILE" ]]; then
    xcrun notarytool submit "$DMG_PATH" --keychain-profile "$KEYCHAIN_PROFILE" --wait
elif [[ -n "${APPLE_ID:-}" && -n "${APPLE_PASSWORD:-}" && -n "${TEAM_ID:-}" ]]; then
    xcrun notarytool submit "$DMG_PATH" \
        --apple-id "$APPLE_ID" \
        --password "$APPLE_PASSWORD" \
        --team-id "$TEAM_ID" \
        --wait
else
    echo "Error: Apple credentials not set."
    echo "Either set KEYCHAIN_PROFILE or set APPLE_ID, APPLE_PASSWORD, and TEAM_ID environment variables."
    exit 1
fi

echo "Stapling notarization ticket to $DMG_PATH..."
xcrun stapler staple "$DMG_PATH"

echo "Validating notarized DMG with spctl..."
spctl --assess --type open --context context:primary-signature --verbose "$DMG_PATH"

echo "Notarization and stapling complete! $DMG_PATH is ready for Gatekeeper distribution."
