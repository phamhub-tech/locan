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
        <LocChart :scans="data" class="xl:col-span-2" />
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
  TFileType,
  type ILocScanChartData,
  type ILocScanFile,
} from "../../_types";

import LocChart from "./LocChart.vue";
import {
  format,
  startOfMonth,
  startOfYear,
  subDays,
  subMonths,
} from "date-fns";
import LocFiles from "./LocFiles.vue";
import { useApiHandle } from "~/_common/core/api/composables";
import type { ScanResultModel } from "~/pages/projects/_models/scan";
import { a } from "vitest/dist/chunks/suite.B2jumIFP.js";

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
	}))
})

const selectedDuration = ref(TDuration.thisMonth);
const filteredScans = computed<[string, ScanResultModel[]] | null>(() => {
  const scans = projectScans.value;
  if (scans === null) return null;

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

  return [formatKey, filteredScans];
});
const data = computed<ILocScanChartData[]>(() => {
  const dMap: Record<string, number> = {};
  const filtered = filteredScans.value;
  if (filtered === null) return [];

  const [formatKey, scans] = filtered;
  for (const scan of scans) {
    let key = format(scan.scannedAt, formatKey);
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

getScans();
function getScans() {
  store.getProjectScans(props.uuid);
}
</script>
