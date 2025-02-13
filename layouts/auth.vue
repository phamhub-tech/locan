<template>
  <div
    :class="[
      'md:flex h-[100dvh] overflow-hidden',
      'bg-gradient-to-r from-[hsl(215,75%,94%)] to-[hsl(323,78%,95%)]',
    ]"
  >
    <div class="h-full flex-1 flex flex-col justify-between p-4">
      <div class="max-sm:hidden" />

      <slot />

      <div
        class="container flex items-center justify-between text-sm md:w-[400px]"
      >
        <p class="text-center">
          &copy; 2024.
          {{ $t("copyright") }}
        </p>

        <div class="flex items-center gap-x-3">
          <template v-for="(link, i) of links" :key="link.label">
            <NuxtLink :to="link.href" class="underline">
              {{ link.label }}
            </NuxtLink>
            <span v-if="i < links.length - 1">•</span>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const i18n = useI18n();
const links = [
  {
    label: i18n.t("termsOfService"),
    href: "#",
  },
  {
    label: i18n.t("privacyPolicy"),
    href: "#",
  },
];
</script>
