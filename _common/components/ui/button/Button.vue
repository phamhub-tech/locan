<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { Primitive, type PrimitiveProps } from "radix-vue";

import { cn } from "@/_common/utils";

import { type ButtonVariants, buttonVariants } from ".";

interface Props extends PrimitiveProps {
  variant?: ButtonVariants["variant"];
  size?: ButtonVariants["size"];
  class?: HTMLAttributes["class"];
  loading?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  as: "button",
});
</script>

<template>
  <Primitive
    :as="as"
    :as-child="asChild"
    :class="
      cn(buttonVariants({ variant, size }), { loading: loading }, props.class)
    "
    :data-primary="variant === 'primary'"
  >
    <slot />
  </Primitive>
</template>

<style lang="css" scoped>
.btn {
  @apply relative transition-colors overflow-hidden;

  &:after {
    content: "";
    transform: translate(-50%, -50%);
    pointer-events: none;
    @apply transition-opacity;
    @apply opacity-0 absolute left-1/2 top-1/2 size-6;
    @apply rounded-full border-2 border-white/20 border-t-white;
  }

  &[data-primary="true"]:before {
    content: "";
    @apply absolute inset-0 bg-gradient-to-b from-transparent to-black/20;
  }

  &.loading {
    @apply grayscale text-transparent pointer-events-none;

    &:after {
      animation: spin 1s linear infinite;
      @apply opacity-100;
    }
  }
}

@keyframes spin {
  to {
    transform: translate(-50%, -50%) rotate(360deg);
  }
}
</style>
