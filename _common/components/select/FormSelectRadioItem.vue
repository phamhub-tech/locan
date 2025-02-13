<template>
  <FormItem class="cursor-pointer flex items-center space-y-0 gap-x-3 px-3.5">
    <FormControl>
      <RadioGroupItem :value="value" />
    </FormControl>
    <FormLabel class="cursor-pointer font-normal flex-1 py-2.5">
      {{ label }}
    </FormLabel>

    <AppTooltip v-if="tooltip" :tooltip="tooltip" class="ml-auto" />
  </FormItem>
</template>

<script setup lang="ts">
import AppTooltip from "../AppTooltip.vue";
import { FormControl, FormItem, FormLabel } from "../ui/form";
import { RadioGroupItem } from "../ui/radio-group";

defineProps<{ label: string; value: string, tooltip?: string }>();
</script>
