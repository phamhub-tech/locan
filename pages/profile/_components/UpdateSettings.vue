<template>
  <div class="space-y-4">
    <h2 class="text__h2">{{ $t("appUpdates") }}</h2>

    <section class="border-y py-3 flex items-center justify-between gap-x-4">
      <div class="flex-1">
        <!-- is downloading update -->
        <template v-if="apiHandle.isLoading.value">
          <p>
            {{
              isCheckingForUpdate
                ? $t("checkingUpdate")
                : $t("downloadingUpdate")
            }}
          </p>
          <div
            v-if="!isCheckingForUpdate"
            class="mt-1 h-3 rounded-[100000px] overflow-hidden bg-slate-200"
            role="progressbar"
          >
            <div
              :style="{
                width: `${updateDownloadPercentage}%`,
              }"
              class="bg-primary h-full transition-all duration-300"
              role="none"
            />
          </div>
          <p v-if="updateInfo" class="mt-2 text-sm text-muted-foreground">
            {{ $t("currentVersion", { version: appInfo!.version }) }}
          </p>
        </template>
        <!-- update downloaded but not installed -->
        <template v-else-if="apiHandle.isSuccess.value">
          <template v-if="updateInfo">
            <p>{{ $t("updateDownloaded") }}</p>
          </template>
          <p v-else>{{ $t("alreadyUpdated") }}</p>
          <p class="text-sm text-muted-foreground">
            {{ $t("currentVersion", { version: appInfo!.version }) }}
          </p>
        </template>
        <Status v-if="apiHandle.isError.value" variant="error" @retry="actOnUpdate">
          {{ apiMsg }}
        </Status>
      </div>
      <FadeTransition>
        <template v-if="!apiHandle.isError.value">
          <RefreshCw
            v-if="isCheckingForUpdate"
            class="text-muted-foreground animate-spin"
          />
          <Button
            v-else-if="!(apiHandle.isSuccess.value && !updateInfo)"
            :class="{
              'w-20 bg-opacity-15 text-foreground': apiHandle.isLoading.value,
            }"
            :disabled="apiHandle.isLoading.value"
            :loading="updateApiHandle.isLoading.value"
            @click="actOnUpdate"
          >
            {{
              apiHandle.isLoading.value
                ? `${updateDownloadPercentage}%`
                : apiHandle.isSuccess.value && updateInfo
                  ? $t("install&Restart")
                  : $t("download&Install")
            }}
          </Button>
          <Button v-else @click="store.checkAndDownloadUpdate()">
            {{ $t("checkForUpdate") }}
          </Button>
        </template>
      </FadeTransition>
    </section>

    <section v-if="updateInfo">
      <p class="text__h2">
        <span class="capitalize">{{ appInfo!.name }}</span> v{{
          updateInfo.version
        }}
      </p>
      <p v-if="updateInfo.date" class="text-sm text-muted-foreground">
        {{
          humanizeDate(
            parse(
              updateInfo.date.replace(/:\d{2}$/, ""),
              "yyyy-MM-dd HH:m:ss.SSS XXX",
              new Date(),
            ),
          )
        }}
      </p>

      <!-- div v-if="updateInfo.body" class="mt-4">
        <p class="font-semibold text-sm">{{ $t("notes") }}</p>
        <p>{{ updateInfo.body }}</p>
      </div -->
    </section>
  </div>
</template>

<script setup lang="ts">
import { RefreshCw } from "lucide-vue-next";
import { parse } from "date-fns";

import { useApiHandle } from "~/_common/core/api/composables";
import Status from "~/_common/components/Status.vue";
import { humanizeDate } from "~/_common/utils";
import FadeTransition from "~/_common/components/transitions/FadeTransition.vue";

import { useSettingsStore } from "../_store";

const store = useSettingsStore();
const {
  updateDownloadApiStatus: apiStatus,
  updateDownloadApiMsg: apiMsg,
  update: updateInfo,
  updateAppApiStatus,
  appInfo,
  updateDownloadProgress,
} = storeToRefs(store);
const apiHandle = useApiHandle(apiStatus);
const updateApiHandle = useApiHandle(updateAppApiStatus);

const isCheckingForUpdate = computed<boolean>(() => {
  return apiHandle.isLoading.value && updateInfo.value === null;
});

const updateDownloadPercentage = computed<number>(() => {
  return Math.floor((updateDownloadProgress.value ?? 0) * 100);
});

async function actOnUpdate() {
  if (apiHandle.isSuccess.value && updateInfo.value) {
    store.updateApp();
    return;
  }

  await store.checkAndDownloadUpdate();
}
</script>
