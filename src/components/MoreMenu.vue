<script setup lang="ts">
import { ref } from "vue";
import { MoreHorizontal } from "lucide-vue-next";
import { useLanguage } from "../composables/useLanguage";
import { DropdownMenuContent, DropdownMenuItem, DropdownMenuPortal, DropdownMenuRoot, DropdownMenuTrigger } from "reka-ui";

const emit = defineEmits<{ openHelp: [] }>();
const open = ref(false);
const { translate } = useLanguage();
</script>

<template>
  <DropdownMenuRoot v-model:open="open">
    <DropdownMenuTrigger as-child><button class="icon-button" type="button" :aria-label="translate('moreMenuLabel')" :title="translate('moreMenuLabel')"><MoreHorizontal :size="18" aria-hidden="true" /></button></DropdownMenuTrigger>
    <DropdownMenuPortal><DropdownMenuContent class="menu-popover secondary-menu" align="end" :side-offset="8">
      <DropdownMenuItem class="menu-item" @select="emit('openHelp')"><span>{{ translate('helpMenuItem') }}</span></DropdownMenuItem>
    </DropdownMenuContent></DropdownMenuPortal>
  </DropdownMenuRoot>
</template>
