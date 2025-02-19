<template>
  <div class="flex">
    <Sidenav
      v-model="sidenavMinimised"
      class="sticky top-0 h-screen shrink-0 z-20"
    />

    <main
      :class="[
        'flex-1 pt-2 px-4 pb-20',
        'border border-black/10',
        'bg-white dark:bg-background',
      ]"
    >
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
import Sidenav from "~/_common/components/nav/Sidenav.vue";

const sidenavMinimised = ref(true);

onMounted(() => (sidenavMinimised.value = window.innerWidth < 1024));
</script>
