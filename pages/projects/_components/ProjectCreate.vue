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
            <Button class="rounded-l-none" type="button">{{ $t("browse") }}</Button>
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

    <FormField v-slot="{ componentField }" name="frameworkId">
      <FormItem>
        <FormLabel>{{ $t('framework') }}</FormLabel>
        <FormControl>
          <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
            <Button
              type="button"
              variant="outline"
              class="h-auto aspect-square flex-col gap-4 p-4 relative"
              :class="{'ring-2 ring-primary': componentField.modelValue === 'auto'}"
              @click="componentField.onChange('auto')"
            >
              <AutoIcon class="!size-12" />
              <span>{{ $t('autoDetect') }}</span>
            </Button>
            <Button
              v-for="framework in frameworks"
              :key="framework.id"
              :class="{'ring-2 ring-primary': componentField.modelValue === framework.id}"
              type="button"
              variant="outline"
              class="h-auto aspect-square flex-col gap-4 p-4"
              @click="componentField.onChange(framework.id)"
            >
              <img :src="framework.icon" :alt="framework.name" class="size-12" />
              <span>{{ framework.name }}</span>
            </Button>
          </div>
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
import { PlusIcon, WorkflowIcon as AutoIcon } from "lucide-vue-next";

import { Button } from "~/_common/components/ui/button";
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "~/_common/components/ui/form";
import { Input } from "~/_common/components/ui/input";
import Modal from "~/_common/components/Modal.vue";
import { useApiHandle } from "~/_common/core/api/composables";
import { selectDirectory } from "~/_common/core/dialog";

import { useProjectsStore } from "../_store/projects";
import { Frameworks } from "../_models/project-framework/types";

const isOpen = ref(false);
const frameworks = Frameworks;

const store = useProjectsStore();
const { addProjectApiStatus: apiStatus, addProjectApiMsg: apiMsg } =
  storeToRefs(store);
const apiHandle = useApiHandle(apiStatus);

const formSchema = toTypedSchema(
  z.object({
    name: z.string().min(1, "Project name is required"),
    rootDir: z.string().min(1, "Project directory is required"),
    frameworkId: z.string().optional().default("other"),
  }),
);
const form = useForm({ validationSchema: formSchema, validateOnMount: false, initialValues: { frameworkId: 'other'} });

const onSubmit = form.handleSubmit(async (values) => {
  await store.addProject({
    name: values.name,
    rootDir: values.rootDir,
    frameworkId: values.frameworkId,
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
