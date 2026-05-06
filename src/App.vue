<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { subscribeKeystrokeUpdate, subscribeListenerError } from './lib/keystroke';

const todayTotal = ref<number | null>(null);
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
      <p class="error-label">Failed to start capturing keystrokes.</p>
      <p class="error-detail">{{ listenerError }}</p>
    </template>
    <template v-else-if="todayTotal !== null">
      <p class="label">TODAY</p>
      <p class="count">{{ todayTotal.toLocaleString('en-US') }}</p>
    </template>
    <template v-else>
      <p class="loading">Loading</p>
    </template>
  </main>
</template>

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

.loading {
  font-size: 1rem;
  opacity: 0.4;
  margin: 0;
}
</style>
