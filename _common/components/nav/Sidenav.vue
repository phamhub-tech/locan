<template>
  <aside class="group duration-300 w-60">
    <div
      :class="[
        'relative flex flex-col h-full w-4/5 max-w-xs transition-transform duration-inherit',
        'md:w-full md:max-w-none',
        'dark:bg-surface md:dark:bg-surface-dark/50 md:dark:border-transparent space-y-6',
        'dark:bg-toolbar-background',
        'p-4',
      ]"
    >
      <Transition name="fade-fast" mode="out-in">
        <div class="flex-1 flex flex-col text-sm gap-y-4 max-sm:mt-3">
          <div
            v-for="(group, g) of groupedRoutes"
            :key="`sidenav-link-group-${g}`"
            class="space-y-1 first:pt-0"
          >
            <div v-if="group.title" class="relative">
              <h3
                :class="[
                  'pointer-events-none',
                  'inline-block text-xsm smallcaps whitespace-nowrap transition-[opacity,transform] duration-300',
                  isMinimised
                    ? 'opacity-0 translate-x-0 absolute'
                    : 'translate-x-5 opacity-60',
                ]"
              >
                {{ group.title }}
              </h3>
              <span
                :class="[
                  'inline-block transition-[opacity,transform] duration-300',
                  isMinimised
                    ? 'opacity-60 translate-x-4'
                    : 'opacity-0 absolute top-0 translate-x-10',
                ]"
              >
                ...
              </span>
            </div>
            <RouterLink
              v-for="(r, i) of group.routes"
              v-slot="{ isActive, isExactActive, navigate }"
              :key="`route-${i}`"
              :to="r.route"
              :exact-active-class="linkActiveClass"
              :active-class="linkActiveClass"
              custom
            >
              <div
                :class="[
                  'relative cursor-pointer flex items-center space-x-3 border border-transparent',
                  'hover:bg-black/5 overflow-hidden rounded-[6px] p-2 transition-all',
                  '',
                  {
                    [linkActiveClass]: r.useExact ? isExactActive : isActive,
                  },
                ]"
                @click="
                  () => {
                    navigate();
                    close(true);
                  }
                "
              >
                <component :is="r.icon" class="w-[21px] shrink-0 opacity-60" />
                <span class="flex-1 whitespace-nowrap font-semibold">
                  {{ r.name }}
                </span>
              </div>
            </RouterLink>
          </div>

          <hr class="mt-auto" />
          <div class="space-y-6">
            <div class="space-y-1">
              <ThemeButton
                v-slot="{ icon }"
                as="div"
                :class="[
                  'relative cursor-pointer flex items-center space-x-3 border border-transparent',
                  ' overflow-hidden rounded-[6px] p-2 transition-all',
                  'hover:bg-black/5 dark:hover:bg-white/5',
                ]"
              >
                <component :is="icon" class="w-[21px] shrink-0 opacity-60" />
                <span class="flex-1 whitespace-nowrap font-semibold">
                  {{ $t("theme") }}
                </span>
              </ThemeButton>
              <RouterLink
                v-for="(r, i) of auxRoutes"
                v-slot="{ isActive, isExactActive, navigate }"
                :key="`route-${i}`"
                :to="r.route"
                :exact-active-class="linkActiveClass"
                :active-class="linkActiveClass"
                custom
              >
                <div
                  :class="[
                    'relative cursor-pointer flex items-center space-x-3 border border-transparent',
                    ' overflow-hidden rounded-[6px] p-2 transition-all',
                    'hover:bg-black/5 dark:hover:bg-white/5',
                    {
                      [linkActiveClass]: r.useExact ? isExactActive : isActive,
                    },
                  ]"
                  @click="
                    () => {
                      navigate();
                      close(true);
                    }
                  "
                >
                  <component
                    :is="r.icon"
                    class="w-[21px] shrink-0 opacity-60"
                  />
                  <span class="flex-1 whitespace-nowrap font-semibold">
                    {{ r.name }}
                  </span>
                </div>
              </RouterLink>
            </div>
          </div>
        </div>
      </Transition>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { RouterLink, type RouteLocationRaw } from "vue-router";
import { useI18n } from "vue-i18n";
import {
  CircleHelpIcon as HelpIcon,
  BriefcaseBusinessIcon as ProjectsIcon,
  UserRoundIcon as UserIcon,
  SlidersVertical as SettingsIcon,
} from "lucide-vue-next";

import { cn, getRouteName } from "~/_common/utils";

import ThemeButton from "./ThemeButton.vue";

const props = withDefaults(defineProps<{ modelValue: boolean }>(), {
  modelValue: true,
});
const emit = defineEmits<{ "update:modelValue": [minimised: boolean] }>();

const linkActiveClass = computed<string>(() =>
  cn(
    "font-medium bg-black/5 hover:bg-black/5",
    "dark:bg-white/10 dark:hover:bg-white/10",
  ),
);

const isMinimised = computed(() => props.modelValue);
const i18n = useI18n();

interface IRoute {
  icon: Component;
  name: string;
  route: RouteLocationRaw;
  useExact?: boolean;
}
interface IGroupedRoute {
  title?: string;
  routes: IRoute[];
}

const groupedRoutes = computed<IGroupedRoute[]>(() => {
  const defaultRoutes: IGroupedRoute = {
    routes: [
      {
        icon: ProjectsIcon,
        name: i18n.t("projects", 2),
        route: { name: getRouteName("projects") },
      },
    ],
  };

  return [defaultRoutes];
});

const auxRoutes = computed<IRoute[]>(() => {
  return [
    {
      icon: SettingsIcon,
      name: i18n.t("settings"),
      route: { name: getRouteName("profile") },
    },
    {
      icon: HelpIcon,
      name: i18n.t("help"),
      route: { name: getRouteName("help") },
    },
  ];
});

function close(onlyOnMobile = false) {
  if (onlyOnMobile && window.innerWidth >= 768) return;
  emit("update:modelValue", true);
}
</script>

<style scoped></style>
