<template>
  <p>Home</p>
</template>

<script lang="ts" setup>
import { getRouteName } from "~/_common/utils";

const i18n = useI18n();
definePageMeta({
  name: "home",
  nameToTranslate: "home",
  noAuth: true,
  redirect: { name: getRouteName("projects") },
});
useSeoMeta({
  title: i18n.t("home"),
});
</script>
