<template>
  <div>
    <template v-if="urls?.length">
      <div
        :class="[
          'group w-full border border-dashed rounded-lg',
          'p-4 transition-all grid-centered',
          'hover:border-primary/50',
          'outline-none focus:ring-primary/20 focus:border-primary/50 focus:ring-4',
        ]"
      >
        <div v-for="({ url }, i) of urls" :key="`url-${i}`">
          <img :src="url" :alt="`Upload-${i}`" class="rounded" />
        </div>
      </div>
    </template>
    <FileInput v-else v-model="modelValue" accept="image/*" />
  </div>
</template>

<script setup lang="ts">
import FileInput from "./FileInput.vue";

interface IImageUrl {
  file: File;
  url: string;
}

interface IProps {
  placeholder?: string;
  multiple?: boolean;

  // eslint-disable-next-line no-unused-vars
  onChange?: (file: FileList | null) => void;
}

const modelValue = defineModel<File | FileList | null>();
defineProps<IProps>();

const urls = computed<IImageUrl[]>(() => {
  let localUrls: IImageUrl[] = [];

  const value = modelValue.value;
  if (value instanceof File)
    localUrls = [
      {
        file: value,
        url: URL.createObjectURL(value),
      },
    ];
  else if (value instanceof FileList) {
    localUrls = Array.from(value).map((file) => ({
      file,
      url: URL.createObjectURL(file),
    }));
  }

  return localUrls;
});
</script>
