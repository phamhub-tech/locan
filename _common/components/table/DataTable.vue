<template>
  <div
    v-bind="attrsToBind"
    :class="
      twMerge(
        '@container border-y bg-white dark:bg-white/[0.025] border-slate-200 p-3 dark:border-slate-300/10',
        $attrs.class as string | undefined,
      )
    "
  >
    <header
      v-if="hasHeader"
      class="sm:flex mb-6 items-center justify-between gap-x-2"
    >
      <p v-if="title">{{ title }}</p>
      <Input
        v-model="query"
        type="search"
        :class="cn('flex-1 sm:max-w-[300px] md:max-w-[350px]', inputClass)"
        :placeholder="searchPlaceholder"
      />

      <div
        class="dt-actions max-sm:mt-2 flex items-center max-sm:justify-center gap-x-1"
      >
        <slot name="actions" />
        <button v-if="hasGridLayout" @click="toggleListMode">
          <LayoutGridIcon v-if="isListMode" />
          <ListIcon v-else />
        </button>
        <button v-if="onRefresh" class="group" @click="onRefresh">
          <RefreshIcon
            :class="[
              'transition-transform duration-300 ease-out',
              isRefreshing ? 'animate-spin' : 'group-hover:rotate-180',
            ]"
          />
        </button>
      </div>
    </header>
    <main :class="twMerge('relative overflow-x-auto', bodyClass)">
      <Transition name="fade">
        <div
          v-if="isRefreshing"
          :class="[
            'absolute bg-white/30 cursor-not-allowed inset-0',
            'dark:bg-transparent',
          ]"
        />
      </Transition>
      <table v-if="isListMode || !hasGridLayout" class="w-full text-left">
        <thead :key="`key-${headerRerenderTriggerKey}`">
          <tr>
            <th v-if="isItemSelected"></th>
            <th
              v-for="(header, i) in headers"
              :key="`header-${i}`"
              @click="sortHeader(header)"
            >
              <div
                :class="
                  twMerge(
                    'flex items-center tracking-wider font-semibold smallcaps gap-x-1',
                    header.sort !== undefined && 'cursor-pointer select-none',
                    (contentAlignment(header.align) === 'center' ||
                      (header.isDate &&
                        contentAlignment(header.align) === undefined)) &&
                      'justify-center text-center',
                    contentAlignment(header.align) === 'right' &&
                      'justify-end text-right',
                    header.classes,
                  )
                "
              >
                <slot name="header" v-bind="{ header }">
                  {{ header.label }}
                </slot>
                <MoveUpIcon
                  v-if="header.sort === 'asc'"
                  class="inline-block h-5 w-5 shrink-0 stroke-2"
                />
                <MoveDownIcon
                  v-else-if="header.sort === 'desc'"
                  class="inline-block h-5 w-5 shrink-0 stroke-2"
                />
              </div>
            </th>
            <th v-if="hasItemActions">Actions</th>
          </tr>
        </thead>
        <tbody ref="itemsList">
          <tr v-if="filteredItems.length === 0">
            <td :colspan="headers.length" class="text-center !pt-8">
              No items found
            </td>
          </tr>
          <template v-else>
            <tr
              v-for="(item, i) of filteredItems"
              :key="`item-${i}`"
              :class="[
                'border-y border-slate-200 dark:border-slate-200/10',
                { 'last:border-y-transparent': !hasFooter },
                isItemSelected && isItemSelected(item)
                  ? 'bg-primary/[0.15] text-primary-dark'
                  : isItemClickable(item) &&
                    'cursor-pointer hover:bg-primary/5',
              ]"
              @click="isItemClickable(item) && onItemClick!(item)"
            >
              <td v-if="isItemSelected" class="text-center">
                <input
                  type="checkbox"
                  class="cursor-pointer"
                  :checked="isItemSelected(item)"
                />
              </td>
              <td
                v-for="(header, j) of headers"
                :key="`item-value-${j}`"
                :class="[
                  {
                    'text-center':
                      contentAlignment(header.align) == 'center' ||
                      (header.isDate &&
                        contentAlignment(header.align) === undefined),
                    'text-right': contentAlignment(header.align) == 'right',
                  },
                  typeof header.itemClasses === 'string' ||
                  header.itemClasses === undefined
                    ? header.itemClasses
                    : header.itemClasses(item),
                  itemClasses,
                ]"
              >
                <slot
                  name="list-item-value"
                  v-bind="{
                    key: header.key,
                    value: header.ignoreValue
                      ? null
                      : getItemValue(item, header.value, header.isDate),
                    item,
                    isSelected: isItemSelected && isItemSelected(item),
                  }"
                >
                  {{ getItemValue(item, header.value, header.isDate) }}
                </slot>
              </td>
              <td v-if="hasItemActions" class="text-center">
                <slot name="item-actions" v-bind="{ item }" />
              </td>
            </tr>
          </template>
        </tbody>
      </table>
      <div
        v-else
        class="@lg:grid-cols-3 @2xl:grid-cols-4 @4xl:grid-cols-5 grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5"
      >
        <!-- <div
        v-else
        class="grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5"
      > -->
        <div
          v-for="(item, i) of filteredItems"
          :key="`grid-item-${i}`"
          class="duration-200"
          @click="isItemClickable(item) && onItemClick!(item)"
        >
          <slot name="grid-item" v-bind="item">
            <div
              :class="[
                'rounded-lg p-3 text-center transition-colors duration-inherit',
                { 'cursor-pointer hover:bg-primary/5': isItemClickable(item) },
              ]"
            >
              <p class="text-semibold">
                {{
                  getItemValue(item, headers[0].value, headers[0].isDate)
                }}
              </p>
              <p class="text-sm opacity-60">
                {{
                  getItemValue(item, headers[1].value, headers[1].isDate)
                }}
              </p>
            </div>
          </slot>
        </div>
      </div>
    </main>
    <footer v-if="hasFooter" class="mt-8 text-sm">
      <div class="items flex items-center justify-between">
        <p v-if="startCount && endCount">
          Showing {{ startCount }} - {{ endCount }} of
          {{ totalItems ?? items.length }}
        </p>
        <p v-else>
          Showing {{ filteredItems.length }} of {{ totalItems ?? items.length }}
        </p>
        <Select v-model="internalPageSize">
          <SelectTrigger class="w-20">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem
              v-for="opt of pageSizeOptions"
              :key="`dt-opt-${opt}`"
              :value="opt.toString()"
            >
              {{ opt }}
            </SelectItem>
          </SelectContent>
        </Select>
      </div>
      <div class="mx-auto mt-3 w-full">
        <Pagination
          v-if="totalPages && totalPages > 1"
          :page="currentPage"
          :pages="totalPages"
          :is-loading="isPaginationLoading"
          @goto-page="(page) => $emit('goto-page', page)"
        />
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts" generic="T">
import {
  ref,
  computed,
  watch,
  useSlots,
  onMounted,
  onUnmounted,
  useAttrs,
} from "vue";
import {
  ListIcon,
  LayoutGridIcon,
  XIcon,
  MoveUpIcon,
  MoveDownIcon,
  RefreshCcw as RefreshIcon,
} from "lucide-vue-next";
import { twMerge } from "tailwind-merge";

