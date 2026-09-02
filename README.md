# German Character Keys for US Keyboards (ÄÖÜß)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [Deutsch](README.de.md)

Type German characters directly on a US ANSI keyboard with a small desktop helper for Windows and macOS.

![German QWERTZ keyboard layout](public/images/german-keyboard-layout.png)

## Features

- Map four physical US keyboard keys to German characters.
- Use `Shift` and `Caps Lock` to enter uppercase and lowercase characters.
- Pass through keys combined with `Ctrl`, `Alt`, `Command`, `Option`, or `Windows` modifiers.
- Toggle the mapping with a configurable global shortcut.
- Optionally launch the app at login.
- Choose Light, Night, or Follow System appearance.
- Use the interface in Simplified Chinese, Traditional Chinese, English, or German.
- Open a dedicated help window with a keyboard reference and mapping details.
- Receive an accessibility permission prompt where the operating system requires it.

## Key mapping

When mapping is enabled, these physical keys produce German characters:

```text
[  -> ü / Ü
'  -> ä / Ä
;  -> ö / Ö
-  -> ß / ẞ
```

Hold `Shift` to enter an uppercase character. Caps Lock changes the default case, and holding `Shift` together with Caps Lock reverses it.

Keys used with `Ctrl`, `Alt`, `Command`, `Option`, or `Windows` are left unchanged. The app works with physical key events and does not inspect the state of an input method, so the mapping also applies while another input method is active.

## Usage

1. Install and launch the app.
2. Enable German character mapping from the main window.
3. Press one of the mapped US keyboard keys in any supported text field.
4. Open the shortcut editor to record or reset the global toggle shortcut.
5. Use the language button to change the interface language.
6. Use the appearance button to choose Light, Night, or Follow System.
7. Open Help for the keyboard diagram and a complete mapping explanation.

On macOS, grant the app permission in **System Settings > Privacy & Security > Accessibility** before enabling mapping. Windows does not require an additional accessibility permission for the keyboard hook.

## Platform support

| Platform | Support | Notes |
| --- | --- | --- |
| Windows x64 | Full mapping support | Uses a low-level keyboard hook; no additional app permission is required. |
| macOS Apple Silicon | Full mapping support | Requires Accessibility permission. Release builds target `aarch64-apple-darwin`. |

## Development

Requirements: Node.js, npm, Rust, and the platform prerequisites documented by Tauri 2.

```bash
npm install
npm run tauri dev
```

The frontend can be built independently:

```bash
npm run build
```

Run Rust tests and formatting checks with:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
npm run format:check
```

The complete project check runs formatting, tests, Clippy, and the frontend build:

```bash
npm run check
```

## Release builds

### Windows

Build the x64 NSIS installer with:

```bash
npm run build:windows-release
```

You can also run `build-windows-release.cmd` from the repository root. The installer is written to `src-tauri/target/release/bundle/nsis/`. The release command produces NSIS only, not MSI.

### macOS

Build the Apple Silicon DMG on an Apple Silicon Mac:

```bash
chmod +x build-macos-release.sh
./build-macos-release.sh
```

The equivalent npm command is `npm run build:macos-release`. The DMG is written to `src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/`.

Unsigned or unnotarized macOS builds are intended for testing and internal distribution. Public distribution requires Apple Developer signing and notarization.

## Limitations

- Password fields, macOS Secure Input, remote desktop sessions, and some elevated windows may restrict keyboard event interception.
- The app maps physical keys and does not detect or modify an input method's internal state.
- On macOS, mapping cannot start until Accessibility permission has been granted.

## Technology

The desktop shell and native keyboard integration use Tauri 2 and Rust. The interface uses Vue 3, Vite, Tailwind CSS 4, Reka UI, Lucide icons, and motion-v. Windows and macOS keyboard backends are implemented in `src-tauri/src/keyboard/`.

## License

This repository does not currently declare a license. All rights remain with the copyright holder until a license file is added.
