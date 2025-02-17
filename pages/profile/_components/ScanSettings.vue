<template>
  <div class="text-sm">
    <p class="text-muted-foreground font-semibold text-xs">
      {{ $t("ignoreExtensions.title") }}
    </p>
    <p class="text-sm mt-0.5">{{ $t("ignoreExtensions.desc") }}</p>
    <div class="mt-1">
      <div
        v-for="ext of ignoredExtensions"
        :key="`ext-${ext}`"
        class="flex justify-between items-center gap-x-4 group hover:bg-highlight-background"
      >
        <Input
          v-if="editingKey === getExtKey(ext)"
          :id="getExtKey(ext)"
          class="h-7 px-2"
          @blur="save(getExtKey(ext))"
          @keyup.enter="save(getExtKey(ext))"
        />
        <p v-else class="font-mono">.{{ ext }}</p>
        <div
          :class="[
            'flex items-center gap-x-0.5 group-hover:opacity-100',
            editingKey === getExtKey(ext) ? '' : 'opacity-0',
          ]"
        >
          <template v-if="editingKey === getExtKey(ext)">
            <Button size="sm" class="h-7 rounded" @click="save(getExtKey(ext))">
              {{ $t("save") }}
            </Button>
            <Button
              size="sm"
              class="h-7 rounded"
              @click="save(getExtKey(ext), false)"
            >
              {{ $t("cancel") }}
            </Button>
          </template>
          <template v-else>
            <Button
              variant="ghost"
              size="icon"
              class="size-7"
              @click="enableEdit(`scan.ignoreExtensions.${ext}`, ext)"
            >
              <PencilIcon class="!size-4" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              class="size-7"
              @click="deleteExt(`scan.ignoreExtensions.${ext}`, ext)"
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
import { Button } from "~/_common/components/ui/button";
import { useSettingsStore } from "../_store";
import { PencilIcon, PlusIcon, Trash2Icon } from "lucide-vue-next";
import { Input } from "~/_common/components/ui/input";

const store = useSettingsStore();
const { settings } = storeToRefs(store);

const scan = computed(() => settings.value!.scan ?? null);
const ignoredExtensions = ref([...scan.value!.ignoreExtensions]);

const editingKey = ref<string | null>();

function enableEdit(key: string, value: string) {
  editingKey.value = key;
  nextTick(() => {
    const input = document.getElementById(key) as HTMLInputElement | null;

    if (input) input.value = value;
    input?.click();
  });
}
function deleteExt(key: string, ext: string) {}

async function save(key: string, useValue = true) {
  if (useValue) {
    const input = document.getElementById(key) as HTMLInputElement | null;
    const value = input?.value.toLowerCase().trim();
    const ignored = ignoredExtensions.value;

    if (!value) {
      ignoredExtensions.value.pop();
    } else {
      if (ignored.includes(value)) return;
      ignoredExtensions.value[ignored.length - 1] = value;

			const s = settings.value!;
			s.scan = {
				...s.scan,
				ignoreExtensions: ignoredExtensions.value,
			}
			await store.saveSettings(s)
    }
  }

  editingKey.value = null;
}

function newItem() {
  const last = ignoredExtensions.value[ignoredExtensions.value.length - 1];
  if (last.trim() === "") return;

  ignoredExtensions.value.push("");
	enableEdit(getExtKey(""), "")
}

function getExtKey(value: string) {
  return `scan.ignoreExtensions.${value ? value : "new"}`;
}
</script>
