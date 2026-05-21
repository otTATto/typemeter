<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app';
import { openUrl } from '@tauri-apps/plugin-opener';
import { onMounted, ref } from 'vue';

const version = ref('');

onMounted(async () => {
  const saved = localStorage.getItem('theme');
  const isDark =
    saved === 'dark' || saved === 'light'
      ? saved === 'dark'
      : window.matchMedia('(prefers-color-scheme: dark)').matches;
  document.documentElement.classList.toggle('dark', isDark);

  try {
    version.value = await getVersion();
  } catch {
    // version は空文字のまま表示しない
  }
});

const openGitHub = () => openUrl('https://github.com/otTATto/typemeter');
</script>

<template>
  <div class="flex flex-col h-screen bg-background-color px-8 pt-8 pb-6 select-none">
    <!-- アプリ名とバージョン -->
    <div class="flex flex-col items-center gap-1.5 mb-6">
      <h1 class="text-4xl font-bold text-base-color tracking-tight">typemeter</h1>
      <p v-if="version" class="text-sm text-sub-color">Version {{ version }}</p>
    </div>

    <!-- コピーライトと GitHub リンク -->
    <div class="flex flex-col items-center gap-2 mb-6">
      <p class="text-sm text-base-color">© 2026 otTATto</p>
      <button
        class="text-sm text-accent-color hover:underline cursor-pointer bg-transparent border-0 p-0"
        @click="openGitHub"
      >
        github.com/otTATto/typemeter
      </button>
    </div>

    <hr class="border-sub-color/30 mb-5" />

    <!-- オープンソースライセンス -->
    <div class="flex-1 overflow-y-auto">
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
</template>
