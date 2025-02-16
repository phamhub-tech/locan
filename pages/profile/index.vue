<template>
  <Tabs default-value="scan" class="flex gap-x-4 items-start">
    <TabsList class="flex-col w-32">
			<TabsTrigger value="scan" class="w-full justify-start">
        {{ $t("scans") }}
      </TabsTrigger>
      <TabsTrigger value="scano" class="w-full justify-start">
        {{ $t("scans") }}
      </TabsTrigger>
    </TabsList>

		<TabsContent value="scan" class="flex-1 mt-0">
			<ScanSettings />
		</TabsContent>
		<TabsContent value="scano">
			<p>Scano</p>
		</TabsContent>
  </Tabs>
</template>

<script setup lang="ts">
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/_common/components/ui/tabs";

import ScanSettings from "./_components/ScanSettings.vue";
</script>
