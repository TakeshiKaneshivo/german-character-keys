<script setup lang="ts">
defineProps<{ tone?: "neutral" | "success" | "warning" }>();
</script>

<template><span class="ui-badge" :class="`ui-badge-${tone ?? 'neutral'}`"><slot /></span></template>
