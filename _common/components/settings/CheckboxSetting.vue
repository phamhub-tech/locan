<template>
  <Setting :title="title">
		<slot />
	</Setting>
</template>

<script setup lang="ts">
import Setting from "./Setting.vue";

defineProps<{ title: string }>();
</script>
