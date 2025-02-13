<template>
  <div
    :id="`status-${variant}`"
    :class="
      cn(
        'mb-2 rounded border p-3',
        variant === 'error' &&
          'border-red-300 bg-red-300/10 text-red-800 dark:border-red-400 dark:text-red-400',
        variant === 'success' &&
          'border-green-300 bg-green-300/10 text-green-800 dark:border-green-800 dark:text-green-400',
        $attrs.class as string | undefined,
      )
    "
  >
    <p v-if="showTitle" class="mb-2 text-2xl">
      {{ title ?? $t(variant) }}
    </p>
    <p>
      <slot />
    </p>
    <slot name="action">
      <Button
        v-if="onRetry && variant === 'error'"
				variant="primary"
        class="mt-2"
        @click.prevent.stop="$emit('retry')"
      >
        {{ $t('retry') }}
      </Button>
    </slot>
  </div>
</template>

<script setup lang="ts">
import { cn } from '../utils';

import { Button } from './ui/button';

defineOptions({
  inheritAttrs: false,
})

withDefaults(
  defineProps<{
    variant: 'error' | 'success'
    title?: string
    showTitle?: boolean
    onRetry?: () => void
  }>(),
  { showTitle: true, shouldRetry: true },
)
defineEmits<{ retry: [] }>()
</script>
