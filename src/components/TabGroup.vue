<!-- DAY / DAYS 表示切り替えタブ -->
<!-- App.vue ヘッダー左側に配置される -->
<script setup lang="ts">
import { CalendarDays, CalendarRange } from 'lucide-vue-next';
</script>

<template>
  <div class="flex items-center gap-1" role="tablist">
    <button
      class="flex items-center gap-1.5 px-4 py-2 rounded-[33px] border-0 bg-base-color text-background-color font-bold cursor-pointer min-h-11"
      role="tab"
      aria-selected="true"
    >
      <CalendarDays :size="18" />
      <span>DAY</span>
    </button>
    <button
      class="flex items-center gap-1.5 px-4 py-2 rounded-[33px] border-0 bg-transparent text-base-color font-bold cursor-pointer min-h-11"
      role="tab"
      aria-selected="false"
    >
      <CalendarRange :size="18" />
      <span>DAYS</span>
    </button>
  </div>
</template>
