import { invoke } from "@tauri-apps/api/core";
import type { OperationResult, Status } from "./types";

const runningInTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
let previewStatus: Status = { enabled: false, shortcut_registered: true, accessibility_granted: true, shortcut: "Ctrl+KeyK", platform: "preview", launch_at_login: false, backend_status: "disabled", message: null };
const previewResult = (message: string | null = null): OperationResult => ({ success: true, message, status: { ...previewStatus } });

export const getStatus = () => runningInTauri ? invoke<Status>("get_status") : Promise.resolve({ ...previewStatus });
export const setEnabled = (enabled: boolean) => runningInTauri ? invoke<OperationResult>("set_enabled", { enabled }) : Promise.resolve((previewStatus = { ...previewStatus, enabled }, previewResult()));
export const setShortcut = (shortcut: string) => runningInTauri ? invoke<OperationResult>("set_shortcut", { shortcut }) : Promise.resolve((previewStatus = { ...previewStatus, shortcut }, previewResult()));
export const resetShortcut = () => runningInTauri ? invoke<OperationResult>("reset_shortcut") : Promise.resolve((previewStatus = { ...previewStatus, shortcut: "Ctrl+KeyK" }, previewResult()));
export const setLaunchAtLogin = (enabled: boolean) => runningInTauri ? invoke<OperationResult>("set_launch_at_login", { enabled }) : Promise.resolve((previewStatus = { ...previewStatus, launch_at_login: enabled }, previewResult()));
export const refreshPermission = () => runningInTauri ? invoke<OperationResult>("refresh_permission") : Promise.resolve(previewResult());
export const openAccessibilitySettings = () => runningInTauri ? invoke("open_accessibility_settings") : Promise.resolve();
export const quitApp = () => runningInTauri ? invoke("quit_app") : Promise.resolve();
export const openHelpWindow = () => runningInTauri ? invoke("open_help_window") : Promise.resolve();
