import { ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";

export type ThemeMode = "system" | "light" | "dark";

const STORAGE_KEY = "german-key-assist.theme";
const theme = ref<ThemeMode>(readTheme());
let listening = false;
let systemListening = false;

const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
const isMac = typeof navigator !== "undefined" && /Mac|iPhone|iPad/i.test(navigator.platform);
const colorSchemeQuery = typeof window !== "undefined" && typeof window.matchMedia === "function"
  ? window.matchMedia("(prefers-color-scheme: dark)")
  : null;

function readTheme(): ThemeMode {
  const saved = localStorage.getItem(STORAGE_KEY);
  return saved === "light" || saved === "dark" || saved === "system" ? saved : "system";
}

function applyTheme(mode: ThemeMode) {
  if (mode === "system") document.documentElement.removeAttribute("data-theme");
  else document.documentElement.dataset.theme = mode;
}

function resolveTheme(mode: ThemeMode): "light" | "dark" {
  if (mode !== "system") return mode;
  return colorSchemeQuery?.matches ? "dark" : "light";
}

async function syncNativeBackground(mode: ThemeMode) {
  if (!isTauri || !isMac) return;
  const background = resolveTheme(mode) === "dark" ? "#202327" : "#ffffff";
  try {
    await getCurrentWindow().setBackgroundColor(background);
  } catch (error) {
    console.error("无法同步 macOS 窗口背景色", error);
  }
}

function refreshTheme(mode: ThemeMode) {
  applyTheme(mode);
  void syncNativeBackground(mode);
}

function ensureStorageListener() {
  if (listening) return;
  listening = true;
  window.addEventListener("storage", (event) => {
    if (event.key !== STORAGE_KEY) return;
    theme.value = readTheme();
    refreshTheme(theme.value);
  });
}

function ensureSystemThemeListener() {
  if (systemListening || !colorSchemeQuery) return;
  systemListening = true;
  const handleChange = () => {
    if (theme.value !== "system") return;
    refreshTheme(theme.value);
  };
  if (typeof colorSchemeQuery.addEventListener === "function") {
    colorSchemeQuery.addEventListener("change", handleChange);
    return;
  }
  colorSchemeQuery.addListener(handleChange);
}

export function useTheme() {
  ensureStorageListener();
  ensureSystemThemeListener();
  refreshTheme(theme.value);

  const setTheme = (mode: ThemeMode) => {
    theme.value = mode;
    localStorage.setItem(STORAGE_KEY, mode);
    refreshTheme(mode);
  };

  return { theme, setTheme };
}
