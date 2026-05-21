<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app';
import { openUrl } from '@tauri-apps/plugin-opener';
import { onMounted, onUnmounted, ref } from 'vue';
import TitleBar from '@/components/TitleBar.vue';
import TitleBarOffset from '@/components/TitleBarOffset.vue';

const version = ref('');

const applyTheme = () => {
  const saved = localStorage.getItem('theme');
  const isDark =
    saved === 'dark' || saved === 'light'
      ? saved === 'dark'
      : window.matchMedia('(prefers-color-scheme: dark)').matches;
  document.documentElement.classList.toggle('dark', isDark);
};

onMounted(async () => {
  applyTheme();
  // storage イベントは他ウィンドウの localStorage 変更時に発火する
  window.addEventListener('storage', applyTheme);

  try {
    version.value = await getVersion();
  } catch {
    // version は空文字のまま表示しない
  }
});

onUnmounted(() => {
  window.removeEventListener('storage', applyTheme);
});

const openRelease = (version: string) =>
  openUrl(`https://github.com/otTATto/typemeter/releases/tag/v${version}`);
const openX = () => openUrl('https://x.com/0123tato');
const openGitHub = () => openUrl('https://github.com/otTATto/typemeter');

// macOS はネイティブメニューバーを使うため、Windows/Linux のみカスタムタイトルバーを表示する
const SHOW_TITLE_BAR = !navigator.userAgent.includes('Macintosh');
</script>

<template>
  <div class="flex flex-col h-screen bg-background-color select-none">
    <TitleBar
      v-if="SHOW_TITLE_BAR"
      :show-menu="false"
      :show-maximize="false"
      title="About typemeter"
    />
    <TitleBarOffset :has-title-bar="SHOW_TITLE_BAR" />

    <div class="flex flex-col justify-center flex-1 px-8 py-6 overflow-y-auto">
      <!-- アプリ名とバージョン -->
      <div class="flex flex-col items-center gap-1.5 mb-6">
        <h1 class="text-4xl font-bold text-base-color tracking-tight">typemeter</h1>
        <button
          v-if="version"
          class="text-sm text-sub-color hover:underline cursor-pointer bg-transparent border-0 p-0"
          @click="openRelease(version)"
        >
          v{{ version }}
        </button>
      </div>

      <!-- コピーライトと GitHub リンク -->
      <div class="flex flex-col items-center gap-2 mb-6">
        <p class="text-sm text-base-color">
          © 2026
          <button
            class="text-sm text-base-color hover:underline cursor-pointer bg-transparent border-0 p-0"
            @click="openX"
          >
            tato
          </button>
        </p>
        <button
          class="text-sm text-accent-color hover:underline cursor-pointer bg-transparent border-0 p-0"
          @click="openGitHub"
        >
          github.com/otTATto/typemeter
        </button>
      </div>

      <hr class="border-sub-color/30 mb-5" />

      <!-- オープンソースライセンス -->
      <div>
        <h2 class="text-xs font-bold text-sub-color uppercase tracking-wider mb-3">
          Open Source Licenses
        </h2>
        <div class="space-y-1 text-xs text-sub-color">
          <p class="font-medium text-base-color">Manjari (Latin Subset)</p>
          <p>Copyright 2018 The Manjari Project Authors</p>
          <p>Licensed under the SIL Open Font License, Version 1.1.</p>
        </div>
      </div>
    </div>
  </div>
</template>
