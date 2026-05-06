<!-- DAY / DAYS 表示切り替えタブ -->
<!-- App.vue ヘッダー左側に配置される -->
<script setup lang="ts">
import { CalendarDays, CalendarRange } from 'lucide-vue-next';
import NavButton from '@/components/NavButton.vue';
</script>

<template>
  <div class="flex items-center gap-1" role="tablist">
    <NavButton :active="true" role="tab" aria-selected="true">
      <CalendarDays :size="18" />
      <span class="translate-y-0.75">DAY</span>
    </NavButton>
    <NavButton role="tab" aria-selected="false">
      <CalendarRange :size="18" />
      <span class="translate-y-0.75">DAYS</span>
    </NavButton>
  </div>
</template>
