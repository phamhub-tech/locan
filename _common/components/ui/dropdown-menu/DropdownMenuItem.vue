<script setup lang="ts">
import { cn } from "@/_common/utils";
import {
  DropdownMenuItem,
  type DropdownMenuItemProps,
  useForwardProps,
} from "radix-vue";
import { computed, type HTMLAttributes } from "vue";

const props = defineProps<
  DropdownMenuItemProps & { class?: HTMLAttributes["class"]; inset?: boolean }
>();

const delegatedProps = computed(() => {
  const { class: _, ...delegated } = props;

  return delegated;
});

const forwardedProps = useForwardProps(delegatedProps);
</script>

<template>
  <DropdownMenuItem
    v-bind="forwardedProps"
    :class="
      cn(
        'opacity-80 relative flex cursor-default select-none items-center rounded-sm',
        'gap-2 px-2 h-10 text-sm outline-none transition-colors',
        'focus:bg-accent focus:text-accent-foreground',
        'data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
				'[&>svg]:size-4 [&>svg]:shrink-0 [&>svg]:opacity-60',
        inset && 'pl-8',
        props.class,
      )
    "
  >
    <slot />
  </DropdownMenuItem>
</template>
