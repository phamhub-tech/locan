<template>
  <div v-if="scan">
    <p class="text-muted-foreground text-sm">
      {{ $t("ignoreExtensions.title") }}
    </p>
    <p class="text-sm">{{ $t("ignoreExtensions.desc") }}</p>
    <div class="mt-1 space-y-2">
      <div
        v-for="ext of scan.ignoreExtensions"
        :key="`ext-${ext}`"
        class="flex justify-between items-center gap-x-4"
      >
        <Input
          v-if="isEditing.state && isEditing.key === ext"
          :id="ext"
          class="h-7"
          @blur="save(ext, $event.target.value)"
          @keyup.enter="save(ext, $event.target.value)"
        />
        <p v-else class="font-mono">.{{ ext }}</p>
        <div class="flex items-center gap-x-1">
          <Button
            variant="ghost"
            size="icon"
            class="size-7"
            @click="enableEdit(ext, ext)"
          >
            <PencilIcon class="!size-4" />
          </Button>
          <Button
            variant="ghost"
            size="icon"
            class="size-7"
            @click="deleteExt(ext)"
          >
            <Trash2Icon class="size-4" />
          </Button>
        </div>
      </div>
      <Button size="sm">
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

const scan = computed(() => settings.value?.scan ?? null);

const isEditing = ref<{ state: boolean; key: string | null }>({
  state: false,
  key: null,
});

function enableEdit(key: string, value: string) {
  isEditing.value = {
    state: true,
    key,
  };

  nextTick(() => {
    const input = document.getElementById(key) as HTMLInputElement | null;

		console.log('I', input, input?.click());
		if (input) input.value = value;
    input?.click();
  });
}
function deleteExt(ext: string) {}

function save(key: string, value: string) {
  isEditing.value = {
    state: false,
    key: null,
  };
}
</script>
