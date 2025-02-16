<template>
  <div
    :class="[
      'grid-centered h-96 rounded-lg border py-4 space-y-4',
      { 'bg-background': !scans?.length },
    ]"
  >
    <LineChart
      v-if="scans?.length"
      :data="scans"
      :categories="['loc']"
      :show-legend="false"
      index="loc"
      class="h-full"
    />

    <p v-else class="text-muted-foreground">{{ $t("noScans") }}</p>
  </div>
</template>

<script setup lang="ts">
import { LineChart } from "~/_common/components/ui/chart-line";
import type { ScanResultModel } from "~/pages/projects/_models/scan";

defineProps<{ scans: ScanResultModel[] | null }>();
</script>
