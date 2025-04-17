<template>
	<Modal v-model="isOpen" :title="$t('newProject')" as="form" @submit="onSubmit">
		<template #trigger>
			<Button>
				<PlusIcon />
				{{ $t("createProject") }}
			</Button>
		</template>

		<FormField v-slot="{ componentField: { modelValue, ...c } }" name="rootDir">
			<FormItem>
				<FormControl>
					<div :tab-index="0" class="flex items-center">
						<Input :model-value="modelValue" :placeholder="$t('selectProjectRootTitle')" :class="[
							'no-focus',
							'!rounded-r-none focus-visible:ring-1 focus-visible:ring-offset-0',
						]" @update:model-value="c['onUpdate:modelValue']" />
						<Button class="rounded-l-none" type="button" @click="browse">{{ $t("browse") }}</Button>
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
					<div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
						<div v-for="framework in frameworks" :key="framework.id" :class="[
							'rounded-lg flex items-center gap-x-1 p-2',
							'transition-colors',
							componentField.modelValue === framework.id
								? 'bg-primary/30'
								: 'bg-slate-100 hover:bg-primary/10',
						]" role="button" @click="componentField.onChange(framework.id)">
							<img :src="framework.icon" :alt="framework.name" class="size-6" />
							<span class="text-xs truncate">{{ framework.name }}</span>
						</div>
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
import { PlusIcon } from "lucide-vue-next";

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
import { Debouncer } from "~/_common/utils";

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
		frameworkId: z.string().default("other"),
	}),
);
const form = useForm({
	validationSchema: formSchema,
	validateOnMount: false,
	initialValues: { frameworkId: 'other' }
});

const debouncer = new Debouncer(1000);
watch(() => form.values.rootDir, (newDir) => {
	if (newDir) {
		debouncer.run(async () => {
			const detectedFramework = await store.detectFramework(newDir);
			form.setFieldValue('frameworkId', detectedFramework);
		})
	} else {
		debouncer.cancel()
	}
});

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
