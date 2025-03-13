<template>
  <div class="space-y-2">
    <div class="flex">
      <h1 class="text__h1">{{ $t("settings", 2) }}</h1>

      <Button variant="outline" size="sm" class="ml-auto" @click="resetSettings">
        <RefreshCwIcon />
        {{ $t("reset") }}
      </Button>
    </div>

    <hr />

    <Tabs v-if="settings" default-value="scan" class="flex gap-x-4 items-start">
      <TabsList class="flex-col w-32">
        <TabsTrigger
          v-for="{ key, title, icon } of settingsCategories"
          :key="`setting-title-${key}`"
          :value="key"
          class="w-full justify-start [&>span]:flex [&>span]:items-center [&>span]:gap-x-2"
        >
          <component :is="icon" class="size-4 shrink-0" />
          {{ title }}
        </TabsTrigger>
      </TabsList>

      <TabsContent
        v-for="{ key, content } of settingsCategories"
        :key="`setting-content-${key}`"
        :value="key"
        class="flex-1 mt-0"
      >
        <component :is="content" />
      </TabsContent>
    </Tabs>
  </div>
</template>

<script setup lang="ts">
import { RefreshCwIcon, CloudDownloadIcon } from "lucide-vue-next";

import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "~/_common/components/ui/tabs";
import { SettingsIcon } from "~/_common/components/icons";

import ScanSettings from "./_components/ScanSettings.vue";
import { useSettingsStore } from "./_store";
import UpdateSettings from "./_components/UpdateSettings.vue";

const store = useSettingsStore();
const { settings } = storeToRefs(store);

const i18n = useI18n();

interface ISettingsCategory {
  icon: Component;
  key: string;
  title: string;
  content: Component;
}
const settingsCategories: ISettingsCategory[] = [
  {
    icon: SettingsIcon,
    key: "scan",
    title: i18n.t("scans", 2),
    content: ScanSettings,
  },
  {
    icon: CloudDownloadIcon,
    key: "update",
    title: i18n.t("update"),
    content: UpdateSettings,
  },
];

function resetSettings() {
  store.resetSettings();
}
</script>
