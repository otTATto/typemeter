<!-- 今日の時間帯別タイプ数をドットグリッドで可視化するスタンプチャート -->
<!-- x 軸 = 時間帯（0〜23時）、y 軸 = カウントレベル（1000 刻み） -->
<!-- App.vue メインカードから <DayStamps /> として呼ばれる -->
<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue';
import { fetchHourlyCounts, subscribeKeystrokeUpdate } from '@/lib/keystroke';

const currentHour = new Date().getHours();

const hourlyData = ref<number[]>(Array(24).fill(0));

// 今日の日付のみリアルタイム更新する（他の日付は onMounted の一度取得のみ）
const todayDate = new Date().toLocaleDateString('en-CA'); // YYYY-MM-DD

const loadTodayHourlyData = async () => {
  hourlyData.value = await fetchHourlyCounts(todayDate);
};

let unlisten: (() => void) | null = null;

onMounted(async () => {
  await loadTodayHourlyData();
  unlisten = await subscribeKeystrokeUpdate(() => loadTodayHourlyData());
});

onUnmounted(() => {
  unlisten?.();
});

const hoveredHour = ref<number | null>(null);

/**
 * - LEVELS              : Y 軸のカウントレベル（1000 刻み）
 * - Y_AXIS_LABELS       : Y 軸に表示するレベルの抜粋
 * - ALWAYS_VISIBLE_HOURS: X 軸で常に sub-color 表示する時刻（3 時間刻み + 23 時）
 * - HOURS               : 時間インデックス（0〜23）の配列
 */
const LEVELS = [1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000];
const Y_AXIS_LABELS = [1000, 5000, 10000];
const ALWAYS_VISIBLE_HOURS = new Set([0, 3, 6, 9, 12, 15, 18, 21, 23]);
const HOURS = Array.from({ length: 24 }, (_, i) => i);

/**
 * SVG ジオメトリ。ドット端から目盛ラベルまで LABEL_GAP = 10px を確保する
 *
 * - DOT_R        : ドットの半径（px）
 * - CHART_LEFT   : ドットグリッド左端 X 座標。"10,000"（~36px）+ LABEL_GAP + DOT_R が収まる幅を確保
 * - CHART_RIGHT  : ドットグリッド右端 X 座標（CHART_LEFT と対称）
 * - CHART_TOP    : ドットグリッド上端 Y 座標。DOT_R 分の上端クリップを防ぐ余白
 * - CHART_BOTTOM : ドットグリッド下端 Y 座標。CHART_TOP 増分を加算して縦スパンを維持
 * - LABEL_GAP    : ドット端から目盛ラベルまでの余白（px）
 * - VIEWBOX_WIDTH: SVG viewBox の幅（px）
 */
const DOT_R = 8;
const CHART_LEFT = 60;
const CHART_RIGHT = 600;
const CHART_TOP = 46; // ツールチップ領域（22px）+ 三角形（7px）+ 余白（17px）
const CHART_BOTTOM = 296; // CHART_TOP 増分（+10）を加算
const LABEL_GAP = 10;
const VIEWBOX_WIDTH = 660;

/**
 * ツールチップ
 *
 * - TOOLTIP_MIN_W: ツールチップの最小幅（px）
 * - COL_W        : ドット列の間隔（px）。ホバー列の強調幅として使用
 */
const TOOLTIP_MIN_W = 80;
const COL_W = (CHART_RIGHT - CHART_LEFT) / 23;

/**
 * @function SVG 上のマウス座標から最近傍の列（時）を算出してホバー状態を更新する
 *
 * @param event MouseEvent（SVG 要素に紐付く）
 */
const onSvgMouseMove = (event: MouseEvent) => {
  const svgEl = event.currentTarget as SVGSVGElement;
  const { left, width } = svgEl.getBoundingClientRect();
  const svgX = ((event.clientX - left) / width) * VIEWBOX_WIDTH;
  const hour = Math.round(((svgX - CHART_LEFT) / (CHART_RIGHT - CHART_LEFT)) * 23);
  hoveredHour.value = hour >= 0 && hour <= 23 ? hour : null;
};

const onSvgMouseLeave = () => {
  hoveredHour.value = null;
};

/** ツールチップ X 座標（viewBox 端からはみ出さないようクランプ済み） */
const tooltipX = computed(() => {
  if (hoveredHour.value === null) return 0;
  const x = CHART_LEFT + (hoveredHour.value / 23) * (CHART_RIGHT - CHART_LEFT);
  const w = tooltipWidth.value;
  return Math.min(Math.max(x, w / 2), VIEWBOX_WIDTH - w / 2);
});

/** ツールチップに表示するテキスト */
const tooltipText = computed(() => {
  if (hoveredHour.value === null) return '';
  const count = hourlyData.value[hoveredHour.value] ?? 0;
  return `${hoveredHour.value}:00 — ${count.toLocaleString('en-US')}`;
});

/** ツールチップテキスト要素への参照（getComputedTextLength で動的幅算出に使用） */
const tooltipTextEl = ref<SVGTextElement | null>(null);

/** ツールチップの現在幅（テキスト長 + 水平パディング 24px、最小 TOOLTIP_MIN_W） */
const tooltipWidth = ref(TOOLTIP_MIN_W);

watch(tooltipText, async () => {
  await nextTick();
  tooltipWidth.value = tooltipTextEl.value
    ? Math.max(Math.ceil(tooltipTextEl.value.getComputedTextLength()) + 24, TOOLTIP_MIN_W)
    : TOOLTIP_MIN_W;
});

