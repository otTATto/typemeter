<!-- ナビゲーション共通ボタン -->
<!-- ホバー時: bg-pond-color / active 時: bg-base-color -->
<!-- TabGroup・DayNav 内のボタンから使用される -->
<script setup lang="ts">
defineProps<{
  active?: boolean;
}>();
</script>

<template>
  <button
    class="inline-flex items-center gap-1.5 px-4 py-2 min-h-11 rounded-full border-0 text-base font-bold cursor-pointer transition-colors duration-200 ease-in-out"
    :class="
      active
        ? 'bg-base-color text-background-color'
        : 'bg-transparent text-base-color hover:bg-sub-color/20'
    "
  >
    <slot />
  </button>
</template>
