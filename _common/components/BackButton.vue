<template>
  <div class="flex items-center gap-x-2">
    <Button size="icon" variant="secondary" @click="goBack">
      <ArrowLeftIcon />
    </Button>
    <slot />
  </div>
</template>

<script setup lang="ts">
import { ArrowLeftIcon } from "lucide-vue-next";

import { Button } from "./ui/button";

const router = useRouter()
function goBack() {
	router.back()
}
</script>
