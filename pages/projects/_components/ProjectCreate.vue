<template>
  <Modal
    v-model="isOpen"
    :title="$t('newProject')"
    as="form"
    @submit="onSubmit"
  >
    <template #trigger>
      <Button> {{ $t("createProject") }} </Button>
    </template>

    <FormField v-slot="{ componentField }" name="name">
      <FormItem>
        <FormLabel> {{ $t("projectName") }} </FormLabel>
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
  FormLabel,
  FormMessage,
} from "~/_common/components/ui/form";
import { Input } from "~/_common/components/ui/input";
import Modal from "~/_common/components/Modal.vue";
import { useApiHandle } from "~/_common/core/api/composables";

import { useProjectsStore } from "../_store/projects";

const isOpen = ref(false);

const store = useProjectsStore();
const { addProjectApiStatus: apiStatus, addProjectApiMsg: apiMsg } =
  storeToRefs(store);
const apiHandle = useApiHandle(apiStatus);

const formSchema = toTypedSchema(
  z.object({
    name: z.string(),
  }),
);
const form = useForm({ validationSchema: formSchema, validateOnMount: false });

const onSubmit = form.handleSubmit(async (values) => {
  await store.addProject({
    name: values.name,
  });

  isOpen.value = false;
});
</script>
