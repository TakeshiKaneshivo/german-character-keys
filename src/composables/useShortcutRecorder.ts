import { computed, ref } from "vue";
import { useLanguage } from "./useLanguage";

const modifierCodes = new Set(["ControlLeft", "ControlRight", "AltLeft", "AltRight", "ShiftLeft", "ShiftRight", "MetaLeft", "MetaRight"]);
const supportedCodes = new Set([
  ...Array.from({ length: 26 }, (_, index) => `Key${String.fromCharCode(65 + index)}`),
  ...Array.from({ length: 10 }, (_, index) => `Digit${index}`),
  ...Array.from({ length: 24 }, (_, index) => `F${index + 1}`),
  "Backquote", "Backslash", "BracketLeft", "BracketRight", "Comma", "Equal", "Minus", "Period", "Quote", "Semicolon", "Slash",
  "Backspace", "CapsLock", "Delete", "End", "Enter", "Escape", "Home", "Insert", "PageDown", "PageUp", "Pause", "PrintScreen", "ScrollLock", "Tab",
  "ArrowDown", "ArrowLeft", "ArrowRight", "ArrowUp", "Space", "NumLock", "Numpad0", "Numpad1", "Numpad2", "Numpad3", "Numpad4", "Numpad5", "Numpad6", "Numpad7", "Numpad8", "Numpad9",
  "NumpadAdd", "NumpadDecimal", "NumpadDivide", "NumpadEnter", "NumpadEqual", "NumpadMultiply", "NumpadSubtract",
]);

function keyName(code: string) {
  if (code.startsWith("Key")) return code.slice(3);
  if (code.startsWith("Digit")) return code.slice(5);
  const names: Record<string, string> = { Backquote: "`", Backslash: "\\", BracketLeft: "[", BracketRight: "]", Comma: ",", Equal: "=", Minus: "-", Period: ".", Quote: "'", Semicolon: ";", Slash: "/", ArrowDown: "↓", ArrowLeft: "←", ArrowRight: "→", ArrowUp: "↑", Space: "Space", NumpadAdd: "+", NumpadSubtract: "-", NumpadMultiply: "*", NumpadDivide: "/" };
  return names[code] ?? code;
}

function displayToken(token: string) {
  const tokenNames: Record<string, string> = { Control: "Ctrl", Ctrl: "Ctrl", Alt: "Alt", Option: "Alt", Shift: "Shift", Super: /Mac|iPhone|iPad/i.test(navigator.platform) ? "⌘" : "Win", Command: "⌘" };
  return tokenNames[token] ?? keyName(token);
}

export function formatShortcut(value: string) { return value.split("+").map(displayToken).join(" + "); }

export function useShortcutRecorder(committed: () => string, save: (shortcut: string) => Promise<unknown>) {
  const { translate } = useLanguage();
  const recording = ref(false);
  const pending = ref("");
  const partial = ref("");
  const message = ref("");
  const messageError = ref(false);
  const displayValue = computed(() => recording.value ? (partial.value || translate("recording")) : formatShortcut(pending.value || committed()));

  function start() { if (!recording.value) { recording.value = true; partial.value = ""; message.value = translate("saveRecorded"); messageError.value = false; } }
  function cancel() { recording.value = false; partial.value = ""; pending.value = committed(); message.value = translate("shortcutHint"); messageError.value = false; }
  async function capture(event: KeyboardEvent) {
    if (!recording.value) return;
    event.preventDefault(); event.stopPropagation();
    if (event.key === "Escape") { cancel(); return; }
    const modifiers = [event.ctrlKey ? "Ctrl" : null, event.altKey ? "Alt" : null, event.shiftKey ? "Shift" : null, event.metaKey ? "Super" : null].filter((token): token is string => token !== null);
    if (["Control", "Alt", "Shift", "Meta"].includes(event.key)) { partial.value = modifiers.map(displayToken).join(" + "); return; }
    if (!modifiers.length) { message.value = translate("requiresModifier"); messageError.value = true; return; }
    if (!supportedCodes.has(event.code) || modifierCodes.has(event.code)) { message.value = translate("unknownKey", { key: event.code }); messageError.value = true; return; }
    pending.value = `${modifiers.join("+")}+${event.code}`;
    partial.value = "";
    recording.value = false; message.value = translate("recorded"); messageError.value = false;
    await save(pending.value);
  }
  return { recording, message, messageError, displayValue, start, cancel, capture };
}
