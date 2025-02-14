<template>
  <div class="text-sm rounded-lg borde p- space-y-4">
    <div class="space-y-2">
      <div
        v-for="{ loc, icon, file } of data"
        :key="`datum-${file}`"
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
            :style="{ width: `${getPercentage(loc)}%` }"
            :class="[
              'rounded absolute left-0 top-0 bottom-0 bg-primary/10',
              'transition-all scale-x-0 duration-500 origin-left',
              { 'scale-x-100': useRealData },
            ]"
            role="none"
          />
          <div class="flex items-center gap-x-2">
            <img :src="icon" class="size-6" />
            <p>{{ $t(`files.${file}`) }}</p>
          </div>
          <p class="font-medium">{{ getPercentage(loc) }}%</p>
        </div>
        <p class="font-medium w-8 text-right">{{ abbreviateCount(loc) }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { abbreviateCount } from "~/_common/utils";

enum TFileType {
  rust = "rust",
  vue = "vue",
  ts = "ts",
  json = "json",
}
interface IData {
  icon: string;
  file: TFileType;
  loc: number;
}
const data: IData[] = [
  {
    icon: "/icons/vue.svg",
    file: TFileType.vue,
    loc: 4_300,
  },
  {
    icon: "/icons/rust.svg",
    file: TFileType.rust,
    loc: 2_000,
  },
  {
    icon: "/icons/typescript.svg",
    file: TFileType.ts,
    loc: 1_300,
  },
  {
    icon: "/icons/json.svg",
    file: TFileType.json,
    loc: 400,
  },
];

const useRealData = ref(false);
let timeout: number | null = null;
onMounted(() => {
  if (timeout !== null) clearTimeout(timeout);

  timeout = window.setTimeout(() => (useRealData.value = true), 500);
});

const total = computed(() => data.reduce((prev, { loc }) => prev + loc, 0));
function getPercentage(value: number): number {
  return Math.round((value / total.value) * 100);
}
</script>
