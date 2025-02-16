<template>
  <component :is="as" variant="ghost" size="icon" @click="toggle">
    <slot :icon="icon">
      <component :is="icon" />
    </slot>
  </component>
</template>

<script setup lang="ts">
import { MonitorIcon, MoonIcon, SunIcon } from "lucide-vue-next";

defineProps<{as: string | Component}>()

const colorMode = useColorMode();
const icon = computed(() => {
  const pref = colorMode.preference;

  let icon = MonitorIcon;
  if (pref === "light") icon = SunIcon;
  else if (pref === "dark") icon = MoonIcon;

  return icon;
});

function toggle() {
  const pref = colorMode.preference;
  let mode = "system";
  if (pref === "system") mode = "dark";
  else if (pref === "dark") mode = "light";

  colorMode.preference = mode;
}
</script>
