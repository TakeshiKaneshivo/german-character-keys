<script setup lang="ts">
import { computed, ref } from "vue";
import { cn } from "../../lib/utils";

const props = withDefaults(defineProps<{
  variant?: "default" | "primary" | "ghost";
  size?: "sm" | "md" | "icon";
  disabled?: boolean;
  type?: "button" | "submit" | "reset";
}>(), { variant: "default", size: "md", type: "button" });

const classes = computed(() => cn("ui-button", `ui-button-${props.variant}`, `ui-button-${props.size}`));
const button = ref<HTMLButtonElement | null>(null);
defineExpose({ focus: () => button.value?.focus() });
</script>

<template>
  <button ref="button" :class="classes" :disabled="disabled" :type="type"><slot /></button>
</template>
