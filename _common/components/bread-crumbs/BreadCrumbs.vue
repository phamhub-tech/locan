<template>
  <div
    :class="['flex items-center my-3 rounded-md text-xs', 'w-max text-sm mb-3']"
  >
    <span class="opacity-60">{{ $t("home") }}</span>
    <ChevronRightIcon :size="18" class="mx-2 opacity-60 shrink-0" role="none" />
    <div
      v-for="(route, i) of routes"
      :key="`bc-${i}`"
      class="flex items-center -translate-x-3 opacity-60 last:opacity-100 last:font-semibold"
    >
      <RouterLink
        exact-active-class="!opacity-100"
        :class="
          cn(
            'px-3 py-1 transition-[background-color,opacity] duration-300 rounded-full opacity-60',
            'hover:bg-slate-200 hover:opacity-80',
            {
              'grid place-items-center w12 aspect-square translate-y-[0.125rem]':
                route.isHome,
            },
            {
              'bg-slate-100 px-2 py-2 rounded opacity-100 hover:opacity-100':
                route.isHome,
            },
          )
        "
        :to="route"
      >
        <span>{{ $t(route.nameToTranslate ?? route.name) }}</span>
      </RouterLink>
      <ChevronRightIcon
        v-if="i < routes.length - 1"
        :size="18"
        class="mx-2 opacity-60 shrink-0"
        role="none"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ChevronRightIcon } from "lucide-vue-next";
import { computed } from "vue";
import { useRoute, type RouteParams } from "vue-router";

import { cn } from "~/_common/utils";

const currentRoute = useRoute();
interface ICrumb {
  name: string;
  path: string;
  nameToTranslate?: string;
  isHome?: boolean;
  params?: RouteParams;
}
const routes = computed<ICrumb[]>(() => {
  const ancestors: Record<string, ICrumb> = {};

  console.log(currentRoute.matched.length);
  for (const route of currentRoute.matched) {
    let toAdd: ICrumb | null = null;
    const name = route.name?.toString();

    if (name) {
      toAdd = {
        path: route.path,
        name,
        nameToTranslate: route.meta.nameToTranslate as string | undefined,
      };
    } else {
      for (const child of route.children) {
        const childName = child.name?.toString();
        if (childName && child.path === "") {
          toAdd = {
            path: route.path,
            name: childName,
            nameToTranslate: child.meta?.nameToTranslate as string | undefined,
          };
          break;
        }
      }
    }

    if (toAdd) {
      const toAddPath = toAdd.path;
      let params: ICrumb["params"] | null = null;
      for (const match of toAddPath.matchAll(/:(?<name>\w+)/g)) {
        const { name } = match.groups!;
        if (params === null) params = {};

        const param = currentRoute.params[name];
        params[name] = param;
      }

      // Add path parameters
      if (params) toAdd.params = params;

      toAdd = {
        ...toAdd,
        isHome: toAdd.name === "home",
      };

      ancestors[toAdd.name] = toAdd;
    } else console.error("Could not add route to breadcrumb:", route);
  }

  return Object.values(ancestors);
});
</script>
