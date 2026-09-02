import { computed, onBeforeUnmount, ref } from "vue";
import { getLanguage, languageAbbreviation, LANGUAGE_OPTIONS, setLanguage, t, type Language, type TranslationKey } from "../i18n";

const language = ref<Language>(getLanguage());
const listeners = new Set<() => void>();
let listening = false;
function ensureStorageListener() {
  if (listening) return;
  listening = true;
  window.addEventListener("storage", (event) => {
    if (event.key === "german-key-assist.language") { language.value = getLanguage(); listeners.forEach((listener) => listener()); }
  });
}

export function useLanguage() {
  ensureStorageListener();
  const menuOpen = ref(false);
  const rerender = () => { language.value = getLanguage(); };
  listeners.add(rerender);
  onBeforeUnmount(() => listeners.delete(rerender));

  const translate = (key: TranslationKey, values: Record<string, string> = {}) => {
    language.value;
    return t(key, values);
  };
  const abbreviation = computed(() => languageAbbreviation(language.value));
  const chooseLanguage = (next: Language) => {
    setLanguage(next);
    language.value = next;
    menuOpen.value = false;
    document.documentElement.lang = next;
  };
  document.documentElement.lang = language.value;
  return { language, menuOpen, options: LANGUAGE_OPTIONS, translate, abbreviation, chooseLanguage };
}
