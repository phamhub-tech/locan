<template>
  <div>
    <table class="w-full">
      <thead>
        <tr>
          <th>{{ $t("name") }}</th>
          <th>{{ $t("loc") }}</th>
          <th>{{ $t("files", 2) }}</th>
          <th>{{ $t("scanned") }}</th>
          <th>{{ $t("created") }}</th>
        </tr>
      </thead>
      <FadeTransition mode="out-in">
        <tbody v-if="apiHandle.isError.value">
          <tr>
            <td colspan="7">
              <Status variant="error" @retry="getProjects">{{ apiMsg }}</Status>
            </td>
          </tr>
        </tbody>
        <tbody v-else-if="apiHandle.isLoading.value">
          <tr v-for="(_, row) of 5" :key="`row-${row}`">
            <td colspan="7" class="">
              <TextLoader :style="`animation-delay: ${row * 100}ms`" />
            </td>
          </tr>
        </tbody>
        <tbody v-else-if="projects" class="divide-y text-sm">
          <template v-if="projects?.length">
            <tr
              v-for="project of projects"
              :key="`project-${project.uuid}`"
              class="group"
              @click="gotoProject(project.uuid)"
            >
              <td>
                <p class="font-semibold">
                  {{ project.name }}
								</p>
								<p class="text-muted-foreground text-xs">{{  project.rootDir }}</p>
              </td>
              <td>{{ project.loc ?? "-" }}</td>
              <td>{{ project.files ?? "-" }}</td>
              <td>{{ project.lastScan ? humanizeDate(project.lastScan, false) : '-' }}</td>
              <td>{{ humanizeDate(project.createdAt, false) }}</td>
            </tr>
          </template>
          <tr v-else>
            <td colspan="4" class="text-center text-sm text-muted-foreground italic">
              {{ $t("noProjects") }}
            </td>
          </tr>
        </tbody>
      </FadeTransition>
    </table>
  </div>
</template>

<script setup lang="ts">
import { format } from "date-fns";

import { TextLoader } from "~/_common/components/loaders/text";
import { useApiHandle } from "~/_common/core/api/composables";
import Status from "~/_common/components/Status.vue";
import { getRouteName, humanizeDate } from "~/_common/utils";
import { FadeTransition } from "~/_common/components/transitions";

import { useProjectsStore } from "../_store/projects";

const store = useProjectsStore();
const {
  projectsApiStatus: apiStatus,
  projectsApiMsg: apiMsg,
  projects,
} = storeToRefs(store);
const apiHandle = useApiHandle(apiStatus);

getProjects();
function getProjects() {
  store.getProjects();
}

function gotoProject(uuid: string) {
  return navigateTo({
    name: getRouteName("project-details"),
    params: { uuid },
  });
}
</script>

<style lang="scss" scoped>
th {
  @apply text-muted-foreground text-xs text-left;

  &:not(:first-child) {
    @apply text-center !important;
  }
}

td {
  &:not(:first-child) {
    @apply text-center;
  }
}

.group:hover td {
  @apply bg-black/10 dark:bg-white/10 transition-colors cursor-pointer;
}
</style>
