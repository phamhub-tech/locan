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
      :categories="['value']"
      :show-legend="false"
      index="label"
      class="h-full"
    />
    <p v-else class="text-muted-foreground">{{ $t("noScans") }}</p>
  </div>
</template>

<script setup lang="ts">
import { LineChart } from "~/_common/components/ui/chart-line";
import type { ILocScanChartData } from "../../_types";

defineProps<{ scans: ILocScanChartData[] | null }>();
</script>
