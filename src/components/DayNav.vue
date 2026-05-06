<!-- 表示日を切り替える ‹ TODAY › ナビゲーション -->
<!-- App.vue ヘッダー中央に配置される -->
<script setup lang="ts">
import { ChevronLeft, ChevronRight } from 'lucide-vue-next';
</script>

<template>
  <div class="flex items-center gap-3">
    <button
      class="flex items-center justify-center size-11 border-0 bg-transparent text-base-color cursor-pointer rounded-full"
      aria-label="前の日"
    >
      <ChevronLeft :size="20" />
    </button>
    <span class="text-base font-bold text-base-color min-w-15 text-center">TODAY</span>
    <button
      class="flex items-center justify-center size-11 border-0 bg-transparent text-base-color cursor-pointer rounded-full"
      aria-label="次の日"
    >
      <ChevronRight :size="20" />
    </button>
  </div>
</template>
