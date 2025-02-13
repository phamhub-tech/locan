<template>
  <div
    :class="
      cn(
        'size-10 overflow-hidden rounded-full border-2',
        $attrs.class as string | undefined,
      )
    "
  >
    <img
      v-if="src"
      :class="cn('size-full object-cover', imgClass)"
      :src="src"
      :alt="alt"
    />
    <div
      v-else-if="alt"
      class="size-full grid-centered bg-slate-100"
      role="none"
    >
      <p :class="cn('font-mono text-xl text-slate-700', fontClass)">
        {{ alt[0].toUpperCase() }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { cn } from "../utils";

defineProps<{
  src?: string | null;
  alt?: string;
  imgClass?: string;
  fontClass?: string;
}>();
</script>
