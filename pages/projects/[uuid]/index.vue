<template>
  <Status v-if="apiHandle.isError.value" variant="error" @retry="getProject">
    {{ apiMsg }}
  </Status>
  <div v-else-if="project" class="space-y-6">
    <div class="space-y-2">
      <section class="flex items-center justify-between">
        <h1 class="text__h1">{{ project.name }}</h1>
        <ProjectScan v-if="project" :project="project" />
      </section>

      <section class="text-sm flex border-y divide-x">
        <div class="px-4 py-2 text-center space-y-1">
          <p class="font-mono text-base">{{ project.loc ?? "--" }}</p>
          <p class="text-xs">{{ $t("loc") }}</p>
        </div>
        <div class="px-4 py-2 text-center space-y-1">
          <p class="font-mono text-base">{{ project.files ?? "--" }}</p>
          <p class="text-xs">{{ $t("files", project.files ?? 0) }}</p>
        </div>
        <div class="px-4 py-2 text-center space-y-1">
          <p class="font-mono text-base">{{ project.scans ?? "--" }}</p>
          <p class="text-xs">{{ $t("scans", project.scans ?? 0) }}</p>
        </div>
        <div class="ml-auto px-4 py-2 text-center space-y-1">
          <p class="font-mono text-base">{{ humanizeDate(new Date(), false)}}</p>
          <p class="text-xs">{{ $t("lastScan") }}</p>
        </div>
      </section>
    </div>

		<div>
			S: {{ scanResults }}
			<br />
			<LocTrend />
		</div>
  </div>
</template>

<script setup lang="ts">
import { useApiHandle } from "~/_common/core/api/composables";

import { useProjectsStore } from "../_store/projects";

import ProjectScan from "./_components/ProjectScan.vue";
import Status from "~/_common/components/Status.vue";
import { humanizeDate } from "~/_common/utils";
import LocTrend from "./_components/loc/LocTrend.vue";

definePageMeta({ name: "project-details" });

const store = useProjectsStore();
const {
  projectApiStatus: apiStatus,
  projectApiMsg: apiMsg,
  project,
	scanResults,
} = storeToRefs(store);
const apiHandle = useApiHandle(apiStatus);

const route = useRoute();
const uuid = route.params.uuid as string;
getProject();
async function getProject() {
  store.getProject(uuid);
}
</script>
