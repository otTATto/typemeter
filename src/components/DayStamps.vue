<!-- 過去 N 日分の入力数をドットグリッドで可視化するスタンプチャート -->
<!-- x 軸 = 日インデックス（0 = 最古、TODAY_INDEX = 今日）-->
<!-- y 軸 = カウントレベル（1000 刻み）、そのレベルに到達した日のドットを緑で塗りつぶす -->
<!-- App.vue メインカードから <DayStamps :today-total="todayTotal" /> として呼ばれる -->
<script setup lang="ts">
const props = defineProps<{
  todayTotal: number;
}>();

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

const DAY_INDICES = Array.from({ length: TOTAL_DAYS }, (_, i) => i);

const dotX = (dayIdx: number) =>
  CHART_LEFT + (dayIdx / (TOTAL_DAYS - 1)) * (CHART_RIGHT - CHART_LEFT);

const dotY = (level: number) =>
  CHART_BOTTOM - ((level - 1000) / (10000 - 1000)) * (CHART_BOTTOM - CHART_TOP);

const dayTotal = (dayIdx: number) =>
  dayIdx === TODAY_INDEX ? props.todayTotal : (MOCK_DAY_TOTALS[dayIdx] ?? 0);

const dotFill = (dayIdx: number, level: number) =>
  dayTotal(dayIdx) >= level ? 'var(--accent-color)' : 'var(--sub-color)';

const dotOpacity = (dayIdx: number, level: number) => {
  if (dayTotal(dayIdx) < level) return 0.3;
  const age = TODAY_INDEX - dayIdx;
  return Math.max(0.4, 1 - age * 0.06);
};
</script>

<template>
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
      <template v-for="dayIdx in DAY_INDICES" :key="`col-${dayIdx}`">
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
</template>

<style scoped>
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
</style>
