<script setup lang="ts">
const props = defineProps(['src']);
</script>

<template>
  <div class="abs-br m-6 flex gap-2">
    <img :src="src" />
  </div>
</template>
