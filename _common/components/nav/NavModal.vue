<template>
  <slot name="trigger" :open-modal="openModal" />

  <TransitionRoot appear :show="isOpen" as="template">
    <Dialog
      as="div"
      class="relative overflow-hidden z-30"
      :initial-focus="focusProxy"
      @close="closeModal"
    >
      <TransitionChild
        as="template"
        enter="duration-300 ease-out"
        enter-from="opacity-0"
        enter-to="opacity-100"
        leave="duration-200 ease-in"
        leave-from="opacity-100"
        leave-to="opacity-0"
      >
        <div
          class="fixed inset-0 bg-slate-500/20 translate-y-14 h-[calc(100vh-3.5rem)] backdrop-blur-[2px] dark:bg-slate-900/20"
        />
      </TransitionChild>

      <div class="fixed inset-0 overflow-hidden mt-14 h-[calc(100vh-3.5rem)]">
        <div class="flex min-h-full justify-end p-1 text-center">
          <TransitionChild
            as="template"
            enter="duration-300 ease-out"
            enter-from="opacity-0 translate-x-10"
            enter-to="opacity-100 translate-x-0"
            leave="duration-200 ease-in"
            leave-from="opacity-100 translate-x-0"
            leave-to="opacity-0 translate-x-10"
          >
            <DialogPanel
              :class="
                twMerge(
                  'h-[calc(100vh-3.5rem-0.75rem)] overflow-hidden w-96 rounded-lg bg-white text-left align-middle shadow-xl transition-all',
                  'divide-y divide-gray-200 dark:divide-gray-700 flex flex-col',
                  'dark:bg-scaffold-tint dark:bg-background-dark dark:border dark:border-slate-100/10',
                  'dark:bg-slate-900',
                  $attrs.class as string | undefined,
                )
              "
            >
              <DialogTitle
                v-if="showTitle"
                as="h3"
                class="mb-3 text-center text-lg font-medium leading-6 text-gray-900"
              >
                <slot name="title" />
              </DialogTitle>
              <slot :close-modal="closeModal" />
              <div
                v-if="hasFooter"
                class="mt-auto bg-slate-100 dark:bg-slate-800 px-4 py-3"
              >
                <slot name="footer" :close-modal="closeModal" />
              </div>
            </DialogPanel>
          </TransitionChild>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>

  <!-- This is needed to prevent headlessui from autofocusing the first focusable element it sees-->
  <div ref="focusProxy" class="hidden" />
</template>

<script setup lang="ts">
import { computed, ref, useSlots } from "vue";
import { twMerge } from "tailwind-merge";
import {
  TransitionRoot,
  TransitionChild,
  Dialog,
  DialogPanel,
  DialogTitle,
} from "@headlessui/vue";

defineOptions({ inheritAttrs: false });
const props = withDefaults(
  defineProps<{
    modelValue?: boolean;
    useInternalState?: boolean;
    showTitle?: boolean;
  }>(),
  { useInternalState: true },
);
const emit = defineEmits<{
  "update:modelValue": [value: boolean];
  close: [];
}>();

const focusProxy = ref<HTMLDivElement | null>();

const isOpenInternal = ref(false);
const isOpen = computed<boolean>(() =>
  props.useInternalState ? isOpenInternal.value : props.modelValue,
);

const slots = useSlots();
const hasFooter = computed<boolean>(() => {
  return Object.keys(slots).includes("footer");
});

function closeModal() {
  isOpenInternal.value = false;
  emit("update:modelValue", false);
  emit("close");
}
function openModal() {
  isOpenInternal.value = true;
  emit("update:modelValue", true);
}
</script>
