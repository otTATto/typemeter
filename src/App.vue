<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { subscribeKeystrokeUpdate, subscribeListenerError } from './lib/keystroke';

const todayTotal = ref(0);
const listenerError = ref<string | null>(null);

const unlisteners: Array<() => void> = [];

onMounted(async () => {
  unlisteners.push(
    await subscribeKeystrokeUpdate((total) => {
      todayTotal.value = total;
    }),
    await subscribeListenerError((message) => {
      listenerError.value = message;
    }),
  );
});

onUnmounted(() => {
  unlisteners.forEach((fn) => fn());
});
</script>

<template>
  <main class="container">
    <template v-if="listenerError">
      <p class="error-label">キーボード監視を開始できませんでした</p>
      <p class="error-detail">{{ listenerError }}</p>
    </template>
    <template v-else>
      <p class="label">今日</p>
      <p class="count">{{ todayTotal.toLocaleString() }}</p>
    </template>
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

.error-label {
  font-size: 1rem;
  color: #e53e3e;
  margin: 0;
}

.error-detail {
  font-size: 0.75rem;
  opacity: 0.6;
  margin: 0;
  font-family: monospace;
}
</style>
