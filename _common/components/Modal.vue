<template>
  <Dialog v-model:open="isOpen">
    <DialogTrigger as-child>
      <slot name="trigger" />
    </DialogTrigger>

    <DialogContent class="p-0 overflow-hidden rounded-lg bg-toolbar-background gap-0">
      <component v-bind="$attrs" :is="as">
        <DialogHeader class="px-4 py-6 bg-white shadow dark:bg-background">
          <DialogTitle>{{ title }}</DialogTitle>

          <div class="space-y-6">
            <slot />
          </div>
        </DialogHeader>

        <DialogFooter class="px-4 py-3 sm:justify-between">
          <DialogClose as-child>
            <Button type="button" variant="outline" class="bg-white dark:bg-background" @click="close">
              {{ $t("cancel") }}
            </Button>
          </DialogClose>
          <slot :close="close" name="footer" />
        </DialogFooter>
      </component>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import {
  Dialog,
  DialogClose,
  DialogFooter,
  DialogTrigger,
} from "~/_common/components/ui/dialog";

defineOptions({ inheritAttrs: false });
withDefaults(defineProps<{ title: string; as?: any }>(), { as: "div" });

const isOpen = defineModel<boolean>({ default: false });

function close() {
  isOpen.value = false;
}
</script>
