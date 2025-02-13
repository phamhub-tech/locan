<template>
  <div class="space-y-6">
    <h1 class="text__h1 py-2">Projects</h1>

    <div class="flex gap-x-4 justify-between items-center">
      <div class="flex gap-x-2 items-center shrink-0">
        <Input :placeholder="$t('search')" class="max-w-72" />
        <DropdownMenu>
          <DropdownMenuTrigger as-child>
            <Button variant="outline" class="shrink-0">
              <FilterIcon />
              {{ $t("filter") }}

              <ChevronDownIcon class="ml-4 text-muted-foreground" />
            </Button>
          </DropdownMenuTrigger>
        </DropdownMenu>
      </div>

      <ProjectCreate />
    </div>

    <Projects class="border-t pt-4" />
  </div>
</template>

<script lang="ts" setup>
import { ChevronDownIcon, ListFilterIcon as FilterIcon } from "lucide-vue-next";

import { Button } from "~/_common/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuTrigger,
} from "~/_common/components/ui/dropdown-menu";
import { Input } from "~/_common/components/ui/input";

import Projects from "./_components/Projects.vue";
import ProjectCreate from "./_components/ProjectCreate.vue";

const i18n = useI18n();
definePageMeta({
  name: "projects",
  nameToTranslate: "projects",
  noAuth: true,
});
useSeoMeta({
  title: i18n.t("projects", 2),
});
</script>
