<script setup lang="ts">
import { inject, ref, onMounted, computed, watch } from 'vue';
import { injectionClicks, injectionClicksElements } from '@slidev/client/constants.ts';
import { parseRangeString } from '@slidev/parser/core';

const props = defineProps<{
  on: string;
  total?: string;
  class?: string;
  style?: string;
}>();

const clicks = inject(injectionClicks);
const elements = inject(injectionClicksElements);

const totalClicks = ref(elements.value.length);
const ranges = ref(parseRangeString(totalClicks, props.on));
const vClickClass = ref('slidev-vclick-hidden');

onMounted(() => {
  totalClicks.value = elements.value.length || Number(props.total) || 0;
  ranges.value = parseRangeString(totalClicks.value, props.on);
});

const show = computed(() => ranges.value.indexOf(clicks.value) !== -1);

watch(show, (newVal) => {
  if (newVal) {
    vClickClass.value = 'slidev-vclick-current';
  } else if (vClickClass.value === 'slidev-vclick-current') {
    vClickClass.value = 'slidev-vclick-prior';
  }
});

// Add this to the template for debugging: {{ clicks }} / {{ totalClicks }} in {{ ranges }} => {{ show() }}
</script>

<template>
  <div :class="[props.class, 'slidev-vclick-target', vClickClass]" :style="props.style">
    <slot />
  </div>
</template>
