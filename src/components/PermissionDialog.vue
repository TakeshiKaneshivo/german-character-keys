<script setup lang="ts">
import { nextTick, ref, watch } from "vue";
import { ExternalLink, RefreshCw, ShieldAlert, X } from "lucide-vue-next";
import { useLanguage } from "../composables/useLanguage";
import Button from "./ui/Button.vue";
const props = defineProps<{ visible: boolean; busy: boolean; feedback: string; feedbackError: boolean }>();
const emit = defineEmits<{ openSettings: []; refresh: []; quit: [] }>();
const { translate } = useLanguage();
const refreshButton = ref<InstanceType<typeof Button> | null>(null);
watch(() => props.visible, async (visible) => { if (visible) { await nextTick(); refreshButton.value?.focus(); } });
</script>

<template>
  <Transition name="permission-dialog">
  <div v-if="visible" class="permission-overlay" role="presentation">
    <section class="permission-dialog" role="dialog" aria-modal="true" aria-labelledby="permission-title" aria-describedby="permission-description">
      <div class="permission-icon"><ShieldAlert :size="18" aria-hidden="true" /></div>
      <div class="permission-copy"><h2 id="permission-title">{{ translate('permissionTitle') }}</h2><p id="permission-description">{{ translate('permissionDescription') }}</p><p class="permission-instruction">{{ translate('permissionInstruction') }}</p><p class="permission-feedback" :class="{ error: feedbackError }" aria-live="polite">{{ feedback }}</p><div class="permission-actions"><Button variant="primary" @click="emit('openSettings')"><ExternalLink :size="14" aria-hidden="true" /><span>{{ translate('openSettings') }}</span></Button><Button ref="refreshButton" :disabled="busy" @click="emit('refresh')"><RefreshCw :size="14" aria-hidden="true" /><span>{{ translate('refreshPermission') }}</span></Button><Button @click="emit('quit')"><X :size="14" aria-hidden="true" /><span>{{ translate('quitApp') }}</span></Button></div></div>
    </section>
  </div>
  </Transition>
</template>
