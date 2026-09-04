# German Character Keys for US Keyboards (ÄÖÜß)

[English](README.md) · [Deutsch](README.de.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md)

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
| macOS Apple Silicon | Full mapping support | Requires Accessibility permission. |

## Limitations

- Password fields, macOS Secure Input, remote desktop sessions, and some elevated windows may restrict keyboard event interception.
