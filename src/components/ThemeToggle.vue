<!-- ライト / ダークモード切り替えトグルスイッチ -->
<!-- Settings ウィンドウの外観設定行に配置される -->
<script setup lang="ts">
import { Moon, Sun } from 'lucide-vue-next';
import { onMounted, onUnmounted, ref } from 'vue';
import ToggleSwitch from '@/components/ToggleSwitch.vue';
import { getTheme, onThemeChange, setTheme } from '@/lib/settings';
import { resolveIsDark } from '@/lib/theme';

const isDark = ref(false);

const toggle = () => {
  const next = !isDark.value;
  isDark.value = next;
  setTheme(next ? 'dark' : 'light').catch((err) => {
    console.error('[ThemeToggle] failed to save theme:', err);
    isDark.value = !next;
  });
};

let isDisposed = false;
let unlistenThemeChange: (() => void) | null = null;

onMounted(async () => {
  const theme = await getTheme();
  if (isDisposed) return;
  isDark.value = resolveIsDark(theme);

  const unlisten = await onThemeChange((theme) => {
    isDark.value = resolveIsDark(theme);
  });
  // 購読完了前に cleanup が呼ばれていた場合は即座に unlisten する
  if (isDisposed) {
    unlisten();
  } else {
    unlistenThemeChange = unlisten;
  }
});

onUnmounted(() => {
  isDisposed = true;
  unlistenThemeChange?.();
});
</script>

<template>
  <ToggleSwitch
    :is-on="isDark"
    :aria-label="isDark ? 'ライトモードに切り替え' : 'ダークモードに切り替え'"
    @toggle="toggle"
  >
    <Moon v-if="isDark" :size="12" />
    <Sun v-else :size="12" />
  </ToggleSwitch>
</template>
