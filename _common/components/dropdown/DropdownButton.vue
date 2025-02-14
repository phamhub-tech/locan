<template>
  <DropdownMenu v-bind="$attrs">
    <DropdownMenuTrigger as-child>
      <Button variant="outline" :class="cn('shrink-0', $attrs.class as string | undefined)">
        <slot name="trigger">
          <component :is="icon" />
          {{ label }}
        </slot>

        <ChevronDownIcon class="ml-4 text-muted-foreground" />
      </Button>
    </DropdownMenuTrigger>

		<DropdownMenuContent>
			<slot />
		</DropdownMenuContent>
  </DropdownMenu>
</template>

<script setup lang="ts">
import { ChevronDownIcon } from 'lucide-vue-next';

import { DropdownMenu, DropdownMenuTrigger } from '../ui/dropdown-menu';
import DropdownMenuContent from '../ui/dropdown-menu/DropdownMenuContent.vue';
import { cn } from '~/_common/utils';

defineOptions({ inheritAttrs: false })
defineProps<{icon: any, label?: string}>()
</script>
