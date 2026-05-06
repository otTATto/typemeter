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
  <div class="flex flex-col h-screen bg-background-color">
    <!-- Error overlay -->
    <template v-if="listenerError">
      <div class="flex flex-col items-center justify-center h-screen gap-2">
        <p class="text-base text-[#e53e3e] m-0">Failed to start capturing keystrokes.</p>
        <p class="text-xs opacity-60 m-0 font-mono">{{ listenerError }}</p>
      </div>
    </template>

    <template v-else>
      <!-- Header -->
      <header class="flex items-center px-6 h-22 shrink-0">
        <div class="flex-1"><TabGroup /></div>
        <DayNav />
        <div class="flex-1 flex justify-end"><ThemeToggle /></div>
      </header>

      <!-- Main content card -->
      <main
        class="flex-1 flex flex-col items-center bg-pond-color rounded-t-[35px] pt-10 px-6 pb-6 overflow-hidden"
      >
        <!-- Date -->
        <p class="flex items-center gap-1 text-xl mb-4">
          <span class="text-base-color">{{ dateYear }}</span>
          <span class="text-sub-color">/</span>
          <span class="text-base-color">{{ dateMonth }}</span>
          <span class="text-sub-color">/</span>
          <span class="text-base-color">{{ dateDay }}</span>
        </p>

        <!-- Circular meter -->
        <div class="-mb-5">
          <MeterRing :value="todayTotal ?? 0" :goal="DAILY_GOAL" />
        </div>

        <!-- Keystroke count -->
        <p
          v-if="todayTotal !== null"
          class="flex justify-center text-[5rem] font-bold mb-8 leading-none"
        >
          <span
            v-for="(char, i) in todayTotal.toLocaleString('en-US').split('')"
            :key="i"
            :class="char === ',' ? 'count-sep' : 'count-digit'"
            >{{ char }}</span
          >
        </p>
        <p v-else class="text-base opacity-40 mb-8">Loading</p>

        <!-- Day stamps chart -->
        <DayStamps :today-total="todayTotal ?? 0" />
      </main>
    </template>
  </div>
</template>

<style scoped>
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
</style>
