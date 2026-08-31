use super::{mapped_character, uppercase, BackendStatus, KeyboardBackend, TargetKey};
use std::{
    collections::HashSet,
    mem::size_of,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex, OnceLock,
    },
    thread::{self, JoinHandle},
};
use windows_sys::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::KeyboardAndMouse::{
            GetAsyncKeyState, GetKeyState, SendInput, INPUT, INPUT_0, KEYBDINPUT, KEYEVENTF_KEYUP,
            KEYEVENTF_UNICODE, VK_CAPITAL, VK_CONTROL, VK_LCONTROL, VK_LMENU, VK_LWIN, VK_MENU,
            VK_RCONTROL, VK_RMENU, VK_RWIN, VK_SHIFT,
        },
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageW, GetMessageW, PostThreadMessageW, SetWindowsHookExW,
            TranslateMessage, UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL,
            WM_KEYDOWN, WM_KEYUP, WM_QUIT, WM_SYSKEYDOWN, WM_SYSKEYUP,
        },
    },
};

const INJECT_MARKER: usize = 0x474b4153;

struct HookState {
    enabled: Arc<AtomicBool>,
    intercepted: Mutex<HashSet<TargetKey>>,
    hook: Mutex<Option<isize>>,
    thread_id: Mutex<Option<u32>>,
    caps_lock: AtomicBool,
    caps_key_down: AtomicBool,
    status: Mutex<BackendStatus>,
}

static HOOK_STATE: OnceLock<Arc<HookState>> = OnceLock::new();

pub struct WindowsBackend {
    state: Arc<HookState>,
    thread: Mutex<Option<JoinHandle<()>>>,
}

impl WindowsBackend {
    pub fn new(enabled: Arc<AtomicBool>) -> Self {
        Self {
            state: Arc::new(HookState {
                enabled,
                intercepted: Mutex::new(HashSet::new()),
                hook: Mutex::new(None),
                thread_id: Mutex::new(None),
                caps_lock: AtomicBool::new(false),
                caps_key_down: AtomicBool::new(false),
                status: Mutex::new(BackendStatus::Disabled),
            }),
            thread: Mutex::new(None),
        }
    }
}

impl KeyboardBackend for WindowsBackend {
    fn start(&self) -> Result<(), String> {
        let _ = HOOK_STATE.set(self.state.clone());
        let state = self.state.clone();
        let (ready_tx, ready_rx) = mpsc::sync_channel(1);
        let handle = thread::Builder::new()
            .name("german-keyboard-hook".into())
            .spawn(move || unsafe {
                *state.thread_id.lock().unwrap() =
                    Some(windows_sys::Win32::System::Threading::GetCurrentThreadId());
                state
                    .caps_lock
                    .store((GetKeyState(VK_CAPITAL as i32) & 1) != 0, Ordering::Relaxed);
                let module = GetModuleHandleW(std::ptr::null());
                let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), module, 0);
                if hook.is_null() {
                    *state.status.lock().unwrap() = BackendStatus::InitializationFailed;
                    let _ = ready_tx.send(Err(format!(
                        "无法注册 Windows 键盘钩子，错误码 {}",
                        windows_sys::Win32::Foundation::GetLastError()
                    )));
                    return;
                }
                *state.hook.lock().unwrap() = Some(hook as isize);
                *state.status.lock().unwrap() = if state.enabled.load(Ordering::Relaxed) {
                    BackendStatus::Running
                } else {
                    BackendStatus::Disabled
                };
                let _ = ready_tx.send(Ok(()));
                let mut message: MSG = std::mem::zeroed();
                while GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) > 0 {
                    TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
                UnhookWindowsHookEx(hook);
                *state.hook.lock().unwrap() = None;
                *state.thread_id.lock().unwrap() = None;
            })
            .map_err(|e| e.to_string())?;
        *self.thread.lock().unwrap() = Some(handle);
        ready_rx.recv().map_err(|e| e.to_string())?
    }

    fn set_enabled(&self, enabled: bool) -> Result<(), String> {
        if self.status() == BackendStatus::InitializationFailed {
            return Err("Windows 键盘钩子未运行".into());
        }
        self.state.enabled.store(enabled, Ordering::Relaxed);
        *self.state.status.lock().unwrap() = if enabled {
            BackendStatus::Running
        } else {
            BackendStatus::Disabled
        };
        Ok(())
    }

    fn refresh_permission(&self) -> Result<bool, String> {
        Ok(true)
    }

    fn stop(&self) {
        unsafe {
            if let Some(hook) = *self.state.hook.lock().unwrap() {
                UnhookWindowsHookEx(hook as HHOOK);
            }
            if let Some(thread_id) = *self.state.thread_id.lock().unwrap() {
                let _ = PostThreadMessageW(thread_id, WM_QUIT, 0, 0);
            }
        }
        if let Some(handle) = self.thread.lock().unwrap().take() {
            let _ = handle.join();
        }
    }

    fn permission_status(&self) -> bool {
        true
    }

    fn status(&self) -> BackendStatus {
        self.state.status.lock().unwrap().clone()
    }
}

