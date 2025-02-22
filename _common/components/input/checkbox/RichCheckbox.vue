<template>
  <div class="flex gap-x-2">
    <Checkbox v-model="checked" :id="id" />
    <div class="grid gap-1.5 leading-none">
      <label :for="id" class="">{{ label }}</label>
      <p class="">
        <slot />
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import Checkbox from "../../ui/checkbox/Checkbox.vue";

defineProps<{ id: string; label: string }>();
const checked = defineModel<boolean>({ required: true });
</script>
