<!-- ナビゲーション共通ボタン -->
<!-- ホバー時: bg-pond-color / active 時: bg-base-color -->
<!-- TabGroup・DayNav 内のボタンから使用される -->
<script setup lang="ts">
defineProps<{
  active?: boolean;
  disabled?: boolean;
}>();
</script>

<template>
  <button
    :disabled="disabled"
    class="inline-flex items-center gap-1.5 px-4 py-2 min-h-11 rounded-full border-0 text-base font-bold transition-colors duration-200 ease-in-out"
    :class="
      disabled
        ? 'text-sub-color cursor-default'
        : active
          ? 'bg-base-color text-background-color cursor-pointer'
          : 'bg-transparent text-base-color hover:bg-sub-color/20 cursor-pointer'
    "
  >
    <slot />
  </button>
</template>
