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
      <p class="count">
        <span
          v-for="(char, i) in todayTotal.toLocaleString('en-US').split('')"
          :key="i"
          :class="char === ',' ? 'count-sep' : 'count-digit'"
          >{{ char }}</span
        >
      </p>
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
  display: flex;
  justify-content: center;
  font-size: 4rem;
  font-weight: 700;
  margin: 0;
}

/*
 * Manjari は tnum OpenType フィーチャーを持たないため font-variant-numeric: tabular-nums が効かない。
 * 代わりに各桁を固定幅の span で囲み、数値更新時の横揺れを防ぐ。
 *
 * 幅は fontTools で計測した advance width をもとに設定（単位: em / UPM = 2048）:
 *   0 = 0.596em  1 = 0.377em  2 = 0.548em  3 = 0.508em  4 = 0.637em (最大)
 *   5 = 0.555em  6 = 0.584em  7 = 0.574em  8 = 0.523em  9 = 0.584em
 *   , = 0.247em
 *
 * digit の width は最大幅の 4 (0.637em) を基準に 0.65em、
 * sep の width はカンマ (0.247em) を基準に 0.25em としている。
 *
 * margin-inline の負値は固定幅ボックス内に生じる余白を詰め、
 * 以前の letter-spacing: -0.02em に相当する字間を再現する。
 */

.count-digit {
  width: 0.65em;
  margin-inline: -0.03em;
  text-align: center;
}

.count-sep {
  width: 0.25em;
  margin-inline: -0.05em;
  text-align: center;
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
