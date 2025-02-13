<template>
  <div>
    <table class="w-full">
      <thead>
        <tr>
          <th>{{ $t("name") }}</th>
          <th>{{ $t("loc") }}</th>
          <th>{{ $t("updated") }}</th>
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
              :key="`project-${project.id}`"
              class="group"
              @click="gotoProject(project.id)"
            >
              <td>
                <div class="flex items-center gap-x-3 font-semibold">
                  {{ project.name }}
                </div>
              </td>
              <td>{{ project.loc ?? "-" }}</td>
              <td>{{ format(project.updatedAt, "dd MMM yyyy") }}</td>
              <td>{{ format(project.createdAt, "dd MMM yyyy") }}</td>
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
import {
  ArchiveIcon,
  CopyIcon,
  EllipsisIcon,
  PencilIcon,
  Trash2Icon,
} from "lucide-vue-next";
import { format } from "date-fns";

import { TextLoader } from "~/_common/components/loaders/text";
import { Button } from "~/_common/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuTrigger,
} from "~/_common/components/ui/dropdown-menu";
import { useApiHandle } from "~/_common/core/api/composables";
import Status from "~/_common/components/Status.vue";
import DropdownMenuContent from "~/_common/components/ui/dropdown-menu/DropdownMenuContent.vue";
import DropdownMenuItem from "~/_common/components/ui/dropdown-menu/DropdownMenuItem.vue";
import { Label } from "~/_common/components/ui/label";
import { getRouteName } from "~/_common/utils";
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

function gotoProject(id: number) {
  return navigateTo({
    name: getRouteName("project-details"),
    params: { id },
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
