<script setup lang="ts">
import { useLanguage } from "../composables/useLanguage";
import type { Language } from "../i18n";
import { Languages } from "lucide-vue-next";
import { DropdownMenuContent, DropdownMenuItem, DropdownMenuPortal, DropdownMenuRoot, DropdownMenuTrigger } from "reka-ui";

const { options, language, abbreviation, translate, chooseLanguage, menuOpen } = useLanguage();
</script>

<template>
  <DropdownMenuRoot v-model:open="menuOpen">
    <DropdownMenuTrigger as-child>
      <button class="icon-button language-button" type="button" :aria-label="translate('languageMenuLabel')" :title="translate('languageMenuLabel')">
      <Languages :size="16" aria-hidden="true" /><span class="language-code">{{ abbreviation }}</span>
      </button>
    </DropdownMenuTrigger>
    <DropdownMenuPortal>
      <DropdownMenuContent class="menu-popover language-menu" align="end" :side-offset="8">
      <DropdownMenuItem v-for="option in options" :key="option.code" class="language-item" :class="{ selected: option.code === language }" :aria-current="option.code === language ? 'true' : undefined" @select="chooseLanguage(option.code as Language)">
        <b>{{ option.abbreviation }}</b><span>{{ translate(option.code === 'zh-CN' ? 'languageSimplified' : option.code === 'zh-TW' ? 'languageTraditional' : option.code === 'en' ? 'languageEnglish' : 'languageGerman') }}</span>
      </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenuPortal>
  </DropdownMenuRoot>
</template>
