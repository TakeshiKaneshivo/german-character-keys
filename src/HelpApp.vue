<script setup lang="ts">
import { ref, watchEffect } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useLanguage } from "./composables/useLanguage";
import { useTheme } from "./composables/useTheme";
import LanguageMenu from "./components/LanguageMenu.vue";
import { ArrowRight, X } from "lucide-vue-next";
import Kbd from "./components/ui/Kbd.vue";

const { translate } = useLanguage();
useTheme();
const helpWindow = getCurrentWindow();
const isMac = typeof navigator !== "undefined" && /Mac|iPhone|iPad/i.test(navigator.platform);
const windowError = ref("");
const mappings = [{ input: "[", output: "ü", upper: "Ü" }, { input: "'", output: "ä", upper: "Ä" }, { input: ";", output: "ö", upper: "Ö" }, { input: "-", output: "ß", upper: "ẞ" }];
watchEffect(() => { document.title = `${translate("appTitle")} - ${translate("helpTitle")}`; });

async function closeHelp() {
  windowError.value = "";
  try {
    await helpWindow.close();
  } catch (error) {
    windowError.value = String(error);
    console.error("无法关闭帮助窗口", error);
  }
}

async function startWindowDrag(event: MouseEvent) {
  if (event.button !== 0) return;
  const page = event.currentTarget as HTMLElement;
  if (event.target === page && event.offsetX >= page.clientWidth) return;
  const target = event.target as HTMLElement | null;
  if (target?.closest("button, a, input, select, textarea, [role=button], [role=menuitem], [data-no-drag]")) return;
  windowError.value = "";
  try {
    await helpWindow.startDragging();
  } catch (error) {
    windowError.value = String(error);
    console.error("无法拖动帮助窗口", error);
  }
}

</script>

<template>
  <main class="help-page" :class="{ 'platform-macos': isMac }" @mousedown="startWindowDrag"><div class="help-shell">
    <header class="help-header"><div class="brand-lockup"><div class="brand-mark"><img class="brand-logo" src="/images/german-character-keys-icon.png" alt="" aria-hidden="true" /></div><div><span class="eyebrow">{{ translate('brandEyebrow') }}</span><h1>{{ translate('helpTitle') }}</h1><p class="subtitle">{{ translate('helpSubtitle') }}</p></div></div><div class="help-header-actions"><LanguageMenu /><button class="icon-button close-button" type="button" :aria-label="translate('closeHelp')" :title="translate('closeHelp')" @click="closeHelp"><X :size="18" aria-hidden="true" /></button></div></header>
    <p v-if="windowError" class="window-action-error" role="alert">{{ windowError }}</p>
    <section class="help-section"><span class="section-kicker">{{ translate('helpKeyboardKicker') }}</span><h2>{{ translate('helpKeyboardTitle') }}</h2><p>{{ translate('helpKeyboardDescription') }}</p><figure class="keyboard-figure"><img src="/images/german-keyboard-layout.png" alt="German QWERTZ keyboard layout" /><figcaption>{{ translate('helpKeyboardCaption') }}</figcaption></figure></section>
    <section class="help-section mapping-explanation"><span class="section-kicker">{{ translate('helpMappingKicker') }}</span><h2>{{ translate('helpMappingTitle') }}</h2><div class="help-mapping-list"><div v-for="mapping in mappings" :key="mapping.input" class="help-mapping-row"><Kbd class="keycap">{{ mapping.input }}</Kbd><ArrowRight class="arrow" :size="14" aria-hidden="true" /><strong>{{ mapping.output }}</strong><span>{{ translate('helpShiftUpper') }} {{ mapping.upper }}</span></div></div><p class="help-note">{{ translate('helpNote') }}</p></section>
  </div></main>
</template>
