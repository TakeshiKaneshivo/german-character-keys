<script setup lang="ts">
import { ArrowRight } from "lucide-vue-next";
import { useLanguage } from "../composables/useLanguage";
import Kbd from "./ui/Kbd.vue";
import { motion, useReducedMotion } from "motion-v";
const mappings = [{ input: "[", output: "ü" }, { input: "'", output: "ä" }, { input: ";", output: "ö" }, { input: "-", output: "ß" }];
const { translate } = useLanguage();
const reducedMotion = useReducedMotion();
</script>

<template>
  <section class="mapping-section">
    <div class="section-heading"><div><span class="section-kicker">{{ translate('mappingKicker') }}</span><h2>{{ translate('currentMapping') }}</h2></div><span class="mapping-count">{{ translate('mappingCount') }}</span></div>
    <div class="mapping-grid"><motion.div v-for="(mapping, index) in mappings" :key="mapping.input" class="mapping-item" :initial="reducedMotion ? false : { opacity: 0, y: 5 }" :animate="{ opacity: 1, y: 0 }" :transition="{ duration: reducedMotion ? 0 : 0.18, delay: reducedMotion ? 0 : index * 0.025 }"><Kbd class="keycap">{{ mapping.input }}</Kbd><ArrowRight class="arrow" :size="14" aria-hidden="true" /><strong class="output-key">{{ mapping.output }}</strong></motion.div></div>
  </section>
</template>
