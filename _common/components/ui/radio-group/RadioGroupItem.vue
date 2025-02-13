<script setup lang="ts">
import { cn } from "@/_common/utils";
import { CheckIcon, Circle } from "lucide-vue-next";
import {
  RadioGroupIndicator,
  RadioGroupItem,
  type RadioGroupItemProps,
  useForwardProps,
} from "radix-vue";
import { computed, type HTMLAttributes } from "vue";

const props = defineProps<
  RadioGroupItemProps & { class?: HTMLAttributes["class"] }
>();

const delegatedProps = computed(() => {
  const { class: _, ...delegated } = props;

  return delegated;
});

const forwardedProps = useForwardProps(delegatedProps);
</script>

<template>
  <RadioGroupItem
    v-bind="forwardedProps"
    :class="
      cn(
        'aspect-square size-5 rounded-full border ring-offset-background',
        'focus:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2',
				'disabled:cursor-not-allowed disabled:opacity-50',
				'transition-colors',
        'data-[state=checked]:border-primary data-[state=checked]:bg-primary',
        props.class,
      )
    "
  >
    <RadioGroupIndicator class="flex items-center justify-center">
      <CheckIcon class="size-4 text-white" />
    </RadioGroupIndicator>
  </RadioGroupItem>
</template>
