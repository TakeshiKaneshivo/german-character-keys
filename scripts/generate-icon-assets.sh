#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ICONS_DIR="$ROOT_DIR/src-tauri/icons"
PUBLIC_IMAGES_DIR="$ROOT_DIR/public/images"
NSIS_DIR="$ROOT_DIR/src-tauri/nsis"
DISABLED_SOURCE="$ICONS_DIR/source/icon-disabled-source.png"
ENABLED_SOURCE="$ICONS_DIR/source/icon-enabled-source.png"
SWIFT_CACHE_DIR="${TMPDIR:-/tmp}/german-character-keys-swift-cache"
SWIFT_BINARY="${TMPDIR:-/tmp}/generate-german-icons"
TAURI_ICON_DIR="${TMPDIR:-/tmp}/german-character-keys-tauri-icons"

swiftc -module-cache-path "$SWIFT_CACHE_DIR" "$ROOT_DIR/scripts/generate-icons.swift" -o "$SWIFT_BINARY"
"$SWIFT_BINARY" "$DISABLED_SOURCE" "$ENABLED_SOURCE" "$ICONS_DIR" "$PUBLIC_IMAGES_DIR" "$NSIS_DIR"

rm -rf "$TAURI_ICON_DIR"
npm exec -- tauri icon "$ICONS_DIR/icon.png" --output "$TAURI_ICON_DIR"

cp "$TAURI_ICON_DIR/32x32.png" "$ICONS_DIR/32x32.png"
cp "$TAURI_ICON_DIR/128x128.png" "$ICONS_DIR/128x128.png"
cp "$TAURI_ICON_DIR/128x128@2x.png" "$ICONS_DIR/128x128@2x.png"
cp "$TAURI_ICON_DIR/icon.icns" "$ICONS_DIR/icon.icns"
cp "$TAURI_ICON_DIR/icon.ico" "$ICONS_DIR/icon.ico"

echo "Generated all icon assets from $DISABLED_SOURCE and $ENABLED_SOURCE"
