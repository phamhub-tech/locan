<template>
  <div class="text-sm">
    <ListSetting
      v-model="ignoredExtensions"
      :title="$t('ignoreExtensions.title')"
      key-prefix="scan.ignoreExtensions"
    >
      {{ $t("ignoreExtensions.desc") }}
    </ListSetting>
  </div>
</template>

<script setup lang="ts">
import { useSettingsStore } from "../_store";

import ListSetting from "./settings/ListSetting.vue";

const store = useSettingsStore();
const { settings } = storeToRefs(store);

const scan = computed(() => settings.value!.scan ?? null);
const ignoredExtensions = ref([...scan.value!.ignoreExtensions]);
watch(ignoredExtensions, save);

const editingKey = ref<string | null>();
async function save(value: string[]) {
  const s = settings.value!;
  s.scan = {
    ...s.scan,
    ignoreExtensions: value,
  };
  await store.saveSettings(s);

  editingKey.value = null;
}
</script>
