<template>
  <div class="text-sm space-y-6">
    <ListSetting
      v-model="ignoredExtensions"
      :title="$t('ignorePathPattern.title')"
      :placeholder="$t('excludePattern')"
      key-prefix="scan.ignorePattern"
    >
      {{ $t("ignorePathPattern.desc") }}
    </ListSetting>

    <CheckboxSetting :title="$t('useGitignore')"> </CheckboxSetting>
  </div>
</template>

<script setup lang="ts">
import { useSettingsStore } from "../_store";
import CheckboxSetting from "./settings/CheckboxSetting.vue";

import ListSetting from "./settings/ListSetting.vue";

const store = useSettingsStore();
const { settings } = storeToRefs(store);

const scan = computed(() => settings.value!.scan ?? null);
const ignoredExtensions = ref([...scan.value!.ignorePatterns]);
watch(ignoredExtensions, save);

const editingKey = ref<string | null>();
async function save(value: string[]) {
  const s = settings.value!;
  s.scan = {
    ...s.scan,
    ignorePatterns: value,
  };
  await store.saveSettings(s);

  editingKey.value = null;
}
</script>
