<template>
  <Button
    variant="secondary"
    :loading="apiHandle.isLoading.value"
    @click="scanProject"
  >
    <ScanIcon />
    {{ $t("newScan") }}
  </Button>
</template>

<script setup lang="ts">
import { FolderSyncIcon as ScanIcon } from "lucide-vue-next";

import { useApiHandle } from "~/_common/core/api/composables";
import { toast } from "~/_common/components/ui/toast";

import type { ProjectModel } from "../../_models/project";
import { useProjectsStore } from "../../_store/projects";

const props = defineProps<{ project: ProjectModel }>();

const store = useProjectsStore();
const {
  projectScanResultsApiStatus: apiStatus,
  projectScanResultsApiMsg: apiMsg,
} = storeToRefs(store);
const apiHandle = useApiHandle(apiStatus);

const i18n = useI18n();
async function scanProject() {
  await store.scanProject(props.project.uuid);

  if (apiHandle.isError.value) {
    toast({
      title: i18n.t("error"),
      description: apiMsg.value,
      variant: "destructive",
    });
  }
}
</script>
