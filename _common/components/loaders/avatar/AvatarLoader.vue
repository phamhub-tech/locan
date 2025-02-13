<template>
  <Skeleton
    :class="
      cn(
        'size-10 overflow-hidden rounded-full',
        $attrs.class as string | undefined,
      )
    "
  />
</template>

<script setup lang="ts">
import { cn } from "~/_common/utils";
import Skeleton from "../../ui/skeleton/Skeleton.vue";
</script>
