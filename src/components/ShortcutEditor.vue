<script setup lang="ts">
import { RotateCcw } from "lucide-vue-next";
import { useLanguage } from "../composables/useLanguage";
import Button from "./ui/Button.vue";
import Kbd from "./ui/Kbd.vue";

defineProps<{ value: string; recording: boolean; message: string; messageError: boolean; busy: boolean }>();
const emit = defineEmits<{ focus: []; blur: []; keydown: [event: KeyboardEvent]; reset: [] }>();
const { translate } = useLanguage();
</script>

<template>
  <section class="command-card">
    <div class="command-card-copy"><span class="section-kicker">{{ translate('shortcutKicker') }}</span><div class="command-title-row"><Kbd>⌘</Kbd><h2>{{ translate('globalShortcut') }}</h2></div><p>{{ translate('shortcutDescription') }}</p></div>
    <div class="shortcut-controls">
      <div class="shortcut-row">
        <input :value="value" class="shortcut-input" readonly :class="{ recording }" :aria-label="translate('shortcutAriaLabel')" @focus="emit('focus')" @click="emit('focus')" @keydown="emit('keydown', $event)" @blur="emit('blur')" />
        <Button :disabled="busy" size="sm" @click="emit('reset')"><RotateCcw :size="14" aria-hidden="true" /><span>{{ translate('reset') }}</span></Button>
      </div>
      <p class="hint" :class="{ error: messageError }" aria-live="polite">{{ message || translate('shortcutHint') }}</p>
    </div>
  </section>
</template>
