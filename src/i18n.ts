export type Language = "zh-CN" | "zh-TW" | "en" | "de";

export const LANGUAGE_OPTIONS: Array<{ code: Language; abbreviation: string; label: string }> = [
  { code: "zh-CN", abbreviation: "SC", label: "简体中文" },
  { code: "zh-TW", abbreviation: "TC", label: "繁體中文" },
  { code: "en", abbreviation: "EN", label: "English" },
  { code: "de", abbreviation: "DE", label: "Deutsch" },
];

export type TranslationKey =
  | "brandEyebrow" | "appTitle" | "appSubtitle" | "languageMenuLabel" | "themeMenuLabel" | "themeSystem" | "themeLight" | "themeDark" | "closeHelp" | "moreMenuLabel" | "helpMenuItem" | "toggleMappingLabel" | "shortcutAriaLabel"
  | "statusOn" | "statusOff" | "statusEnabled" | "statusDisabled" | "statusEnabledFull" | "statusDisabledFull"
  | "shortcutKicker" | "globalShortcut" | "globalShortcutFull" | "shortcutDescription" | "shortcutDescriptionFull" | "save" | "reset" | "shortcutHint" | "shortcutHintFull"
  | "shortcutRegistered" | "shortcutUnavailable" | "mappingKicker" | "currentMapping" | "mappingCount"
  | "permissionTitle" | "permissionDescription" | "permissionInstruction" | "openSettings" | "refreshPermission" | "quitApp" | "autoLaunch"
  | "helpTitle" | "helpSubtitle" | "helpKeyboardKicker" | "helpKeyboardTitle" | "helpKeyboardDescription"
  | "helpKeyboardCaption" | "helpMappingKicker" | "helpMappingTitle" | "helpShiftUpper" | "helpNote"
  | "languageSimplified" | "languageTraditional" | "languageEnglish" | "languageGerman"
  | "recording" | "requiresModifier" | "recorded" | "saveRecorded" | "noShortcut" | "unknownKey";

type TranslationTable = Record<TranslationKey, string>;

