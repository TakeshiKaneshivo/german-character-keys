# Deutsche Zeichentasten für US-Tastaturen (ÄÖÜß)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [Deutsch](README.de.md)

Ein schlankes Desktop-Programm, mit dem du deutsche Zeichen direkt auf einer US-ANSI-Tastatur eingeben kannst. Unterstützt werden Windows und macOS.

![Deutsches QWERTZ-Tastaturlayout](public/images/german-keyboard-layout.png)

## Funktionen

- Vier physische Tasten einer US-Tastatur werden deutschen Zeichen zugeordnet.
- Mit `Shift` und `Caps Lock` lassen sich Groß- und Kleinbuchstaben eingeben.
- Tasten mit `Ctrl`, `Alt`, `Command`, `Option` oder `Windows` werden unverändert weitergegeben.
- Die Zuordnung lässt sich mit einem konfigurierbaren globalen Tastenkürzel umschalten.
- Die App kann beim Anmelden automatisch gestartet werden.
- Wähle zwischen Hell, Nacht und Systemdarstellung.
- Die Oberfläche ist auf vereinfachtem Chinesisch, traditionellem Chinesisch, Englisch und Deutsch verfügbar.
- Ein eigenes Hilfefenster zeigt das Tastaturlayout und die vollständige Zuordnung.
- Wenn das Betriebssystem es verlangt, wird ein Hinweis für Bedienungshilfen angezeigt.

## Tastenbelegung

Bei aktivierter Zuordnung erzeugen diese physischen Tasten einer US-Tastatur deutsche Zeichen:

```text
[  -> ü / Ü
'  -> ä / Ä
;  -> ö / Ö
-  -> ß / ẞ
```

Mit `Shift` wird ein Großbuchstabe eingegeben. Caps Lock ändert die Standardschreibweise; werden `Shift` und Caps Lock zusammen gedrückt, wird sie umgekehrt.

Mit `Ctrl`, `Alt`, `Command`, `Option` oder `Windows` kombinierte Tasten bleiben unverändert. Die App verarbeitet physische Tastaturereignisse und liest den internen Zustand einer Eingabemethode nicht aus. Daher funktioniert die Zuordnung auch bei aktivierter anderer Eingabemethode.

## Verwendung

1. Installiere und starte die App.
2. Aktiviere die Zuordnung deutscher Zeichen im Hauptfenster.
3. Drücke eine der oben aufgeführten Tasten in einem unterstützten Textfeld.
4. Öffne den Shortcut-Editor, um das globale Tastenkürzel aufzuzeichnen oder zurückzusetzen.
5. Ändere die Sprache über die Sprachschaltfläche.
6. Wähle über die Darstellungsschaltfläche Hell, Nacht oder System.
7. Öffne die Hilfe für das Tastaturbild und die vollständige Erklärung.

Auf macOS muss die App zuerst unter **Systemeinstellungen > Datenschutz & Sicherheit > Bedienungshilfen** erlaubt werden. Unter Windows ist keine zusätzliche Bedienungshilfe-Berechtigung erforderlich.

## Plattformunterstützung

| Plattform | Unterstützung | Hinweise |
| --- | --- | --- |
| Windows x64 | Vollständige Zuordnung | Verwendet einen Low-Level-Tastatur-Hook und benötigt keine zusätzliche App-Berechtigung. |
| macOS Apple Silicon | Vollständige Zuordnung | Bedienungshilfen-Berechtigung erforderlich; Release-Ziel ist `aarch64-apple-darwin`. |

## Entwicklung

Benötigt werden Node.js, npm, Rust und die von Tauri 2 vorausgesetzten Plattformwerkzeuge.

```bash
npm install
npm run tauri dev
```

Frontend unabhängig bauen:

```bash
npm run build
```

Rust-Tests und Formatprüfung ausführen:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
npm run format:check
```

Die vollständige Prüfung führt Formatprüfung, Tests, Clippy und den Frontend-Build aus:

```bash
npm run check
```

## Release-Builds

### Windows

Erstelle den NSIS-Installer für Windows x64:

```bash
npm run build:windows-release
```

Alternativ kann `build-windows-release.cmd` im Projektstamm ausgeführt werden. Der Installer wird nach `src-tauri/target/release/bundle/nsis/` geschrieben. Der Befehl erzeugt nur NSIS, keine MSI-Datei.

### macOS

Erstelle die DMG für Apple Silicon auf einem Apple-Silicon-Mac:

```bash
chmod +x build-macos-release.sh
./build-macos-release.sh
```

Der entsprechende npm-Befehl lautet `npm run build:macos-release`. Die DMG wird nach `src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/` geschrieben.

Nicht signierte oder nicht notarized macOS-Builds sind nur für Tests und interne Verteilung gedacht. Für eine öffentliche Veröffentlichung sind Apple-Developer-Signatur und Notarisierung erforderlich.

## Einschränkungen

- Passwortfelder, macOS Secure Input, Remote-Desktop-Sitzungen und manche Fenster mit erhöhten Rechten können das Abfangen von Tastaturereignissen einschränken.
- Die App ordnet physische Tasten zu und erkennt oder verändert nicht den internen Zustand einer Eingabemethode.
- Unter macOS kann die Zuordnung erst nach Erteilung der Bedienungshilfen-Berechtigung gestartet werden.

## Technologie

Desktop-Shell und native Tastaturintegration verwenden Tauri 2 und Rust. Die Oberfläche basiert auf Vue 3, Vite, Tailwind CSS 4, Reka UI, Lucide-Icons und motion-v. Die Tastatur-Backends für Windows und macOS liegen in `src-tauri/src/keyboard/`.

## Lizenz

Dieses Repository hat derzeit keine deklarierte Lizenz. Bis eine Lizenzdatei hinzugefügt wird, verbleiben alle Rechte beim Urheberrechtsinhaber.
