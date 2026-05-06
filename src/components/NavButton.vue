<!-- ナビゲーション共通ボタン -->
<!-- ホバー時: bg-pond-color / active 時: bg-base-color -->
<!-- locked 時: sub-color 表示でクリック可能、押下するとシェイクアニメーション -->
<!-- TabGroup・DayNav 内のボタンから使用される -->
<script setup lang="ts">
import { ref } from 'vue';

defineProps<{
  active?: boolean;
  locked?: boolean;
}>();

const isShaking = ref(false);

const triggerShake = () => {
  if (isShaking.value) return;
  isShaking.value = true;
  setTimeout(() => {
    isShaking.value = false;
  }, 400);
};
</script>

<template>
  <button
    class="inline-flex items-center gap-1.5 px-4 py-2 min-h-11 rounded-full border-0 text-base font-bold cursor-pointer transition-colors duration-200 ease-in-out"
    :class="[
      isShaking ? 'shake' : '',
      locked
        ? 'text-sub-color hover:bg-sub-color/20'
        : active
          ? 'bg-base-color text-background-color'
          : 'bg-transparent text-base-color hover:bg-sub-color/20',
    ]"
    @click="
      () => {
        if (locked) triggerShake();
      }
    "
  >
    <slot />
  </button>
</template>

<style scoped>
@keyframes shake {
  0%,
  100% {
    transform: translateX(0);
  }

  20% {
    transform: translateX(-3px);
  }

  40% {
    transform: translateX(3px);
  }

  60% {
    transform: translateX(-2.5px);
  }

  80% {
    transform: translateX(2px);
  }
}

.shake {
  animation: shake 0.3s ease-in-out;
}
</style>
