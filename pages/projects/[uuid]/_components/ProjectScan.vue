<template>
  <div>
    <Button
      variant="secondary"
      :loading="apiHandle.isLoading.value"
      @click="scanProject"
    >
      <ScanIcon />
      {{ $t("newScan") }}
    </Button>
  </div>
</template>

<script setup lang="ts">
import { useApiHandle } from "~/_common/core/api/composables";
import type { ProjectModel } from "../../_models/project";
import { useProjectsStore } from "../../_store/projects";
import { FolderSyncIcon as ScanIcon } from "lucide-vue-next";

const props = defineProps<{ project: ProjectModel }>();

const store = useProjectsStore();
const {
  scanProjectApiStatus: apiStatus,
  scanProjectApiMsg: apiMsg,
  scanResults,
} = storeToRefs(store);
const apiHandle = useApiHandle(apiStatus);

function scanProject() {
  store.scanProject(props.project.uuid);
}
</script>
