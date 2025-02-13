<template>
  <div
    v-if="isFullReload && apiHandle.isLoading.value"
    class="p-3 pt-20 text-center"
  >
    <SpinLoader class="mx-auto" />
    <p v-if="loadingText?.trim()">{{ loadingText }}</p>
  </div>
  <Status
    v-else-if="apiHandle.isError.value"
    variant="error"
    class="m-3"
    @retry="onRetry"
  >
    {{ apiMsg }}
  </Status>
  <div
    v-bind="attrsToBind"
    :class="
      twMerge('space-y-6 md:space-y-8', $attrs.class as string | undefined)
    "
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { useAttrs } from "vue";
import { twMerge } from "tailwind-merge";

import { removeFromAttrs } from "~/_common/utils";
import { useApiHandle } from "~/_common/core/api/composables";

import Status from "../Status.vue";

import SpinLoader from "./SpinLoader.vue";

defineOptions({ inheritAttrs: false });
withDefaults(
  defineProps<{
    apiHandle: ReturnType<typeof useApiHandle>;
    apiMsg: string;
    loadingText?: string;
    isFullReload?: boolean;
    onRetry?: () => void;
  }>(),
  {
    isFullReload: true,
  },
);

const attrs = useAttrs();
const attrsToBind = removeFromAttrs(attrs, "class");
</script>
