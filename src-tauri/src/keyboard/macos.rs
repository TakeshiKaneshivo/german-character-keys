use super::{mapped_character, uppercase, BackendStatus, KeyboardBackend, TargetKey};
use std::{
    collections::HashSet,
    ffi::c_void,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc, Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

const EVENT_MARKER: i64 = 0x474b4153;
const KCG_HID_EVENT_TAP: u32 = 0;
const KCG_HEAD_INSERT_EVENT_TAP: u32 = 0;
const KCG_EVENT_KEY_DOWN: u32 = 10;
const KCG_EVENT_KEY_UP: u32 = 11;
const KCG_EVENT_FLAGS_CHANGED: u32 = 12;
const KCG_EVENT_TAP_DISABLED_BY_TIMEOUT: u32 = 0xFFFF_FFFE;
const KCG_EVENT_TAP_DISABLED_BY_USER_INPUT: u32 = 0xFFFF_FFFF;
const KCG_EVENT_SOURCE_USER_DATA: i64 = 42;
const KCG_KEYBOARD_EVENT_KEYCODE: i64 = 9;
const KCG_KEYBOARD_EVENT_AUTOREPEAT: i64 = 8;
const FLAG_ALPHA_SHIFT: u64 = 0x10000;
const FLAG_SHIFT: u64 = 0x20000;
const FLAG_CONTROL: u64 = 0x40000;
const FLAG_ALTERNATE: u64 = 0x80000;
const FLAG_COMMAND: u64 = 0x100000;
const FLAG_SECONDARY_FN: u64 = 0x800000;

struct CallbackState {
    enabled: Arc<AtomicBool>,
    intercepted: Mutex<HashSet<TargetKey>>,
    tap: AtomicUsize,
    status: Arc<Mutex<BackendStatus>>,
}

pub struct MacosBackend {
    enabled: Arc<AtomicBool>,
    thread: Mutex<Option<JoinHandle<()>>>,
    run_loop: Arc<Mutex<Option<usize>>>,
    tap: Arc<AtomicUsize>,
    status: Arc<Mutex<BackendStatus>>,
}

impl MacosBackend {
    pub fn new(enabled: Arc<AtomicBool>) -> Self {
        Self {
            enabled,
            thread: Mutex::new(None),
            run_loop: Arc::new(Mutex::new(None)),
            tap: Arc::new(AtomicUsize::new(0)),
            status: Arc::new(Mutex::new(BackendStatus::PermissionRequired)),
        }
    }
}

impl KeyboardBackend for MacosBackend {
    fn start(&self) -> Result<(), String> {
        if !accessibility_granted() {
            *self.status.lock().unwrap() = BackendStatus::PermissionRequired;
            return Err("需要辅助功能权限".into());
        }
        if matches!(
            self.status(),
            BackendStatus::Running | BackendStatus::Disabled
        ) {
            return Ok(());
        }
        if let Some(handle) = self.thread.lock().unwrap().take() {
            let _ = handle.join();
        }
        let enabled = self.enabled.clone();
        let run_loop_slot = self.run_loop.clone();
        let tap_slot = self.tap.clone();
        let status = self.status.clone();
        let (ready_tx, ready_rx) = mpsc::sync_channel(1);
        let handle = thread::Builder::new()
            .name("german-keyboard-event-tap".into())
            .spawn(move || unsafe {
                let state = Box::new(CallbackState {
                    enabled,
                    intercepted: Mutex::new(HashSet::new()),
                    tap: AtomicUsize::new(0),
                    status: status.clone(),
                });
                let callback_state = Box::into_raw(state);
                let mask = (1u64 << KCG_EVENT_KEY_DOWN)
                    | (1u64 << KCG_EVENT_KEY_UP)
                    | (1u64 << KCG_EVENT_FLAGS_CHANGED);
                let tap = CGEventTapCreate(
                    KCG_HID_EVENT_TAP,
                    KCG_HEAD_INSERT_EVENT_TAP,
                    0,
                    mask,
                    event_callback,
                    callback_state as *mut c_void,
                );
                if tap.is_null() {
                    *status.lock().unwrap() = BackendStatus::InitializationFailed;
                    let _ = ready_tx.send(Err("无法创建 macOS 键盘事件 tap".into()));
                    let _ = Box::from_raw(callback_state);
                    return;
                }
                let source = CFMachPortCreateRunLoopSource(std::ptr::null(), tap, 0);
                if source.is_null() {
                    *status.lock().unwrap() = BackendStatus::InitializationFailed;
                    let _ = ready_tx.send(Err("无法创建 macOS RunLoop source".into()));
                    CFRelease(tap as *const c_void);
                    let _ = Box::from_raw(callback_state);
                    return;
                }
                let run_loop = CFRunLoopGetCurrent();
                (*callback_state).tap.store(tap as usize, Ordering::Relaxed);
                tap_slot.store(tap as usize, Ordering::Relaxed);
                *run_loop_slot.lock().unwrap() = Some(run_loop as usize);
                CFRunLoopAddSource(run_loop, source, kCFRunLoopCommonModes);
                CGEventTapEnable(tap, true);
                *status.lock().unwrap() = if (*callback_state).enabled.load(Ordering::Relaxed) {
                    BackendStatus::Running
                } else {
                    BackendStatus::Disabled
                };
                let _ = ready_tx.send(Ok(()));
                CFRunLoopRun();
                CFRunLoopRemoveSource(run_loop, source, kCFRunLoopCommonModes);
                CFRelease(source as *const c_void);
                CFRelease(tap as *const c_void);
                tap_slot.store(0, Ordering::Relaxed);
                *run_loop_slot.lock().unwrap() = None;
                let _ = Box::from_raw(callback_state);
            })
            .map_err(|e| e.to_string())?;
        *self.thread.lock().unwrap() = Some(handle);
        match ready_rx.recv().map_err(|e| e.to_string())? {
            Ok(()) => Ok(()),
            Err(error) => {
                if let Some(handle) = self.thread.lock().unwrap().take() {
                    let _ = handle.join();
                }
                Err(error)
            }
        }
    }

    fn set_enabled(&self, enabled: bool) -> Result<(), String> {
        if matches!(
            self.status(),
            BackendStatus::PermissionRequired | BackendStatus::InitializationFailed
        ) {
            return Err("macOS 键盘事件 tap 不可用".into());
        }
        self.enabled.store(enabled, Ordering::Relaxed);
        *self.status.lock().unwrap() = if enabled {
            BackendStatus::Running
        } else {
            BackendStatus::Disabled
        };
        Ok(())
    }

    fn refresh_permission(&self) -> Result<bool, String> {
        if !accessibility_granted() {
            self.stop();
            *self.status.lock().unwrap() = BackendStatus::PermissionRequired;
            return Ok(false);
        }
        self.start()?;
        Ok(true)
    }

    fn stop(&self) {
        let tap = self.tap.swap(0, Ordering::Relaxed);
        if tap != 0 {
            unsafe {
                CGEventTapEnable(tap as *mut c_void, false);
            }
        }
        if let Some(run_loop) = self.run_loop.lock().unwrap().take() {
            unsafe {
                CFRunLoopStop(run_loop as *mut c_void);
            }
        }
        if let Some(handle) = self.thread.lock().unwrap().take() {
            let _ = handle.join();
        }
        *self.status.lock().unwrap() = BackendStatus::Disabled;
    }

    fn permission_status(&self) -> bool {
        accessibility_granted()
    }
    fn status(&self) -> BackendStatus {
        self.status.lock().unwrap().clone()
    }
}

impl Drop for MacosBackend {
    fn drop(&mut self) {
        self.stop();
    }
}

unsafe extern "C" fn event_callback(
    _: *mut c_void,
    _: u32,
    event: *mut c_void,
    user_data: *mut c_void,
) -> *mut c_void {
    let state = &*(user_data as *const CallbackState);
    let kind = CGEventGetType(event);
    if kind == KCG_EVENT_TAP_DISABLED_BY_TIMEOUT || kind == KCG_EVENT_TAP_DISABLED_BY_USER_INPUT {
        let tap = state.tap.load(Ordering::Relaxed);
        if tap != 0 {
            CGEventTapEnable(tap as *mut c_void, true);
            *state.status.lock().unwrap() = if state.enabled.load(Ordering::Relaxed) {
                BackendStatus::Running
            } else {
                BackendStatus::Disabled
            };
        }
        return event;
    }
    if CGEventGetIntegerValueField(event, KCG_EVENT_SOURCE_USER_DATA) == EVENT_MARKER
        || kind == KCG_EVENT_FLAGS_CHANGED
    {
        return event;
    }
    let keycode = CGEventGetIntegerValueField(event, KCG_KEYBOARD_EVENT_KEYCODE) as u16;
    let Some(key) = mac_key(keycode) else {
        return event;
    };
    if kind == KCG_EVENT_KEY_UP {
        if state.intercepted.lock().unwrap().remove(&key) {
            return std::ptr::null_mut();
        }
        return event;
    }
    let flags = CGEventGetFlags(event);
    if !state.enabled.load(Ordering::Relaxed)
        || flags & (FLAG_CONTROL | FLAG_ALTERNATE | FLAG_COMMAND | FLAG_SECONDARY_FN) != 0
    {
        return event;
    }
    let character = mapped_character(
        key,
        uppercase(flags & FLAG_ALPHA_SHIFT != 0, flags & FLAG_SHIFT != 0),
    );
    let repeat = CGEventGetIntegerValueField(event, KCG_KEYBOARD_EVENT_AUTOREPEAT) != 0;
    if !post_unicode(character, repeat) {
        return event;
    }
    state.intercepted.lock().unwrap().insert(key);
    std::ptr::null_mut()
}

fn mac_key(keycode: u16) -> Option<TargetKey> {
    match keycode {
        33 => Some(TargetKey::LeftBracket),
        39 => Some(TargetKey::Quote),
        41 => Some(TargetKey::Semicolon),
        27 => Some(TargetKey::Minus),
        _ => None,
    }
}

fn accessibility_granted() -> bool {
    unsafe { AXIsProcessTrusted() }
}

unsafe fn post_unicode(character: char, repeat: bool) -> bool {
    let event = CGEventCreateKeyboardEvent(std::ptr::null_mut(), 0, true);
    if event.is_null() {
        return false;
    }
    CGEventSetFlags(event, 0);
    let utf16: Vec<u16> = character.to_string().encode_utf16().collect();
    CGEventKeyboardSetUnicodeString(event, utf16.len(), utf16.as_ptr());
    CGEventSetIntegerValueField(event, KCG_EVENT_SOURCE_USER_DATA, EVENT_MARKER);
    if repeat {
        CGEventSetIntegerValueField(event, KCG_KEYBOARD_EVENT_AUTOREPEAT, 1);
    }
    CGEventPost(KCG_HID_EVENT_TAP, event);
    let up = CGEventCreateKeyboardEvent(std::ptr::null_mut(), 0, false);
    if up.is_null() {
        CFRelease(event as *const c_void);
        return false;
    }
    CGEventSetIntegerValueField(up, KCG_EVENT_SOURCE_USER_DATA, EVENT_MARKER);
    CGEventPost(KCG_HID_EVENT_TAP, up);
    CFRelease(up as *const c_void);
    CFRelease(event as *const c_void);
    true
}

type CFRunLoopRef = *mut c_void;
type CFMachPortRef = *mut c_void;
type CFRunLoopSourceRef = *mut c_void;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> bool;
    fn CGEventTapCreate(
        tap: u32,
        place: u32,
        options: u32,
        mask: u64,
        callback: unsafe extern "C" fn(*mut c_void, u32, *mut c_void, *mut c_void) -> *mut c_void,
        user_info: *mut c_void,
    ) -> CFMachPortRef;
    fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    fn CGEventGetType(event: *mut c_void) -> u32;
    fn CGEventGetFlags(event: *mut c_void) -> u64;
    fn CGEventGetIntegerValueField(event: *mut c_void, field: i64) -> i64;
    fn CGEventCreateKeyboardEvent(source: *mut c_void, keycode: u16, key_down: bool)
        -> *mut c_void;
    fn CGEventKeyboardSetUnicodeString(event: *mut c_void, length: usize, string: *const u16);
    fn CGEventSetFlags(event: *mut c_void, flags: u64);
    fn CGEventSetIntegerValueField(event: *mut c_void, field: i64, value: i64);
    fn CGEventPost(tap: u32, event: *mut c_void);
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRelease(value: *const c_void);
    fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    fn CFRunLoopRun();
    fn CFRunLoopStop(run_loop: CFRunLoopRef);
    fn CFMachPortCreateRunLoopSource(
        allocator: *const c_void,
        port: CFMachPortRef,
        order: isize,
    ) -> CFRunLoopSourceRef;
    fn CFRunLoopAddSource(run_loop: CFRunLoopRef, source: CFRunLoopSourceRef, mode: *const c_void);
    fn CFRunLoopRemoveSource(
        run_loop: CFRunLoopRef,
        source: CFRunLoopSourceRef,
        mode: *const c_void,
    );
    static kCFRunLoopCommonModes: *const c_void;
}