import { pageSizeOptions } from "~/_common/constants";
import { cn, humanizeDate, removeFromAttrs } from "~/_common/utils";

import { Input } from "../ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";

import Pagination from "./Pagination.vue";
import type {
  IHeader,
  ISortField,
  TDataTableAlignment,
} from "./interface";
import type { TSortType } from "~/_common/core/api";

defineOptions({ inheritAttrs: false });
const props = withDefaults(
  defineProps<{
    headers: IHeader<T>[];
    items: T[];
    title?: string;
    searchPlaceholder?: string;
    totalItems?: number;
    currentPage?: number;
    totalPages?: number;
    itemClasses?: string;
    pageSize?: number | null;
    isPaginationLoading?: boolean;
    searchDelay?: number;
    sortDelay?: number;
    hasGridLayout?: boolean;
    hasHeader?: boolean;
    hasFooter?: boolean;
    nullValue?: string;
    align?: TDataTableAlignment;
    isRefreshing?: boolean;
    bodyClass?: string;
    inputClass?: string;
    isItemSelected?: (item: T) => boolean;
    canClickItem?: (item: T) => boolean;
    onItemClick?: (item: T) => void;
    onRefresh?: () => void;
  }>(),
  {
    searchPlaceholder: "Search item",
    hasHeader: true,
    hasFooter: true,
    sortDelay: 800,
    searchDelay: 800,
    nullValue: "-",
  },
);

