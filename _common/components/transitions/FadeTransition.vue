<template>
  <Transition :mode="mode" @enter="onEnter" @leave="onLeave">
    <slot />
  </Transition>
</template>

<script setup lang="ts">
import { animate } from 'motion'

defineProps<{ mode?: "out-in" | "in-out" }>();

async function onEnter(e: Element, onComplete: () => void) {
	await animate(e, { opacity: [0, 1] })
	onComplete()
}
async function onLeave(e: Element, onComplete: () => void) {
	await animate(e, { opacity: 0 })
	onComplete()
}
</script>
