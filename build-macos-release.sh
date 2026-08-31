#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "Error: this script must be run on macOS." >&2
  exit 1
fi

if [[ "$(uname -m)" != "arm64" ]]; then
  echo "Error: this script requires an Apple Silicon Mac (arm64)." >&2
  exit 1
fi

for command_name in node npm cargo; do
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "Error: $command_name is required but was not found in PATH." >&2
    exit 1
  fi
done

if [[ ! -x "node_modules/.bin/tauri" ]]; then
  echo "Installing JavaScript dependencies..."
  npm ci
fi

npm run build:macos-release

APP_PATH="src-tauri/target/aarch64-apple-darwin/release/german-key-assist"
DMG_PATH="src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/German Key Assist_0.1.0_aarch64.dmg"

if [[ ! -f "$APP_PATH" ]]; then
  echo "Error: Release application was not generated: $APP_PATH" >&2
  exit 1
fi
if [[ ! -f "$DMG_PATH" ]]; then
  echo "Error: DMG installer was not generated: $DMG_PATH" >&2
  exit 1
fi

echo
echo "macOS Apple Silicon Release build completed."
echo "Installer: $DMG_PATH"
echo "App:       $APP_PATH"
