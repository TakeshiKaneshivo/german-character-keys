<script setup lang="ts">
import { computed, watchEffect } from "vue";
import { openAccessibilitySettings, openHelpWindow, quitApp } from "./tauri";
import { useAppStatus } from "./composables/useAppStatus";
import { useLanguage } from "./composables/useLanguage";
import { useShortcutRecorder } from "./composables/useShortcutRecorder";
import LanguageMenu from "./components/LanguageMenu.vue";
import ThemeMenu from "./components/ThemeMenu.vue";
import MoreMenu from "./components/MoreMenu.vue";
import ShortcutEditor from "./components/ShortcutEditor.vue";
import MappingGrid from "./components/MappingGrid.vue";
import PermissionDialog from "./components/PermissionDialog.vue";
import { Keyboard, Power, ShieldCheck } from "lucide-vue-next";
import Badge from "./components/ui/Badge.vue";
import Switch from "./components/ui/Switch.vue";
import { useTheme } from "./composables/useTheme";

const { status, error, operationMessage, busy, blocked, permissionFeedback, setEnabled, setLaunchAtLogin, setShortcut, resetShortcut, checkPermission } = useAppStatus();
const { translate } = useLanguage();
useTheme();
const committedShortcut = () => status.value?.shortcut ?? "";
const recorder = useShortcutRecorder(committedShortcut, async (shortcut) => { await setShortcut(shortcut); });
const shortcutValue = computed(() => recorder.displayValue.value);
const shortcutRecording = computed(() => recorder.recording.value);
const shortcutMessage = computed(() => error.value || operationMessage.value || recorder.message.value);
const shortcutMessageError = computed(() => recorder.messageError.value || feedbackError.value);
const statusText = computed(() => error.value || operationMessage.value || (status.value?.enabled ? translate("statusEnabled") : translate("statusDisabled")));
const statusDetail = computed(() => status.value?.enabled ? translate("statusEnabledFull") : translate("statusDisabledFull"));
const feedbackError = computed(() => Boolean(error.value));

watchEffect(() => { document.title = translate("appTitle"); });

async function toggleEnabled(value: boolean) { await setEnabled(value); }
async function toggleLaunch(event: Event) { await setLaunchAtLogin((event.target as HTMLInputElement).checked); }
async function reset() { await resetShortcut(); recorder.cancel(); }
async function openHelp() { try { await openHelpWindow(); } catch (reason) { error.value = String(reason); } }
async function openSettings() { try { await openAccessibilitySettings(); } catch (reason) { error.value = String(reason); } }
async function closeApp() { try { await quitApp(); } catch (reason) { error.value = String(reason); } }
</script>

<template>
  <main class="app-shell">
    <div class="app-window" :inert="blocked" :aria-hidden="blocked ? 'true' : undefined">
      <header class="app-header">
        <div class="brand-lockup">
          <div class="brand-mark"><Keyboard :size="18" aria-hidden="true" /></div>
          <div class="brand-copy"><span class="eyebrow">{{ translate('brandEyebrow') }}</span><h1>{{ translate('appTitle') }}</h1><p class="subtitle">{{ translate('appSubtitle') }}</p></div>
        </div>
        <div class="header-actions"><ThemeMenu /><LanguageMenu /><MoreMenu @open-help="openHelp" /></div>
      </header>
      <section class="status-strip" :class="{ 'is-on': status?.enabled }">
        <div class="status-copy"><div class="status-orb"><ShieldCheck :size="15" aria-hidden="true" /></div><div><p class="status-title" role="status" aria-live="polite">{{ status?.enabled ? translate('statusOn') : translate('statusOff') }}</p><p class="status-detail" :title="statusDetail">{{ statusText }}</p></div></div>
        <div class="status-control"><Badge :tone="status?.enabled ? 'success' : 'neutral'"><span class="status-dot" aria-hidden="true"></span>{{ status?.enabled ? translate('statusOn') : translate('statusOff') }}</Badge><Switch :model-value="Boolean(status?.enabled)" :disabled="!status || busy" :aria-label="translate('toggleMappingLabel')" @update:model-value="toggleEnabled" /></div>
      </section>
      <ShortcutEditor :value="shortcutValue" :recording="shortcutRecording" :message="shortcutMessage" :message-error="shortcutMessageError" :busy="busy" @focus="recorder.start" @blur="recorder.cancel" @keydown="recorder.capture" @reset="reset" />
      <MappingGrid />
      <footer class="app-footer"><label class="login-option"><input type="checkbox" :checked="status?.launch_at_login" :disabled="!status || busy" :aria-label="translate('autoLaunch')" @change="toggleLaunch" /><span>{{ translate('autoLaunch') }}</span></label><span class="platform-label">{{ status?.platform }}</span><Power :size="14" class="footer-icon" aria-hidden="true" /></footer>
    </div>
    <PermissionDialog :visible="Boolean(blocked)" :busy="busy" :feedback="permissionFeedback || error" :feedback-error="feedbackError" @open-settings="openSettings" @refresh="checkPermission" @quit="closeApp" />
  </main>
</template>
