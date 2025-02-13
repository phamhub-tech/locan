<template>
  <div
    ref="dragAreaRef"
    role="button"
    :class="
      cn(
        'group w-full border border-dashed rounded-md',
        'p-4 transition-all grid-centered',
        'hover:border-primary/50',
        'outline-none focus:ring-primary/20 focus:border-primary/50 focus:ring-4',
        $attrs.class as string | undefined,
      )
    "
    tabindex="0"
    @keyup.enter="clickInput"
    @keyup.space="clickInput"
    @click="clickInput"
  >
    <component
      :is="isImageRestricted ? ImageIcon : FileIcon"
      :size="60"
      class="stroke-[1] opacity-60 transition-opacity group-hover:opacity-100"
    />
    <p class="text-muted-foreground" v-if="!files">
      {{
        placeholder ??
        $t(isImageRestricted ? "imageUploadText" : "fileUploadText")
      }}
    </p>
    <div v-else>
      <span v-for="file of files" :key="`file-name-${file}`">
        {{ file }}
      </span>
    </div>
  </div>
  <input ref="inputRef" type="file" :accept="accept" hidden />
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import {
  ImagePlusIcon as ImageIcon,
  FolderPlusIcon as FileIcon,
} from "lucide-vue-next";

import { cn } from "~/_common/utils";

defineOptions({ inheritAttrs: false });
const model = defineModel<File | FileList | null>();

const props = defineProps<{ placeholder?: string; accept?: string }>();
const isImageRestricted = computed<boolean>(
  () => props.accept?.startsWith("image") ?? false,
);

const inputRef = ref<HTMLInputElement | null>(null);
const dragAreaRef = ref<HTMLDivElement | null>(null);
onMounted(() => {
  const input = inputRef.value;
  input?.addEventListener("change", handleInputChange);
});
onUnmounted(() => {
  const input = inputRef.value;
  input?.removeEventListener("change", handleInputChange);
});

const files = computed<string[] | null>(() => {
  const fs = model.value;
  if (!fs) return null;

  if (fs instanceof File) {
    return [fs.name];
  }

  return Array.from(fs).map((f) => f.name);
});

async function handleInputChange() {
  const files = inputRef.value!.files;
  model.value = files;
}

function clickInput(evt: Event) {
  evt.stopPropagation();
  evt.preventDefault();

  const el = inputRef.value;
  el?.click();
}
</script>
