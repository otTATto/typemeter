<!-- 表示日を切り替える ‹ TODAY › ナビゲーション -->
<!-- App.vue ヘッダー中央に配置される -->
<script setup lang="ts">
import { ChevronLeft, ChevronRight } from 'lucide-vue-next';
import NavButton from '@/components/NavButton.vue';
</script>

<template>
  <div class="flex items-center gap-1">
    <NavButton aria-label="前の日">
      <ChevronLeft :size="20" />
    </NavButton>
    <NavButton><span class="translate-y-0.75">TODAY</span></NavButton>
    <NavButton aria-label="次の日">
      <ChevronRight :size="20" />
    </NavButton>
  </div>
</template>
