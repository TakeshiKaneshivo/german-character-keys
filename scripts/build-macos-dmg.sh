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
VOLUME_NAME="German Character Keys"
DMG_PATH="$DMG_DIR/German Character Keys for US Keyboards (ÄÖÜß)_${VERSION}_aarch64.dmg"
STAGING_DIR="$DMG_DIR/.staging-$$"
RW_DMG_PATH="$DMG_DIR/.German Character Keys_${VERSION}_aarch64-rw-$$.dmg"
HYBRID_DMG_PATH="$DMG_DIR/.German Character Keys_${VERSION}_aarch64-hybrid-$$.dmg"
APPLESCRIPT_PATH="$DMG_DIR/.dmg-layout-$$.applescript"
BACKGROUND_SCRIPT_PATH="$DMG_DIR/.dmg-background-$$.swift"
BACKGROUND_PATH="$STAGING_DIR/$APP_NAME/Contents/Resources/dmg-background.png"
SWIFT_MODULE_CACHE_DIR="$DMG_DIR/.swift-module-cache-$$"
DEV_NAME=""
MOUNT_DIR=""

cleanup() {
  if [[ -n "$DEV_NAME" ]]; then
    hdiutil detach "$DEV_NAME" >/dev/null 2>&1 || true
  elif [[ -n "$MOUNT_DIR" ]]; then
    hdiutil detach "$MOUNT_DIR" >/dev/null 2>&1 || true
  fi
  rm -rf "$STAGING_DIR"
  rm -rf "$SWIFT_MODULE_CACHE_DIR"
  rm -f "$RW_DMG_PATH" "$HYBRID_DMG_PATH" "$APPLESCRIPT_PATH" "$BACKGROUND_SCRIPT_PATH"
}
trap cleanup EXIT

mkdir -p "$DMG_DIR"
rm -f "$DMG_PATH"
mkdir -p "$STAGING_DIR"
ditto "$APP_PATH" "$STAGING_DIR/$APP_NAME"
ln -s /Applications "$STAGING_DIR/Applications"
mkdir -p "$(dirname "$BACKGROUND_PATH")"

cat >"$BACKGROUND_SCRIPT_PATH" <<'SWIFT'
import AppKit

let outputPath = CommandLine.arguments[1]
let width: CGFloat = 820
let height: CGFloat = 470

guard let bitmap = NSBitmapImageRep(
  bitmapDataPlanes: nil,
  pixelsWide: Int(width),
  pixelsHigh: Int(height),
  bitsPerSample: 8,
  samplesPerPixel: 4,
  hasAlpha: true,
  isPlanar: false,
  colorSpaceName: .deviceRGB,
  bytesPerRow: 0,
  bitsPerPixel: 0
) else {
  fputs("Unable to create DMG background bitmap.\n", stderr)
  exit(1)
}

bitmap.size = NSSize(width: width, height: height)
NSGraphicsContext.saveGraphicsState()
NSGraphicsContext.current = NSGraphicsContext(bitmapImageRep: bitmap)
NSGraphicsContext.current?.shouldAntialias = true

NSColor.white.setFill()
NSBezierPath(rect: NSRect(x: 0, y: 0, width: width, height: height)).fill()

let title = "Drag German Character Keys.app to Applications to install"
let paragraph = NSMutableParagraphStyle()
paragraph.alignment = .center
let attributes: [NSAttributedString.Key: Any] = [
  .font: NSFont.systemFont(ofSize: 22, weight: .semibold),
  .foregroundColor: NSColor(calibratedWhite: 0.14, alpha: 1.0),
  .paragraphStyle: paragraph
]
let titleRect = NSRect(x: 40, y: height - 74, width: width - 80, height: 34)
title.draw(in: titleRect, withAttributes: attributes)

let hint = "Move the app icon onto the Applications folder."
let hintAttributes: [NSAttributedString.Key: Any] = [
  .font: NSFont.systemFont(ofSize: 14, weight: .regular),
  .foregroundColor: NSColor(calibratedWhite: 0.38, alpha: 1.0),
  .paragraphStyle: paragraph
]
let hintRect = NSRect(x: 40, y: height - 102, width: width - 80, height: 22)
hint.draw(in: hintRect, withAttributes: hintAttributes)
NSGraphicsContext.restoreGraphicsState()

