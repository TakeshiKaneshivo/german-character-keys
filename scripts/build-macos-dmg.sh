#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET_DIR="$ROOT_DIR/src-tauri/target/aarch64-apple-darwin/release"
BUNDLE_DIR="$TARGET_DIR/bundle"
MACOS_DIR="$BUNDLE_DIR/macos"
DMG_DIR="$BUNDLE_DIR/dmg"

cd "$ROOT_DIR"
./node_modules/.bin/tauri build --target aarch64-apple-darwin --bundles app --no-sign

APP_PATH="$(find "$MACOS_DIR" -maxdepth 1 -type d -name '*.app' -print -quit)"
if [[ -z "$APP_PATH" ]]; then
  echo "No macOS application bundle was produced in $MACOS_DIR" >&2
  exit 1
fi

APP_NAME="$(basename "$APP_PATH")"
VERSION="$(node -p "require('./package.json').version")"
DMG_PATH="$DMG_DIR/${APP_NAME%.app}_${VERSION}_aarch64.dmg"
STAGING_DIR="$DMG_DIR/.staging-$$"

cleanup() {
  rm -rf "$STAGING_DIR"
}
trap cleanup EXIT

mkdir -p "$DMG_DIR"
rm -f "$DMG_PATH"
mkdir -p "$STAGING_DIR"
ditto "$APP_PATH" "$STAGING_DIR/$APP_NAME"
ln -s /Applications "$STAGING_DIR/Applications"

hdiutil create \
  -volname "German Character Keys" \
  -srcfolder "$STAGING_DIR" \
  -ov \
  -format UDZO \
  "$DMG_PATH"

echo "DMG created: $DMG_PATH"