export const translations: Record<Language, TranslationTable> = {
  "zh-CN": {
    brandEyebrow: "GERMAN CHARACTER KEYS", appTitle: "美式键盘德语字符键（ÄÖÜß）", appSubtitle: "在美式键盘上输入德语字符",
    languageMenuLabel: "选择语言", themeMenuLabel: "选择皮肤", themeSystem: "跟随系统", themeLight: "浅色", themeDark: "夜间", closeHelp: "关闭帮助窗口", moreMenuLabel: "打开更多菜单", helpMenuItem: "帮助", toggleMappingLabel: "启用或关闭德语辅助映射", shortcutAriaLabel: "全局快捷键，点击后按下组合键录制",
    statusOn: "开启", statusOff: "关闭", statusEnabled: "映射已开启", statusDisabled: "映射已关闭", statusEnabledFull: "四个美式键位正在输出德语字符。", statusDisabledFull: "开启后，四个美式键位会输出德语字符。",
    shortcutKicker: "快速切换", globalShortcut: "全局快捷键", globalShortcutFull: "全局快捷键", shortcutDescription: "快捷键切换映射", shortcutDescriptionFull: "使用快捷键开启或关闭映射。", save: "保存", reset: "恢复", shortcutHint: "快捷键切换辅助映射", shortcutHintFull: "快捷键用于切换辅助映射。",
    shortcutRegistered: "快捷键已注册。", shortcutUnavailable: "快捷键不可用，请更换其他组合；托盘菜单仍可使用。",
    mappingKicker: "键位速查", currentMapping: "当前映射", mappingCount: "4 组键位",
    permissionTitle: "需要辅助功能权限", permissionDescription: "macOS 需要允许本应用监控键盘事件，才能启用映射。", permissionInstruction: "请在“系统设置 → 隐私与安全性 → 辅助功能”中允许本应用，然后返回这里重新检测。", openSettings: "打开系统设置", refreshPermission: "重新检测权限", quitApp: "退出应用", autoLaunch: "登录时自动启动",
    helpTitle: "帮助", helpSubtitle: "了解德语键盘上的特殊字母位置和映射方式。", helpKeyboardKicker: "键盘位置", helpKeyboardTitle: "德语字母在哪里？", helpKeyboardDescription: "绿色键帽标出了德语键盘中常用的特殊字母位置。", helpKeyboardCaption: "German QWERTZ 键盘布局", helpMappingKicker: "映射说明", helpMappingTitle: "美式键位如何输出德语字符", helpShiftUpper: "按住 Shift 输出", helpNote: "映射只在辅助功能开启且按键没有被 Ctrl、Alt、Command、Option 或 Windows 等修饰键占用时生效。",
    languageSimplified: "简体中文", languageTraditional: "繁體中文", languageEnglish: "English", languageGerman: "Deutsch",
    recording: "按下组合键…", requiresModifier: "请至少使用一个修饰键，例如 Ctrl、Alt、Shift 或 Command。", recorded: "快捷键已记录并保存。", saveRecorded: "请按下至少包含一个修饰键的组合键。", noShortcut: "请先录制一个快捷键。", unknownKey: "无法识别按键：{key}",
  },
  "zh-TW": {
    brandEyebrow: "GERMAN CHARACTER KEYS", appTitle: "美式鍵盤德語字元鍵（ÄÖÜß）", appSubtitle: "在美式鍵盤上輸入德語字元",
    languageMenuLabel: "選擇語言", themeMenuLabel: "選擇主題", themeSystem: "跟隨系統", themeLight: "淺色", themeDark: "夜間", closeHelp: "關閉說明視窗", moreMenuLabel: "開啟更多選單", helpMenuItem: "說明", toggleMappingLabel: "啟用或關閉德語輔助映射", shortcutAriaLabel: "全域快速鍵，點擊後按下組合鍵錄製",
    statusOn: "開啟", statusOff: "關閉", statusEnabled: "映射已開啟", statusDisabled: "映射已關閉", statusEnabledFull: "四個美式鍵位正在輸出德語字元。", statusDisabledFull: "開啟後，四個美式鍵位會輸出德語字元。",
    shortcutKicker: "快速切換", globalShortcut: "全域快速鍵", globalShortcutFull: "全域快速鍵", shortcutDescription: "快速鍵切換映射", shortcutDescriptionFull: "使用快速鍵開啟或關閉映射。", save: "儲存", reset: "還原", shortcutHint: "快速鍵切換輔助映射", shortcutHintFull: "快速鍵用於切換輔助映射。",
    shortcutRegistered: "快速鍵已註冊。", shortcutUnavailable: "快速鍵無法使用，請更換其他組合；仍可使用系統匣選單。",
    mappingKicker: "按鍵速查", currentMapping: "目前映射", mappingCount: "4 組按鍵",
    permissionTitle: "需要輔助功能權限", permissionDescription: "macOS 需要允許本應用程式監控鍵盤事件，才能啟用映射。", permissionInstruction: "請在「系統設定 → 隱私權與安全性 → 輔助功能」中允許本應用程式，然後返回此處重新檢查。", openSettings: "開啟系統設定", refreshPermission: "重新檢查權限", quitApp: "退出應用程式", autoLaunch: "登入時自動啟動",
    helpTitle: "說明", helpSubtitle: "了解德語鍵盤上的特殊字母位置和映射方式。", helpKeyboardKicker: "鍵盤位置", helpKeyboardTitle: "德語字母在哪裡？", helpKeyboardDescription: "綠色鍵帽標出了德語鍵盤中常用的特殊字母位置。", helpKeyboardCaption: "German QWERTZ 鍵盤配置", helpMappingKicker: "映射說明", helpMappingTitle: "美式按鍵如何輸出德語字元", helpShiftUpper: "按住 Shift 輸出", helpNote: "映射只在輔助功能開啟且按鍵沒有被 Ctrl、Alt、Command、Option 或 Windows 等修飾鍵佔用時生效。",
    languageSimplified: "簡體中文", languageTraditional: "繁體中文", languageEnglish: "English", languageGerman: "Deutsch",
    recording: "按下組合鍵…", requiresModifier: "請至少使用一個修飾鍵，例如 Ctrl、Alt、Shift 或 Command。", recorded: "快速鍵已記錄並儲存。", saveRecorded: "請按下至少包含一個修飾鍵的組合鍵。", noShortcut: "請先錄製一個快速鍵。", unknownKey: "無法識別按鍵：{key}",
  },
  en: {
    brandEyebrow: "GERMAN CHARACTER KEYS", appTitle: "German Character Keys for US Keyboards (ÄÖÜß)", appSubtitle: "Type German characters on a US keyboard",
    languageMenuLabel: "Choose language", themeMenuLabel: "Choose theme", themeSystem: "System", themeLight: "Light", themeDark: "Night", closeHelp: "Close help window", moreMenuLabel: "Open more menu", helpMenuItem: "Help", toggleMappingLabel: "Enable or disable German character mapping", shortcutAriaLabel: "Global shortcut; click and press a key combination to record it",
    statusOn: "On", statusOff: "Off", statusEnabled: "Mapping on", statusDisabled: "Mapping off", statusEnabledFull: "The four mapped keys are outputting German characters.", statusDisabledFull: "Turn this on to output German characters from four ANSI keys.",
    shortcutKicker: "Quick toggle", globalShortcut: "Global shortcut", globalShortcutFull: "Global shortcut", shortcutDescription: "Shortcut toggles mapping", shortcutDescriptionFull: "Use a shortcut to turn mapping on or off.", save: "Save", reset: "Reset", shortcutHint: "Shortcut toggles mapping", shortcutHintFull: "The shortcut toggles German character mapping.",
    shortcutRegistered: "Shortcut registered.", shortcutUnavailable: "Shortcut unavailable. Choose another combination; the tray menu remains available.",
    mappingKicker: "Key reference", currentMapping: "Current mapping", mappingCount: "4 key pairs",
    permissionTitle: "Accessibility permission required", permissionDescription: "macOS must allow this app to monitor keyboard events before mapping can be enabled.", permissionInstruction: "In System Settings → Privacy & Security → Accessibility, allow this app, then return here and check again.", openSettings: "Open System Settings", refreshPermission: "Check permission again", quitApp: "Quit app", autoLaunch: "Launch at login",
    helpTitle: "Help", helpSubtitle: "Learn where German special letters are located and how mapping works.", helpKeyboardKicker: "Keyboard layout", helpKeyboardTitle: "Where are the German letters?", helpKeyboardDescription: "Green keycaps mark commonly used special letters on a German keyboard.", helpKeyboardCaption: "German QWERTZ keyboard layout", helpMappingKicker: "Mapping details", helpMappingTitle: "How ANSI keys output German characters", helpShiftUpper: "Hold Shift for", helpNote: "Mapping works only when accessibility is enabled and the key is not combined with Ctrl, Alt, Command, Option, or Windows modifiers.",
    languageSimplified: "简体中文", languageTraditional: "繁體中文", languageEnglish: "English", languageGerman: "Deutsch",
    recording: "Press a shortcut…", requiresModifier: "Use at least one modifier such as Ctrl, Alt, Shift, or Command.", recorded: "Shortcut recorded and saved.", saveRecorded: "Press a combination containing at least one modifier.", noShortcut: "Record a shortcut first.", unknownKey: "Unrecognized key: {key}",
  },
  de: {
    brandEyebrow: "GERMAN CHARACTER KEYS", appTitle: "Deutsche Zeichentasten für US-Tastaturen (ÄÖÜß)", appSubtitle: "Deutsche Zeichen auf einer US-Tastatur eingeben",
    languageMenuLabel: "Sprache auswählen", themeMenuLabel: "Design auswählen", themeSystem: "System", themeLight: "Hell", themeDark: "Nacht", closeHelp: "Hilfe schließen", moreMenuLabel: "Weitere Optionen öffnen", helpMenuItem: "Hilfe", toggleMappingLabel: "Deutsche Zeichen-Zuordnung ein- oder ausschalten", shortcutAriaLabel: "Globale Tastenkombination; klicken und Kombination drücken, um sie aufzuzeichnen",
    statusOn: "Ein", statusOff: "Aus", statusEnabled: "Mapping aktiv", statusDisabled: "Mapping aus", statusEnabledFull: "Die vier zugeordneten Tasten geben deutsche Zeichen aus.", statusDisabledFull: "Aktivieren, um deutsche Zeichen über vier ANSI-Tasten einzugeben.",
    shortcutKicker: "Schnellumschaltung", globalShortcut: "Globaler Shortcut", globalShortcutFull: "Globale Tastenkombination", shortcutDescription: "Zuordnung umschalten", shortcutDescriptionFull: "Mit der Tastenkombination die Zuordnung ein- oder ausschalten.", save: "Speichern", reset: "Zurücksetzen", shortcutHint: "Shortcut schaltet Mapping um", shortcutHintFull: "Die Tastenkombination schaltet die Zeichen-Zuordnung um.",
    shortcutRegistered: "Tastenkombination registriert.", shortcutUnavailable: "Tastenkombination nicht verfügbar. Bitte eine andere Kombination wählen; das Tray-Menü bleibt verfügbar.",
    mappingKicker: "Tastenübersicht", currentMapping: "Aktuelle Zuordnung", mappingCount: "4 Tastenpaare",
    permissionTitle: "Bedienungshilfen-Berechtigung erforderlich", permissionDescription: "macOS muss dieser App erlauben, Tastaturereignisse zu überwachen, bevor die Zuordnung aktiviert werden kann.", permissionInstruction: "Erlaube diese App unter Systemeinstellungen → Datenschutz & Sicherheit → Bedienungshilfen und prüfe die Berechtigung anschließend erneut.", openSettings: "Systemeinstellungen öffnen", refreshPermission: "Berechtigung erneut prüfen", quitApp: "App beenden", autoLaunch: "Bei Anmeldung starten",
    helpTitle: "Hilfe", helpSubtitle: "Erfahre, wo deutsche Sonderzeichen liegen und wie die Zuordnung funktioniert.", helpKeyboardKicker: "Tastaturbelegung", helpKeyboardTitle: "Wo sind die deutschen Buchstaben?", helpKeyboardDescription: "Grüne Tasten markieren häufig verwendete Sonderzeichen auf einer deutschen Tastatur.", helpKeyboardCaption: "Deutsche QWERTZ-Tastaturbelegung", helpMappingKicker: "Zuordnung", helpMappingTitle: "So geben ANSI-Tasten deutsche Zeichen aus", helpShiftUpper: "Mit Shift für", helpNote: "Die Zuordnung funktioniert nur bei aktivierten Bedienungshilfen und wenn die Taste nicht mit Ctrl, Alt, Command, Option oder Windows kombiniert wird.",
    languageSimplified: "简体中文", languageTraditional: "繁體中文", languageEnglish: "English", languageGerman: "Deutsch",
    recording: "Tastenkombination drücken…", requiresModifier: "Mindestens eine Zusatztaste wie Ctrl, Alt, Shift oder Command verwenden.", recorded: "Tastenkombination aufgezeichnet und gespeichert.", saveRecorded: "Eine Kombination mit mindestens einer Zusatztaste drücken.", noShortcut: "Zuerst eine Tastenkombination aufzeichnen.", unknownKey: "Taste nicht erkannt: {key}",
  },
};

const STORAGE_KEY = "german-key-assist.language";

export function getLanguage(): Language {
  const saved = localStorage.getItem(STORAGE_KEY) as Language | null;
  return saved && saved in translations ? saved : "zh-CN";
}

export function setLanguage(language: Language) {
  localStorage.setItem(STORAGE_KEY, language);
}

export function t(key: TranslationKey, values: Record<string, string> = {}) {
  let value = translations[getLanguage()][key];
  for (const [name, replacement] of Object.entries(values)) value = value.replace(`{${name}}`, replacement);
  return value;
}

export function languageAbbreviation(language = getLanguage()) {
  return LANGUAGE_OPTIONS.find((option) => option.code === language)?.abbreviation ?? "SC";
}
