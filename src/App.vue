<script setup lang="ts">
import { CalendarDays, ChevronLeft, ChevronRight, Sun, CalendarRange } from 'lucide-vue-next';
import { computed, onMounted, onUnmounted, ref } from 'vue';
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

const now = new Date();
const dateYear = now.getFullYear();
const dateMonth = String(now.getMonth() + 1).padStart(2, '0');
const dateDay = String(now.getDate()).padStart(2, '0');

// Circular meter
const DAILY_GOAL = 10000;
const METER_CX = 135;
const METER_CY = 135;
const METER_R = 108;
const STROKE_WIDTH = 5;

const meterArc = computed(() => {
  const total = todayTotal.value ?? 0;
  const pct = Math.min(total / DAILY_GOAL, 1);
  if (pct === 0) return null;

  // Start angle: 180deg (9 o'clock), sweep clockwise
  const startDeg = 180;
  const sweepDeg = pct * 360;
  const startRad = ((startDeg - 90) * Math.PI) / 180;
  const endRad = ((startDeg + sweepDeg - 90) * Math.PI) / 180;

  const x1 = METER_CX + METER_R * Math.cos(startRad);
  const y1 = METER_CY + METER_R * Math.sin(startRad);
  const x2 = METER_CX + METER_R * Math.cos(endRad);
  const y2 = METER_CY + METER_R * Math.sin(endRad);
  const largeArc = sweepDeg > 180 ? 1 : 0;

  return `M ${x1.toFixed(2)} ${y1.toFixed(2)} A ${METER_R} ${METER_R} 0 ${largeArc} 1 ${x2.toFixed(2)} ${y2.toFixed(2)}`;
});

// Day stamps chart:
//   x-axis = day index (0 = oldest, TODAY_INDEX = today)
//   y-axis = count level (1000, 2000, ..., 10000)
//   Each dot is green if that day reached that level, gray otherwise.
const TOTAL_DAYS = 24;
const TODAY_INDEX = 10;

/** Mock daily totals; index = day index (TODAY_INDEX = today) */
const MOCK_DAY_TOTALS: Record<number, number> = {
  1: 1100,
  2: 3200,
  3: 5800,
  4: 7200,
  5: 6100,
  6: 8900,
  7: 3200,
  8: 5400,
  9: 9100,
};

const LEVELS = [1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000];
const X_AXIS_LABELS = [0, 3, 6, 9, 10, 12, 15, 18, 21, 23];
const Y_AXIS_LABELS = [1000, 5000, 10000];

const DOT_R = 8;
const CHART_LEFT = 36;
const CHART_RIGHT = 624;
const CHART_TOP = 0;
const CHART_BOTTOM = 250;

const dotX = (dayIdx: number) =>
  CHART_LEFT + (dayIdx / (TOTAL_DAYS - 1)) * (CHART_RIGHT - CHART_LEFT);

const dotY = (level: number) =>
  CHART_BOTTOM - ((level - 1000) / (10000 - 1000)) * (CHART_BOTTOM - CHART_TOP);

const isDotFilled = (dayIdx: number, level: number) => {
  const total = dayIdx === TODAY_INDEX ? (todayTotal.value ?? 0) : (MOCK_DAY_TOTALS[dayIdx] ?? 0);
  return total >= level;
};

const dotFill = (dayIdx: number, level: number) => {
  if (!isDotFilled(dayIdx, level)) return 'var(--sub-color)';
  return 'var(--accent-color)';
};

