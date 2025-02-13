<template>
  <Skeleton
    :class="cn(textVariants({ size }), $attrs.class as string | undefined)"
  />
</template>

<script setup lang="ts">
import { cn } from "~/_common/utils";

import { textVariants, type TextLoaderVariants } from ".";
import { Skeleton } from "../../ui/skeleton";

defineProps<{
  size?: TextLoaderVariants["size"];
}>();
</script>
