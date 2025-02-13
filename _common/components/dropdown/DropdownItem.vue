<template>
	<DropdownMenuItem :value="value">
		<slot />
	</DropdownMenuItem>
</template>

<script setup lang="ts">
import { DropdownMenuItem } from '../ui/dropdown-menu';

defineProps<{ value: string }>()
</script>
