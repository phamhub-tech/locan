<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
  <Toaster />
</template>

<script setup lang="ts">
import { Toaster } from "./_common/components/ui/toast";
import { useSettingsStore } from "./pages/profile/_store";

const settingsStore = useSettingsStore();
await settingsStore.init();
</script>
