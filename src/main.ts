import { invoke } from "@tauri-apps/api/core";
import "./styles.css";

type Status = {
  enabled: boolean;
  shortcut_registered: boolean;
  accessibility_granted: boolean;
  shortcut: string;
  platform: string;
  launch_at_login: boolean;
};

type OperationResult = { success: boolean; message?: string | null; status: Status };

const enabled = document.querySelector<HTMLInputElement>("#enabled")!;
const launchAtLogin = document.querySelector<HTMLInputElement>("#launch-at-login")!;
const statusBadge = document.querySelector<HTMLElement>("#status-badge")!;
const statusText = document.querySelector<HTMLElement>("#status-text")!;
const shortcutInput = document.querySelector<HTMLInputElement>("#shortcut-input")!;
const shortcutStatus = document.querySelector<HTMLElement>("#shortcut-status")!;
const permissionSection = document.querySelector<HTMLElement>("#permission-section")!;
const platformLabel = document.querySelector<HTMLElement>("#platform-label")!;

function render(status: Status) {
  enabled.checked = status.enabled;
  statusBadge.textContent = status.enabled ? "开启" : "关闭";
  statusBadge.className = `badge ${status.enabled ? "on" : ""}`;
  statusText.textContent = status.enabled
    ? "四个美式键位正在输出德语字符。"
    : "开启后，四个美式键位会输出德语字符。";
  shortcutInput.value = status.shortcut;
  shortcutStatus.textContent = status.shortcut_registered
    ? "快捷键已注册。"
    : "快捷键不可用，请更换其他组合；托盘菜单仍可使用。";
  platformLabel.textContent = status.platform;
  launchAtLogin.checked = status.launch_at_login;
  permissionSection.classList.toggle("hidden", status.accessibility_granted);
}

function renderOperation(operation: OperationResult) {
  render(operation.status);
  if (operation.message) {
    statusText.textContent = operation.message;
    shortcutStatus.textContent = operation.message;
  }
}

async function refresh() {
  render(await invoke<Status>("get_status"));
}

enabled.addEventListener("change", async () => {
  try { renderOperation(await invoke<OperationResult>("set_enabled", { enabled: enabled.checked })); }
  catch (error) { console.error(error); await refresh(); }
});

launchAtLogin.addEventListener("change", async () => {
  try { renderOperation(await invoke<OperationResult>("set_launch_at_login", { enabled: launchAtLogin.checked })); }
  catch (error) { console.error(error); await refresh(); }
});

document.querySelector("#reset-shortcut")!.addEventListener("click", async () => {
  try { renderOperation(await invoke<OperationResult>("reset_shortcut")); }
  catch (error) { console.error(error); }
});

document.querySelector("#save-shortcut")!.addEventListener("click", async () => {
  try { renderOperation(await invoke<OperationResult>("set_shortcut", { shortcut: shortcutInput.value.trim() })); }
  catch (error) { shortcutStatus.textContent = String(error); }
});

document.querySelector("#open-permissions")!.addEventListener("click", async () => {
  await invoke("open_accessibility_settings");
});

document.querySelector("#refresh-permission")!.addEventListener("click", async () => {
  try { renderOperation(await invoke<OperationResult>("refresh_permission")); }
  catch (error) { shortcutStatus.textContent = String(error); }
});

refresh().catch(console.error);
