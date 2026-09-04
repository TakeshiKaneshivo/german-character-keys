# Deutsche Zeichentasten für US-Tastaturen (ÄÖÜß)

[English](README.md) · [Deutsch](README.de.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md)

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
| macOS Apple Silicon | Vollständige Zuordnung | Bedienungshilfen-Berechtigung erforderlich. |

## Einschränkungen

- Passwortfelder, macOS Secure Input, Remote-Desktop-Sitzungen und manche Fenster mit erhöhten Rechten können das Abfangen von Tastaturereignissen einschränken.
