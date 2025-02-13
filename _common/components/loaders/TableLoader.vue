<template>
  <div class="space-y-1.5">
    <div v-for="(_, row) of rows" :key="`row-${row}`" class="flex gap-x-1">
      <TextLoader
        v-for="(_, col) of cols"
        :key="`col-${row}-${col}`"
        :style="`animation-delay: ${row * 100}ms`"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { TextLoader } from "./text";

withDefaults(defineProps<{ rows?: number; cols?: number }>(), {
  cols: 3,
  rows: 6,
});
</script>
