<script setup lang="ts">
import { Palette, Check } from "lucide-vue-next";
import { DropdownMenuContent, DropdownMenuItem, DropdownMenuPortal, DropdownMenuRoot, DropdownMenuTrigger } from "reka-ui";
import { useLanguage } from "../composables/useLanguage";
import { useTheme, type ThemeMode } from "../composables/useTheme";

const { translate } = useLanguage();
const { theme, setTheme } = useTheme();
const modes: Array<{ value: ThemeMode; label: "themeSystem" | "themeLight" | "themeDark" }> = [
  { value: "system", label: "themeSystem" },
  { value: "light", label: "themeLight" },
  { value: "dark", label: "themeDark" },
];
</script>

<template>
  <DropdownMenuRoot>
    <DropdownMenuTrigger as-child>
      <button class="icon-button theme-button" type="button" :aria-label="translate('themeMenuLabel')" :title="translate('themeMenuLabel')">
        <Palette :size="16" aria-hidden="true" />
      </button>
    </DropdownMenuTrigger>
    <DropdownMenuPortal>
      <DropdownMenuContent class="menu-popover theme-menu" align="end" :side-offset="8">
        <DropdownMenuItem
          v-for="mode in modes"
          :key="mode.value"
          class="theme-item"
          :class="{ selected: mode.value === theme }"
          @select="setTheme(mode.value)"
        >
          <span>{{ translate(mode.label) }}</span><Check v-if="mode.value === theme" :size="14" aria-hidden="true" />
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenuPortal>
  </DropdownMenuRoot>
</template>