const dotOpacity = (dayIdx: number, level: number) => {
  if (!isDotFilled(dayIdx, level)) return 0.3;
  const age = TODAY_INDEX - dayIdx;
  return Math.max(0.4, 1 - age * 0.06);
};
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
        <!-- DAY / DAYS tabs -->
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

        <!-- Date navigation -->
        <div class="day-nav">
          <button class="icon-btn" aria-label="前の日">
            <ChevronLeft :size="20" />
          </button>
          <span class="nav-label">TODAY</span>
          <button class="icon-btn" aria-label="次の日">
            <ChevronRight :size="20" />
          </button>
        </div>

        <!-- Theme toggle (visual only — CSS media query handles actual switching) -->
        <button class="theme-toggle" aria-label="テーマ切り替え">
          <span class="toggle-track">
            <span class="toggle-thumb">
              <Sun :size="12" />
            </span>
          </span>
        </button>
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
          <svg
            :width="METER_CX * 2"
            :height="METER_CY * 2"
            :viewBox="`0 0 ${METER_CX * 2} ${METER_CY * 2}`"
            aria-hidden="true"
          >
            <!-- Track -->
            <circle
              :cx="METER_CX"
              :cy="METER_CY"
              :r="METER_R"
              fill="none"
              class="meter-track"
              :stroke-width="STROKE_WIDTH"
            />
            <!-- Progress arc -->
            <path
              v-if="meterArc"
              :d="meterArc"
              fill="none"
              class="meter-progress"
              :stroke-width="STROKE_WIDTH"
              stroke-linecap="round"
            />
          </svg>
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
        <div class="stamps-wrap">
          <svg width="100%" viewBox="0 0 660 290" aria-label="過去の入力数チャート">
            <!-- Y-axis labels (left) -->
            <text
              v-for="label in Y_AXIS_LABELS"
              :key="`yl-${label}`"
              :x="CHART_LEFT - 6"
              :y="dotY(label) + 4"
              class="chart-label"
              text-anchor="end"
            >
              {{ label.toLocaleString('en-US') }}
            </text>

            <!-- Y-axis labels (right) -->
            <text
              v-for="label in Y_AXIS_LABELS"
              :key="`yr-${label}`"
              :x="CHART_RIGHT + 6"
              :y="dotY(label) + 4"
              class="chart-label"
              text-anchor="start"
            >
              {{ label.toLocaleString('en-US') }}
            </text>

            <!-- Dot grid: TOTAL_DAYS columns × LEVELS rows -->
            <template
              v-for="dayIdx in Array.from({ length: TOTAL_DAYS }, (_, i) => i)"
              :key="`col-${dayIdx}`"
            >
              <circle
                v-for="level in LEVELS"
                :key="`dot-${dayIdx}-${level}`"
                :cx="dotX(dayIdx)"
                :cy="dotY(level)"
                :r="DOT_R"
                :fill="dotFill(dayIdx, level)"
                :opacity="dotOpacity(dayIdx, level)"
              />
            </template>

            <!-- X-axis labels -->
            <g transform="translate(0, 265)">
              <text
                v-for="label in X_AXIS_LABELS"
                :key="label"
                :x="dotX(label)"
                y="0"
                class="chart-label"
                :class="{ 'chart-label-accent': label === TODAY_INDEX }"
                text-anchor="middle"
              >
                {{ label }}
              </text>
            </g>
          </svg>
        </div>
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

.tab-group {
  display: flex;
  align-items: center;
  gap: 4px;
  background-color: transparent;
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

.theme-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 54px;
  height: 44px;
  border: none;
  background: transparent;
  cursor: pointer;
  padding: 0;
}

.toggle-track {
  display: flex;
  align-items: center;
  width: 54px;
  height: 28px;
  background-color: var(--sub-color);
  border-radius: 14px;
  padding: 2px;
}

.toggle-thumb {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background-color: var(--pond-color);
  border-radius: 50%;
  color: var(--base-color);
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

.meter-track {
  stroke: var(--sub-color);
  opacity: 0.35;
}

.meter-progress {
  stroke: var(--accent-color);
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

/* ── Stamps chart ── */
.stamps-wrap {
  width: 100%;
  max-width: 660px;
  flex-shrink: 0;
}

.chart-label {
  font-family: Manjari, Inter, sans-serif;
  font-size: 12px;
  fill: var(--sub-color);
}

.chart-label-accent {
  fill: var(--accent-color);
  font-weight: 700;
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
