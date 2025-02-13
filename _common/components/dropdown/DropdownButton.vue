<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="outline" class="shrink-0">
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

defineProps<{icon: any, label?: string}>()
</script>
