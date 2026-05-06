<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import DayNav from '@/components/DayNav.vue';
import DayStamps from '@/components/DayStamps.vue';
import MeterRing from '@/components/MeterRing.vue';
import TabGroup from '@/components/TabGroup.vue';
import ThemeToggle from '@/components/ThemeToggle.vue';
import { subscribeKeystrokeUpdate, subscribeListenerError } from '@/lib/keystroke';

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

const now = new Date();
const dateYear = now.getFullYear();
const dateMonth = String(now.getMonth() + 1).padStart(2, '0');
const dateDay = String(now.getDate()).padStart(2, '0');

const DAILY_GOAL = 10000;
</script>

<template>
  <div class="app-root">
    <!-- Error overlay -->
    <template v-if="listenerError">
      <div class="error-container">
        <p class="error-label">Failed to start capturing keystrokes.</p>
        <p class="error-detail">{{ listenerError }}</p>
      </div>
    </template>

    <template v-else>
      <!-- Header -->
      <header class="nav">
        <TabGroup />
        <DayNav />
        <ThemeToggle />
      </header>

      <!-- Main content card -->
      <main class="card">
        <!-- Date -->
        <p class="date">
          <span class="date-part">{{ dateYear }}</span>
          <span class="date-sep">/</span>
          <span class="date-part">{{ dateMonth }}</span>
          <span class="date-sep">/</span>
          <span class="date-part">{{ dateDay }}</span>
        </p>

        <!-- Circular meter -->
        <div class="meter-wrap">
          <MeterRing :value="todayTotal ?? 0" :goal="DAILY_GOAL" />
        </div>

        <!-- Keystroke count -->
        <p v-if="todayTotal !== null" class="count">
          <span
            v-for="(char, i) in todayTotal.toLocaleString('en-US').split('')"
            :key="i"
            :class="char === ',' ? 'count-sep' : 'count-digit'"
            >{{ char }}</span
          >
        </p>
        <p v-else class="loading">Loading</p>

        <!-- Day stamps chart -->
        <DayStamps :today-total="todayTotal ?? 0" />
      </main>
    </template>
  </div>
</template>

<style scoped>
.app-root {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--background-color);
}

/* ── Header ── */
.nav {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  height: 88px;
  flex-shrink: 0;
}

/* ── Card ── */
.card {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  background-color: var(--pond-color);
  border-radius: 35px 35px 0 0;
  padding: 40px 24px 24px;
  overflow: hidden;
}

/* ── Date ── */
.date {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 1.25rem;
  margin: 0 0 32px;
}

.date-part {
  color: var(--base-color);
}

.date-sep {
  color: var(--sub-color);
}

/* ── Meter ── */
.meter-wrap {
  margin-bottom: 24px;
}

/* ── Count ── */
.count {
  display: flex;
  justify-content: center;
  font-size: 5rem;
  font-weight: 700;
  margin: 0 0 32px;
  line-height: 1;
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

/* ── Error ── */
.error-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  gap: 8px;
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

/* ── Loading ── */
.loading {
  font-size: 1rem;
  opacity: 0.4;
  margin: 0 0 32px;
}
</style>
