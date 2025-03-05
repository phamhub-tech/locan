<template>
  <div class="text-sm space-y-6">
    <ListSetting
      v-model="ignoredPatterns"
      :title="$t('ignorePathPattern.title')"
      :placeholder="$t('excludePattern')"
      key-prefix="scan.ignorePattern"
    >
      {{ $t("ignorePathPattern.desc") }}
    </ListSetting>

    <Setting :title="$t('useGitignore.title')">
      <RichCheckbox
        v-model="useGitignore"
        id="useGitignore"
        :label="$t('useGitignore.desc')"
      />
    </Setting>
  </div>
</template>

<script setup lang="ts">
import { RichCheckbox } from "~/_common/components/input";

import { ListSetting, Setting } from "~/_common/components/settings"
import { useSettingsStore } from "../_store";

const store = useSettingsStore();
const { settings } = storeToRefs(store);

const scan = computed(() => settings.value!.scan ?? null);
const ignoredPatterns = ref([...scan.value!.ignorePatterns]);
watch(ignoredPatterns, () => save());

const useGitignore = ref(scan.value!.useGitignore);
watch(useGitignore, () => save());

async function save() {
  const s = settings.value!;
  s.scan = {
    ignorePatterns: ignoredPatterns.value,
    useGitignore: useGitignore.value,
  };
  await store.saveSettings(s);
}
</script>
