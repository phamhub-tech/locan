<template>
  <div class="space-y-2">
    <div class="flex">
      <Button variant="secondary" size="sm" class="ml-auto" @click="resetSettings">
        <RefreshCcwIcon />
        {{ $t("reset") }}
      </Button>
    </div>

		<hr />

    <Tabs v-if="settings" default-value="scan" class="flex gap-x-4 items-start">
      <TabsList class="flex-col w-32">
        <TabsTrigger value="scan" class="w-full justify-start">
          {{ $t("scans", 2) }}
        </TabsTrigger>
      </TabsList>

      <TabsContent value="scan" class="flex-1 mt-0">
        <ScanSettings />
      </TabsContent>
    </Tabs>
  </div>
</template>

<script setup lang="ts">
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "~/_common/components/ui/tabs";

import ScanSettings from "./_components/ScanSettings.vue";
import { useSettingsStore } from "./_store";
import { Button } from "~/_common/components/ui/button";
import { RefreshCcwIcon } from "lucide-vue-next";

const store = useSettingsStore();
const { settings } = storeToRefs(store);

function resetSettings() {
	store.resetSettings()
}
</script>
