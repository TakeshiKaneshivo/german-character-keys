import { invoke } from "@tauri-apps/api/core";
import "./styles.css";

type Status = {
  enabled: boolean;
  shortcut_registered: boolean;
  accessibility_granted: boolean;
  shortcut: string;
  platform: string;
  launch_at_login: boolean;
  backend_status: "running" | "disabled" | "permission_required" | "initialization_failed";
  message: string | null;
};

type OperationResult = { success: boolean; message?: string | null; status: Status };

const enabled = document.querySelector<HTMLInputElement>("#enabled")!;
const launchAtLogin = document.querySelector<HTMLInputElement>("#launch-at-login")!;
const statusBadge = document.querySelector<HTMLElement>("#status-badge")!;
const statusLabel = document.querySelector<HTMLElement>("#status-label")!;
const statusText = document.querySelector<HTMLElement>("#status-text")!;
const shortcutInput = document.querySelector<HTMLInputElement>("#shortcut-input")!;
const shortcutStatus = document.querySelector<HTMLElement>("#shortcut-status")!;
const permissionSection = document.querySelector<HTMLElement>("#permission-section")!;
const platformLabel = document.querySelector<HTMLElement>("#platform-label")!;
const menuWrap = document.querySelector<HTMLElement>(".menu-wrap")!;
const menuToggle = document.querySelector<HTMLButtonElement>("#secondary-menu-toggle")!;
const secondaryMenu = document.querySelector<HTMLElement>("#secondary-menu")!;
const openHelp = document.querySelector<HTMLButtonElement>("#open-help")!;

let committedShortcut = "";
let pendingShortcut = "";
let recordingShortcut = false;

const supportedCodes = new Set([
  ...Array.from({ length: 26 }, (_, index) => `Key${String.fromCharCode(65 + index)}`),
  ...Array.from({ length: 10 }, (_, index) => `Digit${index}`),
  ...Array.from({ length: 24 }, (_, index) => `F${index + 1}`),
  "Backquote", "Backslash", "BracketLeft", "BracketRight", "Comma", "Equal", "Minus", "Period", "Quote", "Semicolon", "Slash",
  "Backspace", "CapsLock", "Delete", "End", "Enter", "Escape", "Home", "Insert", "PageDown", "PageUp", "Pause", "PrintScreen", "ScrollLock", "Tab",
  "ArrowDown", "ArrowLeft", "ArrowRight", "ArrowUp", "Space",
  "NumLock", "Numpad0", "Numpad1", "Numpad2", "Numpad3", "Numpad4", "Numpad5", "Numpad6", "Numpad7", "Numpad8", "Numpad9",
  "NumpadAdd", "NumpadDecimal", "NumpadDivide", "NumpadEnter", "NumpadEqual", "NumpadMultiply", "NumpadSubtract",
]);

function keyName(code: string) {
  if (code.startsWith("Key")) return code.slice(3);
  if (code.startsWith("Digit")) return code.slice(5);
  const names: Record<string, string> = {
    Backquote: "`", Backslash: "\\", BracketLeft: "[", BracketRight: "]", Comma: ",", Equal: "=", Minus: "-", Period: ".", Quote: "'", Semicolon: ";", Slash: "/",
    ArrowDown: "↓", ArrowLeft: "←", ArrowRight: "→", ArrowUp: "↑", Space: "Space", NumpadAdd: "+", NumpadSubtract: "-", NumpadMultiply: "*", NumpadDivide: "/",
  };
  return names[code] ?? code;
}

function displayToken(token: string) {
  const tokenNames: Record<string, string> = { Control: "Ctrl", Ctrl: "Ctrl", Alt: "Alt", Option: "Alt", Shift: "Shift", Super: /Mac|iPhone|iPad/i.test(navigator.platform) ? "⌘" : "Win", Command: "⌘" };
  return tokenNames[token] ?? keyName(token);
}

function formatShortcut(value: string) {
  return value.split("+").map(displayToken).join(" + ");
}

function setMenuOpen(open: boolean) {
  secondaryMenu.classList.toggle("hidden", !open);
  menuToggle.setAttribute("aria-expanded", String(open));
}

function setRecordingMessage(message: string, error = false) {
  shortcutStatus.textContent = message;
  shortcutStatus.className = `hint ${error ? "error" : ""}`;
}

function startRecording() {
  if (recordingShortcut) return;
  recordingShortcut = true;
  shortcutInput.classList.add("recording");
  shortcutInput.value = "按下组合键…";
  setRecordingMessage("请按下至少包含一个修饰键的组合键。" );
}

function cancelRecording() {
  recordingShortcut = false;
  shortcutInput.classList.remove("recording");
  shortcutInput.value = formatShortcut(pendingShortcut || committedShortcut);
  setRecordingMessage("快捷键用于切换辅助映射。");
}