impl Drop for WindowsBackend {
    fn drop(&mut self) {
        self.stop();
    }
}

unsafe extern "system" fn hook_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code < 0 {
        return CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param);
    }
    let Some(state) = HOOK_STATE.get() else {
        return CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param);
    };
    let event = &*(l_param as *const KBDLLHOOKSTRUCT);
    if event.flags & 0x10 != 0 && event.dwExtraInfo == INJECT_MARKER {
        return CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param);
    }
    let message = w_param as u32;
    let is_down = message == WM_KEYDOWN || message == WM_SYSKEYDOWN;
    let is_up = message == WM_KEYUP || message == WM_SYSKEYUP;
    if !is_down && !is_up {
        return CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param);
    }
    if event.vkCode == VK_CAPITAL as u32 {
        if is_down && !state.caps_key_down.swap(true, Ordering::Relaxed) {
            state.caps_lock.fetch_xor(true, Ordering::Relaxed);
        } else if is_up {
            state.caps_key_down.store(false, Ordering::Relaxed);
        }
    }
    let Some(key) = target_key(event.scanCode as u16, event.flags) else {
        return CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param);
    };
    if is_up {
        if state.intercepted.lock().unwrap().remove(&key) {
            return 1;
        }
        return CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param);
    }
    if !state.enabled.load(Ordering::Relaxed) || has_blocking_modifier() {
        return CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param);
    }
    let caps = state.caps_lock.load(Ordering::Relaxed);
    let shift = (GetAsyncKeyState(VK_SHIFT as i32) as u16 & 0x8000) != 0;
    let character = mapped_character(key, uppercase(caps, shift));
    if !inject_unicode(character) {
        return CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param);
    }
    state.intercepted.lock().unwrap().insert(key);
    1
}

fn target_key(scan_code: u16, flags: u32) -> Option<TargetKey> {
    if flags & 0x01 != 0 {
        return None;
    }
    match scan_code {
        0x1a => Some(TargetKey::LeftBracket),
        0x28 => Some(TargetKey::Quote),
        0x27 => Some(TargetKey::Semicolon),
        0x0c => Some(TargetKey::Minus),
        _ => None,
    }
}

unsafe fn has_blocking_modifier() -> bool {
    let keys = [
        VK_CONTROL,
        VK_LCONTROL,
        VK_RCONTROL,
        VK_MENU,
        VK_LMENU,
        VK_RMENU,
        VK_LWIN,
        VK_RWIN,
    ];
    keys.iter()
        .any(|key| GetAsyncKeyState(*key as i32) as u16 & 0x8000 != 0)
}

unsafe fn inject_unicode(character: char) -> bool {
    let input = |flags: u32| INPUT {
        r#type: 1,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: 0,
                wScan: character as u16,
                dwFlags: flags | KEYEVENTF_UNICODE,
                time: 0,
                dwExtraInfo: INJECT_MARKER,
            },
        },
    };
    let inputs = [input(0), input(KEYEVENTF_KEYUP)];
    SendInput(
        inputs.len() as u32,
        inputs.as_ptr(),
        size_of::<INPUT>() as i32,
    ) == inputs.len() as u32
}
