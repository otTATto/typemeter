<!-- ライト / ダークモード切り替えトグルスイッチ -->
<!-- Settings ウィンドウの外観設定行に配置される -->
<script setup lang="ts">
import { Moon, Sun } from 'lucide-vue-next';
import { onMounted, onUnmounted, ref } from 'vue';
import { getTheme, onThemeChange, setTheme, type Theme } from '@/lib/settings';

const isDark = ref(false);

const resolveIsDark = (theme: Theme | null): boolean =>
  theme === 'dark' || (theme === null && window.matchMedia('(prefers-color-scheme: dark)').matches);

const toggle = () => {
  const next = !isDark.value;
  isDark.value = next;
  setTheme(next ? 'dark' : 'light').catch((err) => {
    console.error('[ThemeToggle] failed to save theme:', err);
    isDark.value = !next;
  });
};

let unlistenThemeChange: (() => void) | null = null;

onMounted(async () => {
  isDark.value = resolveIsDark(await getTheme());
  unlistenThemeChange = await onThemeChange((theme) => {
    isDark.value = resolveIsDark(theme);
  });
});

onUnmounted(() => {
  unlistenThemeChange?.();
});
</script>

<template>
  <button
    class="flex items-center justify-center w-13.5 h-11 border-0 bg-transparent cursor-pointer p-0"
    :aria-label="isDark ? 'ライトモードに切り替え' : 'ダークモードに切り替え'"
    :aria-pressed="isDark"
    @click="toggle"
  >
    <span class="flex items-center w-13.5 h-7 bg-sub-color rounded-[14px] p-0.5">
      <span
        class="flex items-center justify-center size-6 bg-pond-color rounded-full text-base-color transition-transform duration-200 ease-in-out"
        :class="isDark ? 'translate-x-[26px]' : 'translate-x-0'"
      >
        <Moon v-if="isDark" :size="12" />
        <Sun v-else :size="12" />
      </span>
    </span>
  </button>
</template>
