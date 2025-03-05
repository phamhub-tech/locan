<template>
  <Modal :title="$t('projectSettings')">
    <template #trigger>
      <Button variant="outline" size="icon">
        <SettingsIcon />
      </Button>
    </template>

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

		<template #footer="{ close }">
      <Button @click="close">{{ $t("done") }}</Button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { SlidersVerticalIcon as SettingsIcon } from "lucide-vue-next";

import Modal from "~/_common/components/Modal.vue";
import { useProjectsStore } from "../../_store/projects";
import { ListSetting, Setting } from "~/_common/components/settings";
import RichCheckbox from "~/_common/components/input/checkbox/RichCheckbox.vue";

const store = useProjectsStore();
const { projectSettings: settings, project } = storeToRefs(store);

const ignoredPatterns = ref(
  settings.value!.ignorePatterns ? [...settings.value!.ignorePatterns] : [],
);
watch(ignoredPatterns, () => save());

const useGitignore = ref(settings.value!.useGitignore ?? false);
watch(useGitignore, () => save());

async function save() {
  let ignored: string[] | undefined = ignoredPatterns.value;
  if (ignored.length === 0) ignored = undefined;

  await store.saveSettings(project.value!.rootDir, {
    ignore_patterns: ignored,
    use_gitignore: useGitignore.value,
  });
}
</script>
