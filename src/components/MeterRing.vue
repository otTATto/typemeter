<!-- 今日の入力数を目標値に対する割合で示す SVG 円弧メーター -->
<!-- App.vue メインカードから <MeterRing :value="todayTotal" :goal="DAILY_GOAL" /> として呼ばれる -->
<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  value: number;
  goal: number;
}>();

/**
 * - CX           : SVG キャンバス中心 X 座標（px）
 * - CY           : SVG キャンバス中心 Y 座標（px）
 * - R            : 弧の半径（px）
 * - STROKE_WIDTH : 弧の線幅（px）
 */
const CX = 135;
const CY = 135;
const R = 108;
const STROKE_WIDTH = 8;

/**
 * 角度はすべて時計回り（0°=12時）で統一する
 * トラックは 225°（左下 5π/4）から始まり 270° （7π/4）分を描き、135°（右下 7π/4）で終わる
 *
 * - TRACK_START_DEG: トラック開始角度（時計角度）
 * - TRACK_SWEEP_DEG: トラックの掃引角度
 */
const TRACK_START_DEG = 225;
const TRACK_SWEEP_DEG = 270;

/**
 * @function 時計角度（0°=12時、時計回り）をSVG座標系のラジアンに変換する
 *
 * @param clockDeg 時計角度（度）
 * @returns SVG 座標系のラジアン値
 */
const toRad = (clockDeg: number) => ((clockDeg - 90) * Math.PI) / 180;

/**
 * @function 指定した開始角度・掃引角度のSVG円弧パス文字列を生成する
 *
 * @param startDeg 開始角度（時計角度・度）
 * @param sweepDeg 掃引角度（度）。正値で時計回りに描画される
 * @returns SVG `<path d="...">` に渡せる弧パス文字列
 */
const arcPath = (startDeg: number, sweepDeg: number) => {
  const endDeg = startDeg + sweepDeg;
  const x1 = CX + R * Math.cos(toRad(startDeg));
  const y1 = CY + R * Math.sin(toRad(startDeg));
  const x2 = CX + R * Math.cos(toRad(endDeg));
  const y2 = CY + R * Math.sin(toRad(endDeg));
  const largeArc = sweepDeg > 180 ? 1 : 0;
  return `M ${x1.toFixed(2)} ${y1.toFixed(2)} A ${R} ${R} 0 ${largeArc} 1 ${x2.toFixed(2)} ${y2.toFixed(2)}`;
};

const trackPath = arcPath(TRACK_START_DEG, TRACK_SWEEP_DEG);

const arc = computed(() => {
  const pct = Math.min(props.value / props.goal, 1);
  if (pct === 0) return null;
  return arcPath(TRACK_START_DEG, pct * TRACK_SWEEP_DEG);
});
</script>

<template>
  <svg :width="CX * 2" :height="CY * 2" :viewBox="`0 0 ${CX * 2} ${CY * 2}`" aria-hidden="true">
    <path
      :d="trackPath"
      fill="none"
      class="stroke-sub-color opacity-35"
      :stroke-width="STROKE_WIDTH"
      stroke-linecap="round"
    />
    <path
      v-if="arc"
      :d="arc"
      fill="none"
      class="stroke-accent-color"
      :stroke-width="STROKE_WIDTH"
      stroke-linecap="round"
    />
  </svg>
</template>
