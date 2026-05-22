<script setup lang="ts">
withDefaults(
  defineProps<{
    variant?: 'normal' | 'alert';
    tail?: 'center' | 'left' | 'right';
  }>(),
  { variant: 'normal', tail: 'center' },
);
</script>

<template>
  <div
    class="speech-bubble relative flex items-center gap-1 px-2 py-1 rounded-full text-xs font-bold"
    :class="[
      variant === 'alert' ? 'bg-danger-color text-white' : 'bg-accent-color text-tm-black',
      `tail-${tail}`,
    ]"
  >
    <slot />
  </div>
</template>

<style scoped>
.speech-bubble::after {
  content: '';
  position: absolute;
  bottom: -4px;
  width: 8px;
  height: 8px;
  background-color: inherit;
}

.speech-bubble.tail-center::after {
  left: 50%;
  transform: translateX(-50%) rotate(45deg);
}

.speech-bubble.tail-left::after {
  left: 20%;
  transform: translateX(-50%) rotate(45deg);
}

.speech-bubble.tail-right::after {
  left: 80%;
  transform: translateX(-50%) rotate(45deg);
}
</style>
