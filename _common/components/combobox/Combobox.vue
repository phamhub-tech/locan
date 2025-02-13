<template>
  <div>
    <Select v-model="value">
      <SelectTrigger>
        <SelectValue :placeholder="placeholder ?? $t('select')" />
      </SelectTrigger>
      <SelectContent>
        <div class="py-3">
          <div v-if="filtered.length">
            <SelectItem
              v-for="opt in filtered"
              role="button"
              :key="`key-${opt.key}`"
              :value="opt.value"
            >
              <slot name="option" :option="opt">
                {{ opt.label }}
              </slot>
            </SelectItem>
          </div>
          <p v-else class="text-center text-muted-foreground">
            {{ $t("noItemsFound") }}
          </p>
        </div>
      </SelectContent>
    </Select>
  </div>
</template>

<script setup lang="ts">
import type { IComboOption } from "./interface";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";

const value = defineModel<string | undefined>();
const props = defineProps<{
  options: IComboOption[];
  placeholder?: string;
  searchPlaceholder?: string;
}>();

const query = ref("");

const filtered = computed<IComboOption[]>(() => {
  const q = query.value.toLowerCase().trim();
  if (!q) return props.options;

  return props.options.filter((opt) => opt.key.toLowerCase().includes(q));
});
</script>