/**
 * 目盛ラベルの座標。両側の Y 軸ラベルは text-anchor="end" で右端揃え
 *
 * - Y_LABEL_LEFT_X : 左ラベルの右端 X 座標（= ドット左端 − LABEL_GAP）
 * - Y_LABEL_RIGHT_X: 右ラベルの右端 X 座標（= viewBox 右端 − 左端余白 6px、ドット右端との差 = LABEL_GAP）
 * - X_LABEL_Y      : X 軸ラベルのベースライン Y 座標（= ドット下端 + LABEL_GAP + Manjari 12px キャップハイト ~9px）
 */
const Y_LABEL_LEFT_X = CHART_LEFT - DOT_R - LABEL_GAP;
const Y_LABEL_RIGHT_X = VIEWBOX_WIDTH - 6;
const X_LABEL_Y = CHART_BOTTOM + DOT_R + LABEL_GAP + 9;

const dotX = (hour: number) => CHART_LEFT + (hour / 23) * (CHART_RIGHT - CHART_LEFT);

const dotY = (level: number) =>
  CHART_BOTTOM - ((level - 1000) / (10000 - 1000)) * (CHART_BOTTOM - CHART_TOP);

/**
 * @function ドットの SVG fill 値を返す
 *
 * @param hour 時（0〜23）
 * @param level カウントレベル（1000 刻み）
 * @returns CSS fill 値
 *
 * NOTE:
 *   count ≥ level なら accent-color、count ≤ level - 1000 なら background-color。
 *   その中間では oklch 空間で比率ブレンドした color-mix() 値を返す。
 */
const dotFill = (hour: number, level: number): string => {
  const count = hourlyData.value[hour];
  if (count >= level) return 'var(--accent-color)';
  const prevLevel = level - 1000;
  if (count <= prevLevel) return 'var(--background-color)';
  const pct = Math.round(((count - prevLevel) / 1000) * 100);
  return `color-mix(in oklch, var(--accent-color) ${pct}%, var(--pond-color))`;
};

/**
 * @function X 軸ラベルの CSS クラスを返す
 *
 * @param hour 時（0〜23）
 * @returns Tailwind クラス文字列
 *
 * NOTE:
 *   - 現在時刻: accent-color（アクティブ）
 *   - ALWAYS_VISIBLE_HOURS に含まれる時刻: sub-color（常時表示）
 *   - それ以外: pond-color（実質不可視）
 */
const labelClass = (hour: number): string => {
  if (hour === currentHour) return 'fill-accent-color! font-bold';
  if (ALWAYS_VISIBLE_HOURS.has(hour)) return 'fill-sub-color';
  return 'fill-pond-color';
};
</script>

<template>
  <div class="w-full max-w-165 shrink-0">
    <svg
      width="100%"
      viewBox="0 0 660 330"
      aria-label="今日の時間帯別タイプ数チャート"
      @mousemove="onSvgMouseMove"
      @mouseleave="onSvgMouseLeave"
    >
      <!-- Y-axis labels (left) -->
      <text
        v-for="label in Y_AXIS_LABELS"
        :key="`yl-${label}`"
        :x="Y_LABEL_LEFT_X"
        :y="dotY(label) + 4"
        class="text-xs fill-sub-color"
        text-anchor="end"
      >
        {{ label.toLocaleString('en-US') }}
      </text>

      <!-- Y-axis labels (right) -->
      <text
        v-for="label in Y_AXIS_LABELS"
        :key="`yr-${label}`"
        :x="Y_LABEL_RIGHT_X"
        :y="dotY(label) + 4"
        class="text-xs fill-sub-color"
        text-anchor="end"
      >
        {{ label.toLocaleString('en-US') }}
      </text>

      <!-- Column highlight on hover -->
      <rect
        v-if="hoveredHour !== null"
        :x="dotX(hoveredHour) - COL_W / 2"
        :y="CHART_TOP - DOT_R"
        :width="COL_W"
        :height="CHART_BOTTOM - CHART_TOP + DOT_R * 2"
        :rx="COL_W / 2"
        style="fill: var(--background-color)"
      />

      <!-- Dot grid: 24 columns (hours) × 10 rows (levels) -->
      <template v-for="hour in HOURS" :key="`col-${hour}`">
        <circle
          v-for="level in LEVELS"
          :key="`dot-${hour}-${level}`"
          :cx="dotX(hour)"
          :cy="dotY(level)"
          :r="DOT_R"
          :style="{ fill: dotFill(hour, level) }"
        />
      </template>

      <!-- X-axis labels -->
      <text
        v-for="hour in HOURS"
        :key="hour"
        :x="dotX(hour)"
        :y="X_LABEL_Y"
        class="text-xs"
        :class="labelClass(hour)"
        text-anchor="middle"
      >
        {{ hour }}
      </text>
      <!-- Tooltip -->
      <g v-if="hoveredHour !== null">
        <polygon
          :points="`${tooltipX - 6},27 ${tooltipX + 6},27 ${tooltipX},34`"
          style="fill: var(--base-color)"
        />
        <rect
          :x="tooltipX - tooltipWidth / 2"
          y="5"
          :width="tooltipWidth"
          height="22"
          rx="11"
          style="fill: var(--base-color)"
        />
        <text
          ref="tooltipTextEl"
          :x="tooltipX"
          y="20"
          class="text-xs"
          style="fill: var(--pond-color)"
          text-anchor="middle"
        >
          {{ tooltipText }}
        </text>
      </g>
    </svg>
  </div>
</template>
