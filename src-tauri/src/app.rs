use crate::{
    keyboard::{create_backend, BackendStatus, KeyboardBackend, OperationResult},
    state::{default_shortcut, RuntimeState, SharedState},
};
use std::sync::{atomic::Ordering, Arc};
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::WebviewWindowBuilder,
    AppHandle, Manager, Runtime, State, WebviewUrl,
};
use tauri_plugin_autostart::ManagerExt as AutoStartManagerExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _, event| {
                    if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        let _ = toggle_enabled(app);
                        let _ = sync_tray(app);
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            show_main_window(app);
        }))
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            get_status,
            set_enabled,
            set_shortcut,
            reset_shortcut,
            set_launch_at_login,
            get_permission_status,
            refresh_permission,
            open_accessibility_settings,
            open_help_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running application");
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("无法获取应用配置目录: {e}"))?;
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("无法创建应用配置目录 {}: {e}", config_dir.display()))?;
    let state = Arc::new(SharedState::load(config_dir.join("config.json")));
    let backend: Arc<dyn KeyboardBackend> = Arc::from(create_backend(state.enabled.clone()));
    let permission = backend.permission_status();
    *state.accessibility_granted.lock().unwrap() = permission;

    if let Err(error) = backend.start() {
        state.enabled.store(false, Ordering::Relaxed);
        if backend.status() != BackendStatus::PermissionRequired {
            state.config.lock().unwrap().enabled = false;
            let _ = state.save();
        }
        set_backend_status(&state, backend.status(), Some(error));
    } else if let Err(error) = backend.set_enabled(state.enabled.load(Ordering::Relaxed)) {
        state.enabled.store(false, Ordering::Relaxed);
        state.config.lock().unwrap().enabled = false;
        let _ = state.save();
        set_backend_status(&state, backend.status(), Some(error));
    } else {
        set_backend_status(&state, backend.status(), None);
    }

    app.manage(state.clone());
    app.manage(backend);
    let menu = build_tray_menu(app, &state)?;
    TrayIconBuilder::with_id("main")
        .icon(tauri::include_image!("icons/icon.png"))
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle" => {
                let _ = toggle_enabled(app);
                let _ = sync_tray(app);
            }
            "settings" => {
                show_main_window(app);
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;

    let _ = register_shortcut(app.handle(), &state.config.lock().unwrap().toggle_shortcut);
    if let Some(window) = app.get_webview_window("main") {
        let hidden_window = window.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = hidden_window.hide();
            }
        });
    }
    Ok(())
}

fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_always_on_top(true);
        let _ = window.set_focus();
        let _ = window.set_always_on_top(false);
    }
}

