import { ref } from "vue";

export type ThemeMode = "system" | "light" | "dark";

const STORAGE_KEY = "german-key-assist.theme";
const theme = ref<ThemeMode>(readTheme());
let listening = false;

function readTheme(): ThemeMode {
  const saved = localStorage.getItem(STORAGE_KEY);
  return saved === "light" || saved === "dark" || saved === "system" ? saved : "system";
}

function applyTheme(mode: ThemeMode) {
  if (mode === "system") document.documentElement.removeAttribute("data-theme");
  else document.documentElement.dataset.theme = mode;
}

function ensureStorageListener() {
  if (listening) return;
  listening = true;
  window.addEventListener("storage", (event) => {
    if (event.key !== STORAGE_KEY) return;
    theme.value = readTheme();
    applyTheme(theme.value);
  });
}

export function useTheme() {
  ensureStorageListener();
  applyTheme(theme.value);

  const setTheme = (mode: ThemeMode) => {
    theme.value = mode;
    localStorage.setItem(STORAGE_KEY, mode);
    applyTheme(mode);
  };

  return { theme, setTheme };
}
