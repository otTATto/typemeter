<!-- 今日の入力数を目標値に対する割合で示す SVG 円弧メーター -->
<!-- App.vue メインカードから <MeterRing :value="todayTotal" :goal="DAILY_GOAL" /> として呼ばれる -->
<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  value: number;
  goal: number;
}>();

const CX = 135;
const CY = 135;
const R = 108;
const STROKE_WIDTH = 5;

const arc = computed(() => {
  const pct = Math.min(props.value / props.goal, 1);
  if (pct === 0) return null;

  const startDeg = 180;
  const sweepDeg = pct * 360;
  const startRad = ((startDeg - 90) * Math.PI) / 180;
  const endRad = ((startDeg + sweepDeg - 90) * Math.PI) / 180;

  const x1 = CX + R * Math.cos(startRad);
  const y1 = CY + R * Math.sin(startRad);
  const x2 = CX + R * Math.cos(endRad);
  const y2 = CY + R * Math.sin(endRad);
  const largeArc = sweepDeg > 180 ? 1 : 0;

  return `M ${x1.toFixed(2)} ${y1.toFixed(2)} A ${R} ${R} 0 ${largeArc} 1 ${x2.toFixed(2)} ${y2.toFixed(2)}`;
});
</script>

<template>
  <svg :width="CX * 2" :height="CY * 2" :viewBox="`0 0 ${CX * 2} ${CY * 2}`" aria-hidden="true">
    <circle :cx="CX" :cy="CY" :r="R" fill="none" class="meter-track" :stroke-width="STROKE_WIDTH" />
    <path
      v-if="arc"
      :d="arc"
      fill="none"
      class="meter-progress"
      :stroke-width="STROKE_WIDTH"
      stroke-linecap="round"
    />
  </svg>
</template>

<style scoped>
.meter-track {
  stroke: var(--sub-color);
  opacity: 0.35;
}

.meter-progress {
  stroke: var(--accent-color);
}
</style>
