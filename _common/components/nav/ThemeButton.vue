<template>
  <component :is="as" variant="ghost" size="icon" @click="toggle">
    <slot :icon="icon">
      <component :is="icon" />
    </slot>
  </component>
</template>

<script setup lang="ts">
import type { Theme } from "@tauri-apps/api/window";
import { MonitorIcon, MoonIcon, SunIcon } from "lucide-vue-next";
import { useSettingsStore } from "~/pages/profile/_store";

defineProps<{as: string | Component}>()

const store = useSettingsStore()
const { appInfo } = storeToRefs(store)

const icon = computed(() => {
  const theme = appInfo.value?.theme;

  let icon = MonitorIcon;
  if (theme === "light") icon = SunIcon;
  else if (theme === "dark") icon = MoonIcon;

  return icon;
});

function toggle() {
  const theme = appInfo.value?.theme ?? null;
  let mode: Theme | null = null;
  if (theme === null) mode = "dark";
  else if (theme === "dark") mode = "light";

	store.setTheme(mode)
}
</script>
