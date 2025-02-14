<template>
  <div class="space-y-2">
    <div class="flex justify-en items-center gap-x-2">
      <h2 class="font-semibold">{{ $t("scans", 2) }}</h2>
      <DropdownButton
        :icon="CalendarIcon"
        :label="$t(selectedDuration)"
        class="ml-auto"
      >
        <DropdownItem
          v-for="duration of Object.values(TDuration)"
          :value="duration"
          @click="selectedDuration = duration"
        >
          {{ $t(duration) }}
        </DropdownItem>
      </DropdownButton>
    </div>

    <div class="space-y-4">
      <div class="grid xl:grid-cols-3 gap-4">
        <LocChart :data="data" class="xl:col-span-2" />
        <LocFiles />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  CalendarIcon,
  TrendingUpIcon,
  TrendingDownIcon,
} from "lucide-vue-next";

import { DropdownButton, DropdownItem } from "~/_common/components/dropdown";

import { TDuration, type ILocScanChartData } from "../../_types";

import LocChart from "./LocChart.vue";
import {
  format,
  startOfMonth,
  startOfYear,
  subDays,
  subMonths,
} from "date-fns";
import LocFiles from "./LocFiles.vue";

const scans = [
  {
    loc: 506,
    date: subMonths(new Date(), 2),
  },
  {
    loc: 606,
    date: subMonths(new Date(), 2),
  },
  {
    loc: 816,
    date: subMonths(new Date(), 1),
  },
  {
    loc: 710,
    date: subDays(new Date(), 9),
  },
  {
    loc: 730,
    date: subDays(new Date(), 2),
  },
  {
    loc: 717,
    date: subDays(new Date(), 1),
  },
  {
    loc: 745,
    date: new Date(),
  },
];

const selectedDuration = ref(TDuration.thisMonth);
const data = computed<ILocScanChartData[]>(() => {
  const selected = selectedDuration.value;

  let formatKey = "MMM yyyy";
  let filteredScans = scans;
  if (selected === TDuration.thisMonth) {
    formatKey = "eee do";
    const earliest = startOfMonth(new Date());
    filteredScans = scans.filter((s) => s.date >= earliest);
  } else if (selected === TDuration.thisYear) {
    const earliest = startOfYear(new Date());
    filteredScans = scans.filter((s) => s.date >= earliest);
  }

  const dMap: Record<string, number> = {};
  for (const scan of filteredScans) {
    let key = format(scan.date, formatKey);
    const loc = dMap[key] as number | undefined;

    if (loc === undefined) {
      dMap[key] = scan.loc;
    } else {
      dMap[key] = Math.max(loc, scan.loc);
    }
  }

  return Object.entries(dMap).map(([month, value]) => ({
    label: month,
    value: value,
  }));
});
</script>