//! Do not use the new emits syntax until https://github.com/vuejs/language-tools/issues/3431 is fixed
const emit = defineEmits<{
  (e: "update-page-size", size: number): void;
  (e: "goto-page", page: number): void;
  (e: "search", query: string): void;
  (e: "sort", field: ISortField): void;
}>();

const attrs = useAttrs();
const attrsToBind = computed<Record<string, any>>(() => {
  return removeFromAttrs(attrs, "class");
});

const internalPageSize = computed<string>({
	get: () => `${props.pageSize ?? 10}`,
  set: (v) => emit("update-page-size", +v),
});

function contentAlignment(
  headerAlignment?: TDataTableAlignment,
): TDataTableAlignment | undefined {
  return headerAlignment ?? props.align;
}

function isItemClickable(item: T): boolean {
  if (props.canClickItem) return props.canClickItem(item);
  return !!props.onItemClick;
}

const startCount = computed<number | null>(() => {
  const page = props.currentPage;
  const pageSize = props.pageSize;
  if (!page || !pageSize) return null;

  return (page - 1) * pageSize + 1;
});
const endCount = computed<number | null>(() => {
  const start = startCount.value;
  const pageSize = props.pageSize;

  if (start === null || !pageSize) return null;

  return Math.min(start + pageSize - 1, props.totalItems ?? props.items.length);
});

const slots = useSlots();
const hasItemActions = ref(false);
let observer: MutationObserver | null = null;
const itemsList = ref<HTMLElement | null>(null);
onMounted(() => {
  observer?.disconnect();
  hasItemActions.value = slots["item-actions"] !== undefined;

  observer = new MutationObserver(() => {
    hasItemActions.value = slots["item-actions"] !== undefined;
  });
  observer.observe(itemsList.value!, {
    childList: true,
    subtree: true,
  });
});
onUnmounted(() => observer?.disconnect);

const query = ref("");
let searchTimeout: number | null = null;
watch(query, (value) => {
  if (searchTimeout) clearTimeout(searchTimeout);

  searchTimeout = window.setTimeout(
    () => emit("search", value.trim()),
    props.searchDelay ?? 0,
  );
});

const filteredItems = computed(() => {
  return props.items;
});

const isListMode = ref(true);
function toggleListMode(): void {
  isListMode.value = !isListMode.value;
}

const headerRerenderTriggerKey = ref(true);
let sortTimeout: number | null = null;
function sortHeader(header: IHeader<T>) {
  if (sortTimeout) clearTimeout(sortTimeout);
  const mode = header.sort;
  if (mode === undefined) return;

  let sortMode: TSortType = "desc";
  if (mode === "desc") sortMode = "asc";
  else if (mode === "asc") sortMode = null;

  header.sort = sortMode;
  headerRerenderTriggerKey.value = !headerRerenderTriggerKey.value;
  sortTimeout = window.setTimeout(
    () =>
      emit("sort", {
        name: (header.key ?? header.value) as string,
        value: sortMode,
      }),
    props.sortDelay ?? 0,
  );
}

function getItemValue(
  item: any,
  key: IHeader<T>["value"],
  formatAsDate?: boolean,
): any {
  if (typeof key === "function") {
    const value = key(item as T);
    return value ?? props.nullValue;
  }

  const split = key.toString().split(".");
  if (split.length === 1) {
    let returnValue: string | number | Date | null = item[key];
    if (returnValue === null || returnValue.toString().trim() === "")
      return props.nullValue;

    if (formatAsDate)
      return humanizeDate(returnValue as Date, false)
        .split(" ")
        .map((value) => value.replace(/^(\w)/, (c) => c.toUpperCase()))
        .join(" ");

    return returnValue ?? props.nullValue;
  }

  return getItemValue(
    item[split[0] as string],
    split.slice(1).join("."),
    formatAsDate,
  );
}
</script>

<style lang="scss">
th {
  @apply whitespace-nowrap;
}

.dt-actions button:not(.not-action) {
  @apply cursor-pointer rounded p-2 hover:bg-slate-200 dark:hover:bg-slate-50/10;

  svg {
    @apply h-6 w-6;
  }
}
</style>
