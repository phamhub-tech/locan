<template>
  <div class="inline-block h-[32px]">
    <span
      class="block loader"
      style="animation-duration: 1.5s; animation-delay: 0.25s"
    />
  </div>
</template>

<style lang="scss" scoped>
.loader {
  @apply animate-bounce block relative rounded-full w-[16px] h-[16px] bg-primary;

  &:before,
  &:after {
    content: "";
    transform: rotate(0deg) translate(20px);
    @apply bg-secondary inset-0 absolute rounded-full;
    animation: rotate 1.5s ease infinite;
  }
  &:after {
    animation-delay: 0.85s;
  }
}

@keyframes rotate {
  100% {
    transform: rotate(360deg) translate(20px);
  }
}
</style>
