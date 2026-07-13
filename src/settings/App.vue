<script setup lang="ts">
import { platform } from '@tauri-apps/plugin-os';
import { onMounted, onUnmounted, ref } from 'vue';
import AlwaysOnTopToggle from '@/components/AlwaysOnTopToggle.vue';
import LaunchAtLoginToggle from '@/components/LaunchAtLoginToggle.vue';
import ThemeToggle from '@/components/ThemeToggle.vue';
import TitleBar from '@/components/TitleBar.vue';
import TitleBarOffset from '@/components/TitleBarOffset.vue';
import { initTheme } from '@/lib/theme';

// macOS はネイティブメニューバーを使うため、Windows/Linux のみカスタムタイトルバーを表示する
// UA による初期値で初回レンダリングのちらつきを防ぎ、plugin-os で確定する
const showTitleBar = ref(!navigator.userAgent.includes('Macintosh'));

let cleanupTheme: (() => void) | null = null;

onMounted(async () => {
  cleanupTheme = initTheme();
  showTitleBar.value = (await platform()) !== 'macos';
});

onUnmounted(() => {
  cleanupTheme?.();
});
</script>

<template>
  <div class="flex flex-col h-screen bg-background-color">
    <TitleBar v-if="showTitleBar" :show-menu="false" :show-maximize="false" title="Settings" />
    <TitleBarOffset :has-title-bar="showTitleBar" />

    <main class="flex flex-col px-6 py-4">
      <div
        class="flex items-center justify-between h-11"
        role="group"
        aria-labelledby="theme-label"
      >
        <span id="theme-label" class="text-sm text-base-color">Light/Dark Theme</span>
        <ThemeToggle />
      </div>
      <div
        class="flex items-center justify-between h-11"
        role="group"
        aria-labelledby="always-on-top-label"
      >
        <span id="always-on-top-label" class="text-sm text-base-color">Always on Top</span>
        <AlwaysOnTopToggle />
      </div>
      <div
        class="flex items-center justify-between h-11"
        role="group"
        aria-labelledby="launch-at-login-label"
      >
        <span id="launch-at-login-label" class="text-sm text-base-color">Launch at PC Login</span>
        <LaunchAtLoginToggle />
      </div>
    </main>
  </div>
</template>