function captureShortcut(event: KeyboardEvent) {
  if (!recordingShortcut) return;
  event.preventDefault();
  event.stopPropagation();

  if (event.key === "Escape") {
    cancelRecording();
    return;
  }

  const modifierTokens = [
    event.ctrlKey ? "Ctrl" : null,
    event.altKey ? "Alt" : null,
    event.shiftKey ? "Shift" : null,
    event.metaKey ? "Super" : null,
  ].filter((token): token is string => token !== null);

  if (["Control", "Alt", "Shift", "Meta"].includes(event.key)) {
    shortcutInput.value = modifierTokens.map(displayToken).join(" + ");
    return;
  }

  if (modifierTokens.length === 0) {
    setRecordingMessage("请至少使用一个修饰键，例如 Ctrl、Alt、Shift 或 Command。", true);
    return;
  }

  if (!supportedCodes.has(event.code) || ["ControlLeft", "ControlRight", "AltLeft", "AltRight", "ShiftLeft", "ShiftRight", "MetaLeft", "MetaRight"].includes(event.code)) {
    setRecordingMessage(`无法识别按键：${event.code}`, true);
    return;
  }

  pendingShortcut = `${modifierTokens.join("+")}+${event.code}`;
  shortcutInput.value = formatShortcut(pendingShortcut);
  recordingShortcut = false;
  shortcutInput.classList.remove("recording");
  setRecordingMessage("快捷键已记录，点击“保存”完成注册。" );
}

function render(status: Status) {
  enabled.checked = status.enabled;
  statusLabel.textContent = status.enabled ? "开启" : "关闭";
  statusBadge.className = `badge ${status.enabled ? "on" : ""}`;
  statusText.textContent = status.enabled
    ? "四个美式键位正在输出德语字符。"
    : "开启后，四个美式键位会输出德语字符。";
  committedShortcut = status.shortcut;
  if (!recordingShortcut) {
    pendingShortcut = status.shortcut;
    shortcutInput.value = formatShortcut(status.shortcut);
  }
  shortcutStatus.textContent = status.shortcut_registered
    ? "快捷键已注册。"
    : "快捷键不可用，请更换其他组合；托盘菜单仍可使用。";
  shortcutStatus.className = "hint";
  platformLabel.textContent = status.platform;
  launchAtLogin.checked = status.launch_at_login;
  permissionSection.classList.toggle("hidden", status.accessibility_granted);
}

function renderOperation(operation: OperationResult) {
  render(operation.status);
  if (operation.message) {
    statusText.textContent = operation.message;
    shortcutStatus.textContent = operation.message;
    shortcutStatus.className = `hint ${operation.success ? "" : "error"}`;
  }
}

async function refresh() {
  render(await invoke<Status>("get_status"));
}

async function runOperation(command: string, args: Record<string, unknown> = {}) {
  try {
    renderOperation(await invoke<OperationResult>(command, args));
  } catch (error) {
    console.error(error);
    try {
      await refresh();
    } catch (refreshError) {
      console.error(refreshError);
      statusText.textContent = String(error);
      setRecordingMessage(String(error), true);
    }
  }
}

enabled.addEventListener("change", async () => {
  await runOperation("set_enabled", { enabled: enabled.checked });
});

launchAtLogin.addEventListener("change", async () => {
  await runOperation("set_launch_at_login", { enabled: launchAtLogin.checked });
});

shortcutInput.addEventListener("focus", startRecording);
shortcutInput.addEventListener("click", startRecording);
shortcutInput.addEventListener("keydown", captureShortcut);
shortcutInput.addEventListener("blur", () => {
  if (recordingShortcut) cancelRecording();
});

document.querySelector("#reset-shortcut")!.addEventListener("click", async () => {
  await runOperation("reset_shortcut");
});

document.querySelector("#save-shortcut")!.addEventListener("click", async () => {
  if (!pendingShortcut) {
    setRecordingMessage("请先录制一个快捷键。", true);
    return;
  }
  await runOperation("set_shortcut", { shortcut: pendingShortcut });
});

menuToggle.addEventListener("click", () => {
  setMenuOpen(secondaryMenu.classList.contains("hidden"));
});

openHelp.addEventListener("click", async () => {
  setMenuOpen(false);
  try {
    await invoke("open_help_window");
  } catch (error) {
    console.error(error);
    statusText.textContent = String(error);
  }
});

document.addEventListener("click", (event) => {
  if (!menuWrap.contains(event.target as Node)) setMenuOpen(false);
});

document.addEventListener("keydown", (event) => {
  if (event.key === "Escape") setMenuOpen(false);
});

document.querySelector("#open-permissions")!.addEventListener("click", async () => {
  try {
    await invoke("open_accessibility_settings");
  } catch (error) {
    console.error(error);
  }
});

document.querySelector("#refresh-permission")!.addEventListener("click", async () => {
  await runOperation("refresh_permission");
});

refresh().catch(console.error);
