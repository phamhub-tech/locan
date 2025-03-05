<template>
  <div>
    <p class="text-muted-foreground font-semibold text-sm">
      {{ title }}
    </p>
    <div class="mt-0.5">
      <slot />
    </div>
    <div class="mt-1">
      <div
        v-for="(entry, i) of settingToUse"
        :key="getEntryKey(entry)"
        class="flex justify-between items-center gap-x-4 group hover:bg-highlight-background"
      >
        <Input
          v-if="editingKey === getEntryKey(entry)"
          :id="getEntryKey(entry)"
          :placeholder="placeholder"
          class="h-7 px-2"
          @blur="save(getEntryKey(entry), i)"
          @keyup.enter="save(getEntryKey(entry), i)"
        />
        <p v-else class="font-mono">{{ entry }}</p>
        <div
          :class="[
            'flex items-center gap-x-0.5 group-hover:opacity-100',
            editingKey === getEntryKey(entry) ? '' : 'opacity-0',
          ]"
        >
          <template v-if="editingKey === getEntryKey(entry)">
            <Button
              size="sm"
              class="h-7 rounded"
              @click="save(getEntryKey(entry), i)"
            >
              {{ $t("save") }}
            </Button>
            <Button
              size="sm"
              class="h-7 rounded"
              @click="save(getEntryKey(entry), i, false)"
            >
              {{ $t("cancel") }}
            </Button>
          </template>
          <template v-else>
            <Button
              variant="ghost"
              size="icon"
              class="size-7"
              @click="enableEdit(getEntryKey(entry), entry)"
            >
              <PencilIcon class="!size-4" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              class="size-7"
              @click="deleteAt(i)"
            >
              <Trash2Icon class="size-4" />
            </Button>
          </template>
        </div>
      </div>

      <Button size="sm" class="mt-2" @click="newItem">
        <PlusIcon />
        {{ $t("addItem") }}
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { PencilIcon, PlusIcon, Trash2Icon } from "lucide-vue-next";

const props = defineProps<{
  title: string;
  keyPrefix: string;
  placeholder?: string;
}>();
const setting = defineModel<string[]>({ required: true });

const editingKey = ref<string | null>();
const settingToUse = ref<string[]>(Array.from(setting.value));
const settingToUseLast = computed<string | undefined>(() => {
  const s = settingToUse.value;
  if (s.length) return settingToUse.value[s.length - 1];

  return;
});

function getEntryKey(value: string) {
  return `${props.keyPrefix}.${value ? value : "new"}`;
}

function newItem() {
  if (settingToUseLast.value?.trim() === "") return;

  settingToUse.value.push("");
  enableEdit(getEntryKey(""), "");
}

function save(key: string, index: number, useValue = true) {
  if (useValue) {
    const input = document.getElementById(key) as HTMLInputElement | null;
    const value = input?.value.toLowerCase().trim();
    const s = setting.value;

    if (!value) {
      if (!settingToUse.value[settingToUse.value.length - 1])
        settingToUse.value.pop();
    } else {
      // Don't save if trying to add an already existing item
      if (s.includes(value)) return;

      settingToUse.value[index] = value;

      // Create a new array so that listeners can be triggered
      setting.value = Array.from(settingToUse.value);
    }
  }

  editingKey.value = null;
}

function enableEdit(key: string, value: string) {
  editingKey.value = key;
  nextTick(() => {
    const input = document.getElementById(key) as HTMLInputElement | null;

    if (input) input.value = value;
    input?.click();
  });
}
function deleteAt(index: number) {
  settingToUse.value.splice(index, 1);
  setting.value = settingToUse.value;
}
</script>
