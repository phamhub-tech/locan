<template>
  <div>
    <p class="text-muted-foreground font-semibold text-sm">
      {{ title }}
    </p>
    <div class="text-sm mt-0.5">
      <slot name="description" />
    </div>
    <div class="mt-1">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ title: string }>();
</script>
