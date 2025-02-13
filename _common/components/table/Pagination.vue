<template>
  <div class="mx-auto flex w-fit flex-wrap gap-2">
    <div v-if="shrinkPages" class="text-center space-y-1">
      <p class="opacity-60">
        {{
          currentPage === 1
            ? $t('pageOfPages', { pages })
            : $t('pagesOfPages', { page: currentPage, pages })
        }}
      </p>
      <div
        :class="[
          'flex rounded-lg border border-slate-300 items-stretch',
          'gap-x-1 w-max mx-auto',
          'dark:border-slate-700',
        ]"
      >
        <Button
          :class="[
            'rounded-l-lg rounded-r-none text-inherit p-3 bg-primary bg-opacity-20 transition-colors hover:bg-opacity-40',
            'disabled:bg-slate-100',
            'dark:bg-primary/20 dark:hover:bg-primary/40 dark:disabled:bg-primary/20',
          ]"
          :loading="isLoading"
          :disabled="currentPage === 1"
          @click="gotoPrevPage"
        >
          <ChevronLeftIcon class="w-5 h-5" />
        </Button>
        <div class="flex gap-x-1 items-center py-1">
          <div class="flex gap-x-1">
            <Button
              v-for="pageCount of 3"
              :key="`page-i-${pageCount}`"
              :class="[
                'select-none h-10 w-9 border transition-all duration-300',
                pageCount === currentPage
                  ? 'border-primary bg-primary text-white dark:text-black'
                  : '!bg-transparent text-[inherit] border-slate-300 hover:border-primary/30 hover:bg-primary/20 dark:border-slate-300/20',
              ]"
              :loading="isLoading && pageCount === currentPage"
              @click="gotoPage(pageCount)"
            >
              {{ pageCount }}
            </Button>
          </div>
          <div class="w-16 text-center">
            <input
              v-if="showInput"
              ref="input"
              v-model="inputPage"
              type="number"
              class="border-none focus:border-primary rounded bg-transparent w-16 text-center"
              :min="0"
              :max="maxPage"
              :disabled="isLoading"
              @blur="
                () => {
                  showInput = false
                  gotoPage(inputPage)
                }
              "
              @keyup.enter="() => gotoPage(inputPage)"
            />
            <p v-else class="" role="button" @click="showInput = true">...</p>
          </div>
          <div class="flex gap-x-1">
            <Button
              v-for="pageAfter of pagesToShowAfter"
              :key="`page-after-${pageAfter}`"
              :class="[
                'select-none h-10 w-max border transition-all duration-300',
                pageAfter === currentPage
                  ? 'border-primary bg-primary text-white dark:text-black'
                  : '!bg-transparent text-[inherit] border-slate-300 hover:border-primary/30 hover:bg-primary/20 dark:border-slate-300/20',
              ]"
              :loading="isLoading && pageAfter === currentPage"
              @click="gotoPage(pageAfter)"
            >
              {{ pageAfter }}
            </Button>
          </div>
        </div>
        <Button
          :class="[
            'rounded-r-lg rounded-l-none text-inherit p-3 bg-primary bg-opacity-20 transition-colors hover:bg-opacity-40',
            'disabled:bg-slate-100',
            'dark:bg-primary/20 dark:hover:bg-primary/40 dark:disabled:bg-primary/20',
          ]"
          :loading="isLoading"
          :disabled="currentPage === maxPage"
          @click="gotoNextPage"
        >
          <ChevronRightIcon class="w-5 h-5" />
        </Button>
      </div>
      <p v-if="inputPage > maxPage" class="opacity-60 text-red-500">
        {{ $t('maxIs', { num: maxPage }) }}
      </p>
      <p v-if="inputPage < 1" class="opacity-60 text-red-500">{{ $t('minIs', { num: 1 }) }}</p>
    </div>
    <template v-else>
      <Button
        v-for="pageCount of pages"
        :key="`page-i-${pageCount}`"
        :class="[
          'select-none h-10 w-9 border transition-all duration-300',
          pageCount === currentPage
            ? 'border-primary bg-primary text-white dark:text-black'
            : '!bg-transparent text-[inherit] border-slate-300 hover:border-primary/30 hover:bg-primary/20 dark:border-slate-300/20',
        ]"
        :loading="isLoading && pageCount === currentPage"
        @click="gotoPage(pageCount)"
      >
        {{ pageCount }}
      </Button>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue'
import { ChevronLeftIcon, ChevronRightIcon } from 'lucide-vue-next';

import { Button } from '../ui/button';

const props = withDefaults(
  defineProps<{
    pages: number
    page?: number
    isLoading?: boolean
  }>(),
  {
    isLoading: false,
    page: 1,
  },
)
const emits = defineEmits<{
  gotoPage: [page: number]
}>()

const modelValue = defineModel<number | number>({ default: undefined })

const input = ref<HTMLInputElement | null>(null)
const showInput = ref(false)
watch(showInput, async () => {
  nextTick(() => {
    if (input.value === null) return
    input.value.focus()
  })
})

const maxPagesBeforeOverflow = 8
const shrinkPages = computed<boolean>(() => props.pages > maxPagesBeforeOverflow)
const currentPage = computed<number>(() => modelValue.value ?? props.page!)
const maxPage = computed<number>(() => props.pages)
const inputPage = ref(currentPage.value)
watch(currentPage, (p) => (inputPage.value = p))

const pagesToShowAfter = computed<number[]>(() => {
  const count = Math.min(2, props.pages - maxPagesBeforeOverflow)
  const remainder = props.pages - count

  const pages: number[] = []
  for (let i = 0; i < count; ++i) {
    pages.push(remainder + i + 1)
  }

  return pages
})

function gotoPage(p: number) {
  if (p === props.page || p > props.pages || p < 1) return

  if (modelValue.value) modelValue.value = p
  else emits('gotoPage', p)
}
let timer: number | null = null
async function gotoPrevPage() {
  if (timer) window.clearTimeout(timer)

  const prevPage = inputPage.value - 1
  inputPage.value = prevPage
  timer = window.setTimeout(() => {
    gotoPage(prevPage)
  }, 300)
}
function gotoNextPage() {
  if (timer) window.clearTimeout(timer)

  const nextPage = inputPage.value + 1
  inputPage.value = nextPage
  timer = window.setTimeout(() => {
    gotoPage(nextPage)
  }, 300)
}
</script>

<style scoped></style>
