<script setup lang="ts">
import { DropdownMenuContent, DropdownMenuItem, DropdownMenuPortal, DropdownMenuRoot, DropdownMenuTrigger } from "reka-ui";
</script>

<template>
  <DropdownMenuRoot>
    <DropdownMenuTrigger as-child><slot name="trigger" /></DropdownMenuTrigger>
    <DropdownMenuPortal>
      <DropdownMenuContent class="ui-menu" align="start" :side-offset="8"><slot /></DropdownMenuContent>
    </DropdownMenuPortal>
  </DropdownMenuRoot>
</template>
