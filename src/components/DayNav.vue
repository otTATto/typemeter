<script setup lang="ts">
import { ChevronLeft, ChevronRight } from 'lucide-vue-next';
</script>

<template>
  <div class="day-nav">
    <button class="icon-btn" aria-label="前の日">
      <ChevronLeft :size="20" />
    </button>
    <span class="nav-label">TODAY</span>
    <button class="icon-btn" aria-label="次の日">
      <ChevronRight :size="20" />
    </button>
  </div>
</template>

<style scoped>
.day-nav {
  display: flex;
  align-items: center;
  gap: 12px;
}

.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  border: none;
  background: transparent;
  color: var(--base-color);
  cursor: pointer;
  border-radius: 50%;
}

.nav-label {
  font-family: Manjari, Inter, sans-serif;
  font-size: 1rem;
  font-weight: 700;
  color: var(--base-color);
  min-width: 60px;
  text-align: center;
}
</style>
