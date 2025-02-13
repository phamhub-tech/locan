<script lang="ts" setup>
import type { LabelProps } from "radix-vue";
import type { HTMLAttributes } from "vue";
import { Label } from "@/_common/components/ui/label";
import { cn } from "@/_common/utils";
import { useFormField } from "./useFormField";
import AppTooltip from "../../AppTooltip.vue";

const props = defineProps<
  LabelProps & {
    class?: HTMLAttributes["class"];
    tooltip?: string;
  }
>();

const { error, formItemId } = useFormField();
</script>

<template>
  <Label
    :class="
      cn(
        error && 'text-destructive',
        tooltip && 'flex items-center gap-x-1',
        props.class,
      )
    "
    :for="formItemId"
  >
    <slot />

    <AppTooltip v-if="tooltip" :tooltip="tooltip" />
  </Label>
</template>
