<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { cn } from "@/_common/utils";

interface SkeletonProps {
  class?: HTMLAttributes["class"];
}

const props = defineProps<SkeletonProps>();
</script>

<template>
  <div
    :class="
      cn('animate-pulse rounded-md bg-slate-200 !duration-1000', props.class)
    "
  />
</template>
