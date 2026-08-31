use serde::Serialize;
use std::sync::{atomic::AtomicBool, Arc};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TargetKey {
    LeftBracket,
    Quote,
    Semicolon,
    Minus,
}

pub fn mapped_character(key: TargetKey, uppercase: bool) -> char {
    match (key, uppercase) {
        (TargetKey::LeftBracket, false) => 'ü',
        (TargetKey::LeftBracket, true) => 'Ü',
        (TargetKey::Quote, false) => 'ä',
        (TargetKey::Quote, true) => 'Ä',
        (TargetKey::Semicolon, false) => 'ö',
        (TargetKey::Semicolon, true) => 'Ö',
        (TargetKey::Minus, false) => 'ß',
        (TargetKey::Minus, true) => 'ẞ',
    }
}

pub fn uppercase(caps_lock: bool, shift: bool) -> bool {
    caps_lock ^ shift
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BackendStatus {
    Running,
    Disabled,
    PermissionRequired,
    InitializationFailed,
}

#[derive(Clone, Debug, Serialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: Option<String>,
    pub status: crate::state::RuntimeState,
}

pub trait KeyboardBackend: Send + Sync {
    fn start(&self) -> Result<(), String>;
    fn set_enabled(&self, enabled: bool) -> Result<(), String>;
    fn refresh_permission(&self) -> Result<bool, String>;
    fn stop(&self);
    fn permission_status(&self) -> bool;
    fn status(&self) -> BackendStatus;
}

#[cfg(target_os = "macos")]
mod macos;
#[cfg(windows)]
mod windows;

pub fn create_backend(enabled: Arc<AtomicBool>) -> Box<dyn KeyboardBackend> {
    #[cfg(windows)]
    {
        return Box::new(windows::WindowsBackend::new(enabled));
    }
    #[cfg(target_os = "macos")]
    {
        return Box::new(macos::MacosBackend::new(enabled));
    }
    #[allow(unreachable_code)]
    Box::new(UnsupportedBackend)
}

struct UnsupportedBackend;
impl KeyboardBackend for UnsupportedBackend {
    fn start(&self) -> Result<(), String> {
        Err("unsupported platform".into())
    }
    fn set_enabled(&self, _: bool) -> Result<(), String> {
        Ok(())
    }
    fn refresh_permission(&self) -> Result<bool, String> {
        Ok(false)
    }
    fn stop(&self) {}
    fn permission_status(&self) -> bool {
        false
    }
    fn status(&self) -> BackendStatus {
        BackendStatus::InitializationFailed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_matrix_is_correct() {
        for (key, lower, upper) in [
            (TargetKey::LeftBracket, 'ü', 'Ü'),
            (TargetKey::Quote, 'ä', 'Ä'),
            (TargetKey::Semicolon, 'ö', 'Ö'),
            (TargetKey::Minus, 'ß', 'ẞ'),
        ] {
            assert_eq!(mapped_character(key, false), lower);
            assert_eq!(mapped_character(key, true), upper);
        }
    }

    #[test]
    fn caps_and_shift_use_xor() {
        assert!(!uppercase(false, false));
        assert!(uppercase(false, true));
        assert!(uppercase(true, false));
        assert!(!uppercase(true, true));
    }
}
