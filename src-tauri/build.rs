fn main() {
    for path in [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico",
        "icons/icon.png",
        "icons/icon-enabled.png",
        "icons/source/icon-disabled-source.png",
        "icons/source/icon-enabled-source.png",
        "icons/tray-macos-disabled.png",
        "icons/tray-macos-enabled.png",
        "icons/tray-macos-disabled@2x.png",
        "icons/tray-macos-enabled@2x.png",
        "nsis/installer-header.bmp",
        "nsis/installer-sidebar.bmp",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
    tauri_build::build()
}
