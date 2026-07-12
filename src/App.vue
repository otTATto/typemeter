<script setup lang="ts">
import { platform } from '@tauri-apps/plugin-os';
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import AlwaysOnTopToggle from '@/components/AlwaysOnTopToggle.vue';
import DayNav from '@/components/DayNav.vue';
import DayStamps from '@/components/DayStamps.vue';
import MeterRing from '@/components/MeterRing.vue';
import TabGroup from '@/components/TabGroup.vue';
import TitleBar from '@/components/TitleBar.vue';
import TitleBarOffset from '@/components/TitleBarOffset.vue';
import UpdateModal from '@/components/UpdateModal.vue';
import { formatDate } from '@/lib/date';
import {
  fetchHourlyCounts,
  subscribeKeystrokeUpdate,
  subscribeListenerError,
} from '@/lib/keystroke';
import { initTheme } from '@/lib/theme';
import { checkForUpdate, type UpdateInfo } from '@/lib/update';

const todayDate = ref(formatDate(new Date()));
const targetDate = ref(todayDate.value);
const isToday = computed(() => targetDate.value === todayDate.value);

const todayTotal = ref<number | null>(null);
const pastTotal = ref<number | null>(null);
const displayTotal = computed(() => (isToday.value ? todayTotal.value : pastTotal.value));

const listenerError = ref<string | null>(null);
const updateInfo = ref<UpdateInfo | null>(null);

const unlisteners: Array<() => void> = [];

// macOS はネイティブメニューバーを使うため、Windows/Linux のみカスタムタイトルバーを表示する
// UA による初期値で初回レンダリングのちらつきを防ぎ、plugin-os で確定する
const showTitleBar = ref(!navigator.userAgent.includes('Macintosh'));

let cleanupTheme: (() => void) | null = null;

onMounted(async () => {
  cleanupTheme = initTheme();
  showTitleBar.value = (await platform()) !== 'macos';
  checkForUpdate().then((info) => {
    updateInfo.value = info;
  });
  unlisteners.push(
    await subscribeKeystrokeUpdate((total) => {
      todayTotal.value = total;
      const newTodayDate = formatDate(new Date());
      if (newTodayDate !== todayDate.value) {
        const wasToday = targetDate.value === todayDate.value;
        todayDate.value = newTodayDate;
        if (wasToday) {
          targetDate.value = newTodayDate;
        }
      }
    }),
    await subscribeListenerError((message) => {
      listenerError.value = message;
    }),
  );
});

onUnmounted(() => {
  unlisteners.forEach((fn) => fn());
  cleanupTheme?.();
});

// < は過去方向（右スライド）、> / TODAY は未来方向（左スライド）
const transitionDirection = ref<'left' | 'right'>('left');

watch(targetDate, async (newDate, oldDate) => {
  transitionDirection.value = newDate < oldDate ? 'right' : 'left';
  pastTotal.value = null;

  if (newDate === todayDate.value) {
    return;
  }
  try {
    const counts = await fetchHourlyCounts(newDate);
    pastTotal.value = counts.reduce((a, b) => a + b, 0);
  } catch (err) {
    console.error('[App] fetchHourlyCounts failed:', err);
  }
});

const targetDateParts = computed(() => {
  const [year, month, day] = targetDate.value.split('-');
  return { year, month, day };
});

const displayChars = computed(() =>
  displayTotal.value !== null ? displayTotal.value.toLocaleString('en-US').split('') : [],
);

const DAILY_GOAL = 10000;
</script>

