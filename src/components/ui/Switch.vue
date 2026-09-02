<script setup lang="ts">
import { SwitchRoot, SwitchThumb } from "reka-ui";

defineProps<{ modelValue?: boolean; disabled?: boolean; ariaLabel: string }>();
const emit = defineEmits<{ "update:modelValue": [value: boolean] }>();
</script>

<template>
  <SwitchRoot :model-value="modelValue" :disabled="disabled" :aria-label="ariaLabel" class="ui-switch" @update:model-value="emit('update:modelValue', $event)">
    <SwitchThumb class="ui-switch-thumb" />
  </SwitchRoot>
</template>
