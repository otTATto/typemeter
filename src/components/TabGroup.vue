<!-- DAY / DAYS 表示切り替えタブ -->
<!-- App.vue ヘッダー左側に配置される -->
<script setup lang="ts">
import { CalendarDays, CalendarRange } from 'lucide-vue-next';
</script>

<template>
  <div class="tab-group" role="tablist">
    <button class="tab tab-active" role="tab" aria-selected="true">
      <CalendarDays :size="18" />
      <span>DAY</span>
    </button>
    <button class="tab" role="tab" aria-selected="false">
      <CalendarRange :size="18" />
      <span>DAYS</span>
    </button>
  </div>
</template>

<style scoped>
.tab-group {
  display: flex;
  align-items: center;
  gap: 4px;
}

.tab {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border-radius: 33px;
  border: none;
  background: transparent;
  color: var(--base-color);
  font-family: Manjari, Inter, sans-serif;
  font-size: 1rem;
  font-weight: 700;
  cursor: pointer;
  min-height: 44px;
}

.tab-active {
  background-color: var(--base-color);
  color: var(--background-color);
}
</style>
