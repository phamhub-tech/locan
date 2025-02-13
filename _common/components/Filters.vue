<template>
  <NavModal class="relative text-[13px]">
    <template #trigger="{ openModal }">
      <Button variant="ghost" @click="openModal">
				<FilterIcon />
			</Button>
    </template>
    <template #default="{ closeModal }">
			<slot :closeModal="closeModal" />
    </template>
  </NavModal>
</template>

<script setup lang="ts">
import { FilterIcon } from "lucide-vue-next";
import NavModal from "~/_common/components/nav/NavModal.vue";
import { Button } from "~/_common/components/ui/button";
</script>
