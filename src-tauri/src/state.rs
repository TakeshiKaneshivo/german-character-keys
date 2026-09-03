use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use crate::keyboard::BackendStatus;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub enabled: bool,
    pub toggle_shortcut: String,
    pub launch_at_login: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            toggle_shortcut: default_shortcut(),
            launch_at_login: false,
        }
    }
}

pub fn default_shortcut() -> String {
    if cfg!(target_os = "macos") {
        "Command+D".into()
    } else {
        "Ctrl+D".into()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RuntimeState {
    pub enabled: bool,
    pub shortcut_registered: bool,
    pub accessibility_granted: bool,
    pub shortcut: String,
    pub platform: String,
    pub launch_at_login: bool,
    pub backend_status: BackendStatus,
    pub message: Option<String>,
}

pub struct SharedState {
    pub config: Mutex<AppConfig>,
    pub operation_lock: Mutex<()>,
    pub enabled: Arc<std::sync::atomic::AtomicBool>,
    pub shortcut_registered: Mutex<bool>,
    pub accessibility_granted: Mutex<bool>,
    pub backend_status: Mutex<BackendStatus>,
    pub message: Mutex<Option<String>>,
    pub config_path: PathBuf,
}

impl SharedState {
    pub fn load(config_path: PathBuf) -> Self {
        let temp_path = config_path.with_extension("json.tmp");
        let _ = fs::remove_file(&temp_path);
        let config: AppConfig = fs::read_to_string(&config_path)
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_default();
        let enabled = Arc::new(std::sync::atomic::AtomicBool::new(config.enabled));
        Self {
            config: Mutex::new(config),
            operation_lock: Mutex::new(()),
            enabled,
            shortcut_registered: Mutex::new(false),
            accessibility_granted: Mutex::new(!cfg!(target_os = "macos")),
            backend_status: Mutex::new(if cfg!(target_os = "macos") {
                BackendStatus::PermissionRequired
            } else {
                BackendStatus::Disabled
            }),
            message: Mutex::new(None),
            config_path,
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let _operation = self.operation_lock.lock().map_err(|e| e.to_string())?;
        self.save_locked()
    }

    pub(crate) fn save_locked(&self) -> Result<(), String> {
        let config = self.config.lock().map_err(|e| e.to_string())?.clone();
        let contents = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
        let temp_path = self.config_path.with_extension("json.tmp");
        let write_result = (|| {
            let mut file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&temp_path)
                .map_err(|e| e.to_string())?;
            file.write_all(contents.as_bytes())
                .map_err(|e| e.to_string())?;
            file.sync_all().map_err(|e| e.to_string())?;
            drop(file);
            replace_config(&temp_path, &self.config_path)
        })();
        if write_result.is_err() {
            let _ = fs::remove_file(&temp_path);
        }
        write_result
    }

    pub fn runtime(&self) -> RuntimeState {
        let config = self.config.lock().expect("config mutex poisoned");
        RuntimeState {
            enabled: self.enabled.load(std::sync::atomic::Ordering::Relaxed),
            shortcut_registered: *self
                .shortcut_registered
                .lock()
                .expect("shortcut mutex poisoned"),
            accessibility_granted: *self
                .accessibility_granted
                .lock()
                .expect("permission mutex poisoned"),
            shortcut: config.toggle_shortcut.clone(),
            platform: std::env::consts::OS.to_string(),
            launch_at_login: config.launch_at_login,
            backend_status: self
                .backend_status
                .lock()
                .expect("backend mutex poisoned")
                .clone(),
            message: self.message.lock().expect("message mutex poisoned").clone(),
        }
    }
}

fn replace_config(temp_path: &Path, config_path: &Path) -> Result<(), String> {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::{
            Foundation::GetLastError,
            Storage::FileSystem::{MoveFileExW, MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH},
        };
        let source: Vec<u16> = temp_path.as_os_str().encode_wide().chain(Some(0)).collect();
        let target: Vec<u16> = config_path
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();
        let ok = unsafe {
            MoveFileExW(
                source.as_ptr(),
                target.as_ptr(),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
            )
        };
        if ok == 0 {
            return Err(format!("配置文件替换失败，Windows 错误码 {}", unsafe {
                GetLastError()
            }));
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        fs::rename(temp_path, config_path).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_disabled() {
        let config = AppConfig::default();
        assert!(!config.enabled);
        assert!(!config.launch_at_login);
        assert_eq!(
            config.toggle_shortcut,
            if cfg!(target_os = "macos") {
                "Command+D"
            } else {
                "Ctrl+D"
            }
        );
    }

    #[test]
    fn config_can_be_overwritten_repeatedly() {
        let path = std::env::temp_dir().join(format!(
            "german-key-assist-test-{}.json",
            std::process::id()
        ));
        let state = SharedState::load(path.clone());
        state.config.lock().unwrap().enabled = true;
        state.save().unwrap();
        state.config.lock().unwrap().enabled = false;
        state.save().unwrap();
        let loaded = SharedState::load(path.clone());
        assert!(!loaded.config.lock().unwrap().enabled);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn corrupt_config_falls_back_to_defaults() {
        let path = std::env::temp_dir().join(format!(
            "german-key-assist-corrupt-{}.json",
            std::process::id()
        ));
        fs::write(&path, b"not-json").unwrap();
        let state = SharedState::load(path.clone());
        assert!(!state.config.lock().unwrap().enabled);
        assert_eq!(
            state.config.lock().unwrap().toggle_shortcut,
            default_shortcut()
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn load_removes_stale_temp_file() {
        let path = std::env::temp_dir().join(format!(
            "german-key-assist-stale-{}.json",
            std::process::id()
        ));
        let temp_path = path.with_extension("json.tmp");
        fs::write(&temp_path, b"stale").unwrap();
        let _state = SharedState::load(path.clone());
        assert!(!temp_path.exists());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn concurrent_saves_write_valid_json_snapshots() {
        let path = std::env::temp_dir().join(format!(
            "german-key-assist-concurrent-{}.json",
            std::process::id()
        ));
        let state = Arc::new(SharedState::load(path.clone()));
        let mut workers = Vec::new();
        for enabled in [true, false, true, false] {
            let state = Arc::clone(&state);
            workers.push(std::thread::spawn(move || {
                state.config.lock().unwrap().enabled = enabled;
                state.save().unwrap();
            }));
        }
        for worker in workers {
            worker.join().unwrap();
        }
        let contents = fs::read_to_string(&path).unwrap();
        let parsed: AppConfig = serde_json::from_str(&contents).unwrap();
        assert!(!parsed.toggle_shortcut.is_empty());
        let _ = fs::remove_file(path);
    }
}