<template>
  <div class="flex flex-col h-screen bg-background-color">
    <TitleBar v-if="showTitleBar" />

    <!-- Error overlay -->
    <template v-if="listenerError">
      <div class="flex flex-col items-center justify-center h-screen gap-2">
        <p class="text-base text-danger-color m-0">Failed to start capturing keystrokes.</p>
        <p class="text-xs opacity-60 m-0 font-mono">{{ listenerError }}</p>
      </div>
    </template>

    <template v-else>
      <TitleBarOffset :has-title-bar="showTitleBar" />
      <UpdateModal
        v-if="updateInfo"
        :version="updateInfo.version"
        :current-version="updateInfo.currentVersion"
        :release-url="updateInfo.releaseUrl"
        @dismiss="updateInfo = null"
      />

      <!-- Header -->
      <header class="flex items-center px-6 h-22 shrink-0">
        <div class="flex-1"><TabGroup /></div>
        <DayNav v-model="targetDate" :today-date="todayDate" />
        <div class="flex-1 flex justify-end"><AlwaysOnTopToggle /></div>
      </header>

      <!-- Main content card（背景は固定、内部コンテンツのみスライド） -->
      <main class="flex-1 relative overflow-hidden bg-pond-color rounded-t-[35px]">
        <Transition :name="`slide-${transitionDirection}`">
          <div
            :key="targetDate"
            class="content-area absolute inset-0 flex flex-col items-center pt-10 px-6 pb-6 overflow-y-auto"
          >
            <!-- Date -->
            <p class="flex items-center gap-1 text-xl mb-4">
              <span class="text-base-color">{{ targetDateParts.year }}</span>
              <span class="text-sub-color">/</span>
              <span class="text-base-color">{{ targetDateParts.month }}</span>
              <span class="text-sub-color">/</span>
              <span class="text-base-color">{{ targetDateParts.day }}</span>
            </p>

            <!-- Circular meter -->
            <div class="-mb-5">
              <MeterRing :value="displayTotal ?? 0" :goal="DAILY_GOAL" />
            </div>

            <!-- Keystroke count -->
            <!--
              外側の span がスロット（幅・高さ固定 + overflow-hidden）。
              内側の span は :key="char" で値が変わると enter/leave アニメーションする。
            -->
            <p
              v-if="displayTotal !== null"
              class="flex justify-center text-[5rem] font-bold mb-8 leading-none"
            >
              <span
                v-for="(char, i) in displayChars"
                :key="displayChars.length - i"
                :class="char === ',' ? 'count-sep' : 'count-digit'"
                class="relative h-[1em]"
              >
                <Transition name="digit-roll">
                  <span :key="char" class="absolute inset-0 flex items-center justify-center">{{
                    char
                  }}</span>
                </Transition>
              </span>
            </p>
            <p v-else class="flex items-center justify-center h-20 mb-8 text-base opacity-40">
              Loading
            </p>

            <!-- Day stamps chart -->
            <DayStamps :date="targetDate" class="-mt-8" />
          </div>
        </Transition>
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

.content-area {
  scrollbar-width: none;
}

.content-area::-webkit-scrollbar {
  display: none;
}

/* < ボタン: 旧コンテンツが右へ退場、新コンテンツが左から登場 */
.slide-right-enter-active {
  transition:
    transform 0.2s ease-out,
    opacity 0.15s ease,
    filter 0.3s ease;
}

.slide-right-leave-active {
  transition:
    transform 0.2s ease-out,
    opacity 0.1s ease,
    filter 0.3s ease;
}

.slide-right-enter-from {
  transform: translateX(-100%);
  opacity: 0;
  filter: blur(25px);
}

.slide-right-leave-to {
  transform: translateX(100%);
  opacity: 0;
  filter: blur(25px);
}

/* > / TODAY ボタン: 旧コンテンツが左へ退場、新コンテンツが右から登場 */
.slide-left-enter-active {
  transition:
    transform 0.2s ease-out,
    opacity 0.15s ease,
    filter 0.3s ease;
}

.slide-left-leave-active {
  transition:
    transform 0.2s ease-out,
    opacity 0.1s ease,
    filter 0.3s ease;
}

.slide-left-enter-from {
  transform: translateX(100%);
  opacity: 0;
  filter: blur(25px);
}

.slide-left-leave-to {
  transform: translateX(-100%);
  opacity: 0;
  filter: blur(25px);
}

/* 桁アニメーション: 旧桁が下へ退場、新桁が上から登場 */
.digit-roll-enter-active,
.digit-roll-leave-active {
  transition:
    transform 0.2s ease,
    opacity 0.1s ease,
    filter 0.3s ease;
}

.digit-roll-enter-from {
  transform: translateY(-100%);
  opacity: 0;
  filter: blur(6px);
}

.digit-roll-leave-to {
  transform: translateY(100%);
  opacity: 0;
  filter: blur(6px);
}
</style>
