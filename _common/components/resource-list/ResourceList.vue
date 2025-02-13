<template>
  <ResourceLoader
    :apiHandle="apiHandle"
    :apiMsg="apiMsg"
    :isFullReload="!isRefreshing && items === null"
    @retry="getData"
  >
    <DataTable
      v-if="items"
      :items="items"
      :headers="headers"
      :search-placeholder="placeholder"
      :current-page="page"
      :page-size="pageSize"
      :total-pages="pages"
      :total-items="total"
      :is-refreshing="isRefreshing"
      :is-pagination-loading="apiHandle.isLoading.value"
      align="center"
      @refresh="refresh"
      @search="(q) => (query = q)"
      @sort="sortData"
      @goto-page="(p) => (page = p)"
      @update-page-size="(size) => (pageSize = size)"
			@item-click="onItemClick"
    >
      <template #actions>
				<slot name="actions" />
      </template>
      <template #list-item-value="{ item, key }">
				<slot name="list-item-value" :item="item" :key="key" />
      </template>
    </DataTable>
  </ResourceLoader>
</template>

<script setup lang="ts" generic="T, S extends TSortParams<any>">
import { handleSortParams, type IHeader, type ISortField } from "~/_common/components/table";
import type { TSortParams } from "~/_common/core/api";
import { ApiHandle } from "~/_common/core/api/composables";

import ResourceLoader from "../loaders/ResourceLoader.vue";
import DataTable from "../table/DataTable.vue";
import type { IResourceData } from "./types";

const props = defineProps<{
  apiMsg: string;
  pageSize: number;
  pages: number;
  total: number;
  page: number;
  items: T[] | null;
  apiHandle: ApiHandle;
  sortParams: S;
	placeholder?: string;
	headers: IHeader<T>[]
	onItemClick?: (item: T) => void
}>();
const emit = defineEmits<{
  setPageSize: [size: number];
  setSortParams: [params: S];
	getData: [data: IResourceData];
}>();

const query = ref("");
const page = ref(props.page);
const pageSize = computed({
  get() {
    return props.pageSize;
  },
  set(size) {
    emit("setPageSize", size);
  },
});
watch(page, getData);
watch(pageSize, () => resetPageAndGet(page, getData));
watch(query, () => resetPageAndGet(page, getData));

function sortData(field: ISortField<S>) {
  const params = handleSortParams(props.sortParams, field);
	emit('setSortParams', params)
  getData();
}

let isRefreshing = ref(false);
async function refresh() {
  isRefreshing.value = true;
  await getData();
  isRefreshing.value = false;
}

getData();
async function getData() {
  const q = query.value.trim();

  emit("getData", {
    pageSize: pageSize.value,
    page: page.value,
    query: q,
  });
}

function resetPageAndGet(page: Ref<number>, getData: () => void) {
  if (page.value > 1) {
    page.value = 1;

    // Return since setting the page value will automatically trigger the getData function
    return;
  }

  // We're good, just get the data without resetting the page
  getData();
}
</script>