guard let png = bitmap.representation(using: .png, properties: [:]) else {
  fputs("Unable to render DMG background image.\n", stderr)
  exit(1)
}

try png.write(to: URL(fileURLWithPath: outputPath))
SWIFT

mkdir -p "$SWIFT_MODULE_CACHE_DIR"
/usr/bin/swift \
  -module-cache-path "$SWIFT_MODULE_CACHE_DIR" \
  "$BACKGROUND_SCRIPT_PATH" \
  "$BACKGROUND_PATH"

if ! hdiutil create \
  -volname "$VOLUME_NAME" \
  -srcfolder "$STAGING_DIR" \
  -ov \
  -fs HFS+ \
  -format UDRW \
  "$RW_DMG_PATH"; then
  # Some restricted macOS environments cannot allocate a writable device for
  # `hdiutil create`. Build a hybrid image first, then convert it back to a
  # writable image so Finder metadata and open-folder flags can still be added.
  rm -f "$RW_DMG_PATH" "$HYBRID_DMG_PATH"
  hdiutil makehybrid \
    -default-volume-name "$VOLUME_NAME" \
    -hfs \
    -o "$HYBRID_DMG_PATH" \
    "$STAGING_DIR"
  hdiutil convert \
    -format UDRW \
    -ov \
    -o "$RW_DMG_PATH" \
    "$HYBRID_DMG_PATH"
fi

ATTACH_OUTPUT="$(hdiutil attach -readwrite -noverify -noautoopen "$RW_DMG_PATH")"
DEV_NAME="$(printf '%s\n' "$ATTACH_OUTPUT" | awk '/^\/dev\// { print $1; exit }')"
MOUNT_DIR="$(printf '%s\n' "$ATTACH_OUTPUT" | sed -n 's#^/dev/[^[:space:]]*[[:space:]].*[[:space:]]\(/Volumes/.*\)$#\1#p' | head -n 1)"

if [[ -z "$DEV_NAME" || -z "$MOUNT_DIR" || ! -d "$MOUNT_DIR" ]]; then
  echo "Unable to mount writable DMG for Finder metadata." >&2
  echo "$ATTACH_OUTPUT" >&2
  exit 1
fi

cat >"$APPLESCRIPT_PATH" <<'APPLESCRIPT'
on run argv
  set volumeName to item 1 of argv
  set appName to item 2 of argv
  set backgroundPath to item 3 of argv
  tell application "Finder"
    tell disk volumeName
      open
      tell container window
        set current view to icon view
        set toolbar visible to false
        set statusbar visible to false
        set the bounds to {120, 120, 940, 590}
      end tell

      set opts to the icon view options of container window
      tell opts
        set icon size to 96
        set text size to 14
        set arrangement to not arranged
        set background picture to (POSIX file backgroundPath as alias)
      end tell

      set position of item appName to {230, 235}
      set position of item "Applications" to {530, 235}
      close
      open
      delay 2
    end tell
  end tell
end run
APPLESCRIPT

/usr/bin/osascript "$APPLESCRIPT_PATH" "$VOLUME_NAME" "$APP_NAME" "$MOUNT_DIR/$APP_NAME/Contents/Resources/dmg-background.png"

for _ in {1..10}; do
  [[ -f "$MOUNT_DIR/.DS_Store" ]] && break
  sleep 1
done

if [[ ! -f "$MOUNT_DIR/.DS_Store" ]]; then
  echo "Finder did not write .DS_Store for the DMG window layout." >&2
  exit 1
fi

chflags hidden "$MOUNT_DIR/.DS_Store"
rm -rf "$MOUNT_DIR/.fseventsd" "$MOUNT_DIR/.Trashes"

if ! bless --folder "$MOUNT_DIR" --openfolder "$MOUNT_DIR" >/dev/null 2>&1; then
  bless --folder "$MOUNT_DIR" >/dev/null 2>&1 || \
    echo "Warning: unable to bless DMG folder; Finder layout was still written." >&2
fi

hdiutil detach "$DEV_NAME"
DEV_NAME=""
MOUNT_DIR=""

rm -f "$DMG_PATH"
hdiutil convert \
  -format UDZO \
  -ov \
  -o "$DMG_PATH" \
  "$RW_DMG_PATH"

echo "DMG created: $DMG_PATH"
