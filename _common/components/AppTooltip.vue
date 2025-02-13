<template>
  <TooltipProvider>
    <Tooltip>
      <TooltipTrigger as-child>
				<CircleHelpIcon class="opacity-40" :size="20" />
      </TooltipTrigger>

      <TooltipContent>
        <p>{{ tooltip }}</p>
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
</template>

<script setup lang="ts">
import { CircleHelpIcon } from "lucide-vue-next";

import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "./ui/tooltip";

defineProps<{ tooltip: string }>();
</script>
