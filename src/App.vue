<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { computed, onMounted, onUnmounted, ref } from 'vue';

const todayCountAtStart = ref(0);
const sessionCount = ref(0);

const todayTotal = computed(() => todayCountAtStart.value + sessionCount.value);

let unlisten: (() => void) | null = null;

onMounted(async () => {
  todayCountAtStart.value = await invoke<number>('get_today_count');

  unlisten = await listen<number>('keystroke_update', (event) => {
    sessionCount.value = event.payload;
  });
});

onUnmounted(() => {
  unlisten?.();
});
</script>

<template>
  <main class="container">
    <p class="label">今日</p>
    <p class="count">{{ todayTotal.toLocaleString() }}</p>
  </main>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-synthesis: none;
  text-rendering: optimizelegibility;
  -webkit-font-smoothing: antialiased;
  color: #0f0f0f;
  background-color: #f6f6f6;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}
</style>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  gap: 8px;
}

.label {
  font-size: 1rem;
  opacity: 0.6;
  margin: 0;
}

.count {
  font-size: 4rem;
  font-weight: 700;
  letter-spacing: -0.02em;
  margin: 0;
}
</style>
