<template>
  <Modal
    v-model="isOpen"
    :title="$t('newProject')"
    as="form"
    @submit="onSubmit"
  >
    <template #trigger>
      <Button>
        <PlusIcon />
        {{ $t("createProject") }}
      </Button>
    </template>

    <FormField v-slot="{ componentField: { modelValue } }" name="rootDir">
      <FormItem>
        <FormControl>
          <div
            :tab-index="0"
            class="flex items-center"
            role="button"
            @click="browse"
          >
            <Input
              :model-value="modelValue"
              :placeholder="$t('selectProjectRootTitle')"
              class="pointer-events-none !rounded-r-none"
              disabled
            />
            <Button class="rounded-l-none">{{ $t("browse") }}</Button>
          </div>
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>
    <FormField v-slot="{ componentField }" name="name">
      <FormItem>
        <FormControl>
          <Input :placeholder="$t('projectName')" v-bind="componentField" />
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>

    <template #footer>
      <Button :loading="apiHandle.isLoading.value">
        {{ $t("createProject") }}
      </Button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";

import { Button } from "~/_common/components/ui/button";
import {
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "~/_common/components/ui/form";
import { Input } from "~/_common/components/ui/input";
import Modal from "~/_common/components/Modal.vue";
import { useApiHandle } from "~/_common/core/api/composables";

import { useProjectsStore } from "../_store/projects";
import { selectDirectory } from "~/_common/core/dialog";
import { PlusIcon } from "lucide-vue-next";

const isOpen = ref(false);

const store = useProjectsStore();
const { addProjectApiStatus: apiStatus, addProjectApiMsg: apiMsg } =
  storeToRefs(store);
const apiHandle = useApiHandle(apiStatus);

const formSchema = toTypedSchema(
  z.object({
    name: z.string(),
    rootDir: z.string(),
  }),
);
const form = useForm({ validationSchema: formSchema, validateOnMount: false });

const onSubmit = form.handleSubmit(async (values) => {
  await store.addProject({
    name: values.name,
    rootDir: values.rootDir,
  });

  isOpen.value = false;
});

const i18n = useI18n();
async function browse() {
  const dir = await selectDirectory(i18n.t("selectProjectRootTitle"));
  if (dir === null) return;

  form.setValues({ rootDir: dir }, true);
}
</script>