fn build_tray_menu<R: Runtime, M: Manager<R>>(
    app: &M,
    state: &SharedState,
) -> tauri::Result<Menu<R>> {
    let runtime = state.runtime();
    let enable = CheckMenuItem::with_id(
        app,
        "toggle",
        "启用辅助映射",
        runtime.enabled,
        true,
        None::<&str>,
    )?;
    let status = MenuItem::with_id(app, "status", tray_status(&runtime), false, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    Menu::with_items(app, &[&status, &enable, &settings, &quit])
}

fn tray_status(runtime: &RuntimeState) -> String {
    match runtime.backend_status {
        BackendStatus::PermissionRequired => "当前状态：需要权限".into(),
        BackendStatus::InitializationFailed => "当前状态：初始化失败".into(),
        BackendStatus::Running if runtime.enabled => "当前状态：开启".into(),
        _ => "当前状态：关闭".into(),
    }
}

fn sync_tray(app: &AppHandle) -> Result<(), String> {
    let state = app.state::<Arc<SharedState>>();
    let menu = build_tray_menu(app, &state).map_err(|e| e.to_string())?;
    app.tray_by_id("main")
        .ok_or_else(|| "托盘图标不存在".to_string())?
        .set_menu(Some(menu))
        .map_err(|e| e.to_string())
}

fn register_shortcut(app: &AppHandle, value: &str) -> Result<(), String> {
    let state = app.state::<Arc<SharedState>>();
    let parsed = parse_shortcut(value);
    let result = parsed.map_err(|e| e.to_string()).and_then(|shortcut| {
        app.global_shortcut()
            .register(shortcut)
            .map_err(|e| e.to_string())
    });
    *state.shortcut_registered.lock().unwrap() = result.is_ok();
    if let Err(error) = result {
        *state.message.lock().unwrap() = Some(format!("快捷键注册失败: {error}"));
        return Err(error);
    }
    Ok(())
}

fn parse_shortcut(value: &str) -> Result<Shortcut, String> {
    value
        .parse()
        .map_err(|error| format!("无效快捷键: {error}"))
}

fn set_backend_status(state: &SharedState, status: BackendStatus, message: Option<String>) {
    *state.backend_status.lock().unwrap() = status;
    *state.message.lock().unwrap() = message;
}

fn result(state: &SharedState, success: bool, message: Option<String>) -> OperationResult {
    *state.message.lock().unwrap() = message.clone();
    OperationResult {
        success,
        message,
        status: state.runtime(),
    }
}

fn failure(state: &SharedState, message: impl Into<String>) -> OperationResult {
    result(state, false, Some(message.into()))
}

fn apply_enabled(app: &AppHandle, enabled: bool) -> Result<OperationResult, String> {
    let state = app.state::<Arc<SharedState>>();
    let _operation = state.operation_lock.lock().map_err(|e| e.to_string())?;
    apply_enabled_locked(app, &state, enabled)
}

fn apply_enabled_locked(
    app: &AppHandle,
    state: &SharedState,
    enabled: bool,
) -> Result<OperationResult, String> {
    let old_enabled = state.enabled.load(Ordering::Relaxed);
    if old_enabled == enabled {
        return Ok(result(state, true, None));
    }
    let backend = app.state::<Arc<dyn KeyboardBackend>>();
    if let Err(error) = backend.set_enabled(enabled) {
        set_backend_status(state, backend.status(), Some(error.clone()));
        return Ok(failure(state, error));
    }
    let old_config = state.config.lock().map_err(|e| e.to_string())?.clone();
    state.config.lock().map_err(|e| e.to_string())?.enabled = enabled;
    if let Err(error) = state.save_locked() {
        state
            .config
            .lock()
            .map_err(|e| e.to_string())?
            .clone_from(&old_config);
        let _ = backend.set_enabled(old_enabled);
        set_backend_status(state, backend.status(), Some(error.clone()));
        return Ok(failure(state, error));
    }
    state.enabled.store(enabled, Ordering::Relaxed);
    set_backend_status(state, backend.status(), None);
    Ok(result(state, true, None))
}

fn toggle_enabled(app: &AppHandle) -> Result<OperationResult, String> {
    let next = !app
        .state::<Arc<SharedState>>()
        .enabled
        .load(Ordering::Relaxed);
    apply_enabled(app, next)
}

#[tauri::command]
fn get_status(state: State<'_, Arc<SharedState>>) -> RuntimeState {
    state.runtime()
}

#[tauri::command]
fn set_enabled(app: AppHandle, enabled: bool) -> Result<OperationResult, String> {
    let operation = apply_enabled(&app, enabled)?;
    let _ = sync_tray(&app);
    Ok(operation)
}

#[tauri::command]
fn set_shortcut(app: AppHandle, shortcut: String) -> Result<OperationResult, String> {
    let parsed = parse_shortcut(&shortcut)?;
    let state = app.state::<Arc<SharedState>>();
    let _operation = state.operation_lock.lock().map_err(|e| e.to_string())?;
    let old_value = state
        .config
        .lock()
        .map_err(|e| e.to_string())?
        .toggle_shortcut
        .clone();
    let old_parsed: Option<Shortcut> = old_value.parse().ok();
    let manager = app.global_shortcut();
    manager.unregister_all().map_err(|e| e.to_string())?;
    if let Err(error) = manager.register(parsed) {
        let restored = old_parsed
            .and_then(|old| manager.register(old).err())
            .is_none();
        *state.shortcut_registered.lock().unwrap() = restored && old_parsed.is_some();
        return Ok(failure(
            &state,
            if restored {
                error.to_string()
            } else {
                format!("{error}; 旧快捷键恢复失败")
            },
        ));
    }
    state
        .config
        .lock()
        .map_err(|e| e.to_string())?
        .toggle_shortcut = shortcut;
    if let Err(error) = state.save_locked() {
        let _ = manager.unregister_all();
        let restored = old_parsed
            .and_then(|old| manager.register(old).err())
            .is_none();
        state
            .config
            .lock()
            .map_err(|e| e.to_string())?
            .toggle_shortcut = old_value;
        *state.shortcut_registered.lock().unwrap() = restored && old_parsed.is_some();
        return Ok(failure(&state, error));
    }
    *state.shortcut_registered.lock().unwrap() = true;
    let operation = result(&state, true, None);
    let _ = sync_tray(&app);
    Ok(operation)
}

#[tauri::command]
fn reset_shortcut(app: AppHandle) -> Result<OperationResult, String> {
    set_shortcut(app, default_shortcut())
}

#[tauri::command]
fn set_launch_at_login(app: AppHandle, enabled: bool) -> Result<OperationResult, String> {
    let state = app.state::<Arc<SharedState>>();
    let _operation = state.operation_lock.lock().map_err(|e| e.to_string())?;
    let old_config = state.config.lock().map_err(|e| e.to_string())?.clone();
    let old_autostart = app.autolaunch().is_enabled().map_err(|e| e.to_string())?;
    let operation = if enabled {
        app.autolaunch().enable()
    } else {
        app.autolaunch().disable()
    };
    if let Err(error) = operation {
        return Ok(failure(&state, error.to_string()));
    }
    state
        .config
        .lock()
        .map_err(|e| e.to_string())?
        .launch_at_login = enabled;
    if let Err(error) = state.save_locked() {
        let _ = if old_autostart {
            app.autolaunch().enable()
        } else {
            app.autolaunch().disable()
        };
        state
            .config
            .lock()
            .map_err(|e| e.to_string())?
            .clone_from(&old_config);
        return Ok(failure(&state, error));
    }
    Ok(result(&state, true, None))
}

#[tauri::command]
fn get_permission_status(state: State<'_, Arc<SharedState>>) -> bool {
    *state.accessibility_granted.lock().unwrap()
}

#[tauri::command]
fn refresh_permission(app: AppHandle) -> Result<OperationResult, String> {
    let state = app.state::<Arc<SharedState>>();
    let _operation = state.operation_lock.lock().map_err(|e| e.to_string())?;
    let backend = app.state::<Arc<dyn KeyboardBackend>>();
    let granted = match backend.refresh_permission() {
        Ok(granted) => granted,
        Err(error) => {
            set_backend_status(&state, backend.status(), Some(error.clone()));
            return Ok(failure(&state, error));
        }
    };
    *state.accessibility_granted.lock().unwrap() = granted;
    set_backend_status(&state, backend.status(), None);
    if !granted {
        return Ok(failure(&state, "仍未获得辅助功能权限"));
    }
    let operation = if state.config.lock().unwrap().enabled {
        apply_enabled_locked(&app, &state, true)?
    } else {
        result(&state, true, None)
    };
    let _ = sync_tray(&app);
    Ok(operation)
}

#[tauri::command]
fn open_accessibility_settings() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn open_help_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("help") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_always_on_top(true);
        let _ = window.set_focus();
        let _ = window.set_always_on_top(false);
        return Ok(());
    }

    let window = WebviewWindowBuilder::new(&app, "help", WebviewUrl::App("help.html".into()))
        .title("德语键盘辅助输入 - 帮助")
        .inner_size(820.0, 620.0)
        .resizable(true)
        .build()
        .map_err(|error| error.to_string())?;
    let _ = window.set_always_on_top(true);
    let _ = window.set_focus();
    let _ = window.set_always_on_top(false);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_shortcut;

    #[test]
    fn parses_multi_modifier_shortcuts() {
        assert!(parse_shortcut("Ctrl+Shift+K").is_ok());
        assert!(parse_shortcut("Ctrl+Shift+KeyK").is_ok());
        assert!(parse_shortcut("Alt+Digit1").is_ok());
        assert!(parse_shortcut("Command+Option+K").is_ok());
        assert!(parse_shortcut("Alt+Shift+A").is_ok());
        assert!(parse_shortcut("Ctrl+F12").is_ok());
    }

    #[test]
    fn rejects_invalid_shortcuts() {
        assert!(parse_shortcut("Ctrl+K+Shift").is_err());
        assert!(parse_shortcut("Ctrl").is_err());
    }
}
