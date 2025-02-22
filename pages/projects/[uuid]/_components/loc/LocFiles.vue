<template>
  <div class="text-sm rounded-lg borde p- space-y-4">
    <div class="space-y-1">
      <div
        v-for="(stat, i) of data"
        :key="`datum-${stat.file}`"
        class="flex items-center"
      >
        <div
          :class="[
            'relative flex-1 flex justify-between items-center',
            'bg-primary/[0.02] rounded',
            'p-3 gap-x-2',
          ]"
        >
          <div
            :style="{
              width: `${getPercentage(stat.loc)}%`,
              transitionDelay: `${i * 100}ms`,
            }"
            :class="[
              'rounded absolute left-0 top-0 bottom-0 bg-primary/10',
              'transition-all scale-x-0 duration-1000 origin-left',
              { 'scale-x-100': useRealData },
            ]"
            role="none"
          />
          <div class="flex items-center gap-x-2">
						<UnknownIcon v-if="stat.file === TFileType.unknown" class="size-6 opacity-80" />
            <img v-else :src="stat.icon" class="size-6 object-contain" />
            <div>
              <p class="font-medium">{{ capitalize(stat.file) }}</p>
              <p class="text-xs text-muted-foreground">
                .{{ stat.ext }} • {{ $t("filesCount", stat.files) }}
              </p>
            </div>
          </div>
          <p class="font-medium">{{ getPercentage(stat.loc) }}%</p>
        </div>
        <p class="font-medium w-8 text-right">
          {{ abbreviateCount(stat.loc) }}
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { capitalize } from "vue";

import { abbreviateCount } from "~/_common/utils";

import { TFileType, type ILocScanFile } from "../../_types";
import UnknownIcon from "./UnknownIcon.vue";

const props = defineProps<{ data: ILocScanFile[] }>();

const useRealData = ref(false);
let timeout: number | null = null;
onMounted(() => {
  if (timeout !== null) clearTimeout(timeout);

  timeout = window.setTimeout(() => (useRealData.value = true), 500);
});

const total = computed(() =>
  props.data.reduce((prev, { loc }) => prev + loc, 0),
);
function getPercentage(value: number): number {
  return Math.round((value / total.value) * 100);
}
</script>
