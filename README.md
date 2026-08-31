# German Key Assist

一个面向美式 ANSI 键盘的德语字符辅助输入工具。

## 映射

开启后，程序将以下物理按键转换为德语字符：

```text
[       -> ü / Ü
'       -> ä / Ä
;       -> ö / Ö
-       -> ß / ẞ
```

大写由 `Caps Lock XOR Shift` 决定。按住 Ctrl、Alt、Command、Option 或 Windows 键时，原始按键会被放行。程序不检测任何输入法内部状态，因此开启后中文输入法中的目标按键也会被转换。

## 开发

```text
npm install
npm run tauri dev
```

Windows 正式 Release 构建可以直接双击项目根目录的 `build-windows-release.cmd`。也可以运行：

```text
npm run build:windows-release
```

Windows Release 安装器位于 `src-tauri/target/release/bundle/nsis/German Key Assist_0.1.0_x64-setup.exe`，直接运行的 Release EXE 位于 `src-tauri/target/release/german-key-assist.exe`。两者都不会打开命令行窗口。脚本只支持 Windows x64，不会生成 MSI。不要使用 `--debug` 构建分发版本。

macOS Release 构建必须在 Apple Silicon Mac 上执行：

```bash
chmod +x build-macos-release.sh
./build-macos-release.sh
```

也可以运行：

```text
npm run build:macos-release
```

macOS 安装器位于 `src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/German Key Assist_0.1.0_aarch64.dmg`，直接运行的应用位于 `src-tauri/target/aarch64-apple-darwin/release/german-key-assist`。脚本只生成 Apple Silicon `arm64` DMG，不在 Windows 上交叉编译 macOS。

macOS 未签名或未公证的构建仅用于测试和内部分发。正式发布时需要配置 Apple Developer、Developer ID 签名和公证。

开发应用需要桌面权限来创建窗口和托盘。Windows 键盘钩子不需要额外的应用权限；macOS 首次开启映射前需要在系统设置的“隐私与安全性 > 辅助功能”中允许本应用。

## 验证

```text
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
npm run build:windows-release
```

macOS 只构建 Apple Silicon `arm64` 版本。密码框、Secure Input、远程桌面和部分高权限窗口的事件拦截能力取决于操作系统限制。
