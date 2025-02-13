<template>
  <div
    class="size-10 overflow-hidden rounded-full border-2 p-[1px]"
		:style="{ borderColor: colorToHsl(color), color: colorToHsl(color, { lum: '30%'}) }"
  >
    <img
      v-if="src"
      class="size-full object-cover rounded-full"
      :src="src"
      :alt="alt"
    />
    <div
			v-else
			class="size-full grid-centered"
      role="none"
      :style="{ backgroundColor: colorToHsl(color, { opacity: '0.1' }) }"
    >
			<p class="font-mono text-xl">{{ alt[0] }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { IColor } from "../types";
import { colorToHsl } from "../utils";

defineProps<{ src?: string | null; color: IColor; alt: string }>();
</script>
