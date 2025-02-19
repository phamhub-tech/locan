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
      <div class="grid 2xl:grid-cols-3 gap-4">
        <LocChart :scans="data" class="2xl:col-span-2" />
        <LocFiles :data="latestFiles" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { CalendarIcon } from "lucide-vue-next";

import { DropdownButton, DropdownItem } from "~/_common/components/dropdown";
import { useProjectsStore } from "~/pages/projects/_store/projects";

import {
  TDuration,
  type ILocScanChartData,
  type ILocScanFile,
} from "../../_types";

import LocChart from "./LocChart.vue";
import { format, startOfMonth, startOfYear } from "date-fns";
import LocFiles from "./LocFiles.vue";
import { useApiHandle } from "~/_common/core/api/composables";

const props = defineProps<{ uuid: string }>();

const store = useProjectsStore();
const {
  projectScansApiStatus: apiStatus,
  projectScansApiMsg: apiMsg,
  projectScans,
  projectScanFiles,
} = storeToRefs(store);
const apiHandle = useApiHandle(apiStatus);

const latestFiles = computed<ILocScanFile[]>(() => {
  const files = projectScanFiles.value;
  if (files === null) return [];

  return files.map((f) => ({
    icon: `/icons/${f.fileType}.svg`,
    file: f.fileType as any,
    loc: f.loc,
		files: f.files,
		ext: f.extension,
  }));
});

const selectedDuration = ref(TDuration.thisMonth);
const data = computed<ILocScanChartData[]>(() => {
  const scans = projectScans.value;
  if (scans === null) return [];

  const selected = selectedDuration.value;

  let formatKey = "MMM yyyy";
  let filteredScans = scans;
  if (selected === TDuration.thisMonth) {
    formatKey = "eee do";
    const earliest = startOfMonth(new Date());
    filteredScans = scans.filter((s) => s.scannedAt >= earliest);
  } else if (selected === TDuration.thisYear) {
    const earliest = startOfYear(new Date());
    filteredScans = scans.filter((s) => s.scannedAt >= earliest);
  }

  const dMap: Record<string, { loc: number; date: Date }> = {};
  for (const scan of filteredScans) {
    const date = scan.scannedAt;
    let key = format(date, formatKey);
    const stored = dMap[key];

    if (stored === undefined || stored.date.getTime() < date.getTime()) {
      dMap[key] = {
        loc: scan.loc,
        date,
      };
    }
  }

  return Object.entries(dMap).map(([month, { loc }]) => ({
    label: month,
    value: loc,
  }));
});

getScans();
function getScans() {
  store.getProjectScans(props.uuid);
}
</script>
