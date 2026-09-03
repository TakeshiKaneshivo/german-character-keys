# 美式鍵盤德語字元鍵（ÄÖÜß）

[English](README.md) · [Deutsch](README.de.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md)

一個輕量桌面工具，讓你直接在美式 ANSI 鍵盤上輸入德語字元，支援 Windows 與 macOS。

![德語 QWERTZ 鍵盤配置](public/images/german-keyboard-layout.png)

## 功能

- 將四個美式鍵盤實體按鍵映射為德語字元。
- 使用 `Shift` 與 `Caps Lock` 輸入大寫與小寫字元。
- 與 `Ctrl`、`Alt`、`Command`、`Option` 或 `Windows` 修飾鍵組合時透傳原始按鍵。
- 使用可設定的全域快速鍵切換映射。
- 可選擇登入時自動啟動。
- 支援淺色、夜間與跟隨系統主題。
- 支援簡體中文、繁體中文、English 與 Deutsch 介面。
- 提供獨立說明視窗、鍵盤示意圖與映射說明。
- 在作業系統要求時顯示輔助功能權限提示。

## 按鍵映射

啟用映射後，以下美式鍵盤實體按鍵會輸出德語字元：

```text
[  -> ü / Ü
'  -> ä / Ä
;  -> ö / Ö
-  -> ß / ẞ
```

按住 `Shift` 可輸入大寫字元。Caps Lock 會改變預設大小寫，同時按住 `Shift` 與 Caps Lock 時會反轉大小寫。

與 `Ctrl`、`Alt`、`Command`、`Option` 或 `Windows` 組合使用時，按鍵保持不變。程式處理實體按鍵事件，不讀取輸入法內部狀態，因此其他輸入法啟用時也會執行映射。

## 使用方式

1. 安裝並啟動應用程式。
2. 在主介面啟用德語字元映射。
3. 在支援輸入的文字欄位中按下上述按鍵。
4. 開啟快速鍵編輯器，錄製或還原全域切換快速鍵。
5. 使用語言按鈕切換介面語言。
6. 使用外觀按鈕選擇淺色、夜間或跟隨系統。
7. 開啟說明頁查看鍵盤圖與完整映射說明。

macOS 使用者需要先在 **系統設定 > 隱私權與安全性 > 輔助使用** 中允許本應用程式，然後才能啟用映射。Windows 不需要額外的輔助功能權限。

## 平台支援

| 平台 | 支援情況 | 說明 |
| --- | --- | --- |
| Windows x64 | 完整支援映射 | 使用低階鍵盤勾點，不需要額外應用程式權限。 |
| macOS Apple Silicon | 完整支援映射 | 需要輔助使用權限，Release 目標為 `aarch64-apple-darwin`。 |

## 開發

需要 Node.js、npm、Rust，以及 Tauri 2 要求的平台開發環境。

```bash
npm install
npm run tauri dev
```

單獨建置前端：

```bash
npm run build
```

執行 Rust 測試與格式檢查：

```bash
cargo test --manifest-path src-tauri/Cargo.toml
npm run format:check
```

完整檢查會執行格式檢查、測試、Clippy 與前端建置：

```bash
npm run check
```

## Release 建置

### Windows

建置 Windows x64 NSIS 安裝程式：

```bash
npm run build:windows-release
```

也可以在專案根目錄執行 `build-windows-release.cmd`。安裝程式輸出到 `src-tauri/target/release/bundle/nsis/`，此命令只產生 NSIS，不產生 MSI。

### macOS

請在 Apple Silicon Mac 上建置 Apple Silicon DMG：

```bash
chmod +x build-macos-release.sh
./build-macos-release.sh
```

等效 npm 命令是 `npm run build:macos-release`。DMG 輸出到 `src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/`。

未簽署或未公證的 macOS 建置僅用於測試與內部分發。公開發佈需要 Apple Developer 簽署與公證。

## 限制

- 密碼欄位、macOS Secure Input、遠端桌面工作階段和部分高權限視窗可能限制鍵盤事件攔截。
- 程式映射實體按鍵，不偵測或修改輸入法內部狀態。
- macOS 必須先取得輔助使用權限才能啟動映射。

## 技術堆疊

桌面外殼與原生鍵盤整合使用 Tauri 2 與 Rust。介面使用 Vue 3、Vite、Tailwind CSS 4、Reka UI、Lucide 圖示與 motion-v。Windows 與 macOS 鍵盤後端位於 `src-tauri/src/keyboard/`。

## License

目前儲存庫尚未宣告授權條款。在加入授權檔案前，所有權利歸著作權持有人所有。
