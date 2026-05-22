<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app';
import { openUrl } from '@tauri-apps/plugin-opener';
import { platform } from '@tauri-apps/plugin-os';
import { BadgeAlert, CircleCheck } from 'lucide-vue-next';
import { onMounted, onUnmounted, ref } from 'vue';
import SpeechBubble from '@/components/SpeechBubble.vue';
import TitleBar from '@/components/TitleBar.vue';
import TitleBarOffset from '@/components/TitleBarOffset.vue';
import { initTheme } from '@/lib/theme';
import { checkForUpdate, type UpdateInfo } from '@/lib/update';

const version = ref('');
const updateInfo = ref<UpdateInfo | null>(null);
const updateCheckDone = ref(false);

// macOS はネイティブメニューバーを使うため、Windows/Linux のみカスタムタイトルバーを表示する
// UA による初期値で初回レンダリングのちらつきを防ぎ、plugin-os で確定する
const showTitleBar = ref(!navigator.userAgent.includes('Macintosh'));

let cleanupTheme: (() => void) | null = null;

onMounted(async () => {
  cleanupTheme = initTheme();
  showTitleBar.value = (await platform()) !== 'macos';

  try {
    version.value = await getVersion();
  } catch {
    // version は空文字のまま表示しない
  }

  updateInfo.value = await checkForUpdate();
  updateCheckDone.value = true;
});

onUnmounted(() => {
  cleanupTheme?.();
});

const openRelease = (version: string) =>
  openUrl(`https://github.com/otTATto/typemeter/releases/tag/v${version}`);
const openX = () => openUrl('https://x.com/0123tato');
const openGitHub = () => openUrl('https://github.com/otTATto/typemeter');
</script>

<template>
  <div class="flex flex-col h-screen bg-background-color select-none">
    <TitleBar
      v-if="showTitleBar"
      :show-menu="false"
      :show-maximize="false"
      title="About typemeter"
    />
    <TitleBarOffset :has-title-bar="showTitleBar" />

    <div class="flex flex-col justify-center flex-1 px-8 py-6 overflow-y-auto">
      <!-- アプリ名とバージョン -->
      <div class="flex flex-col items-center gap-4 mb-6">
        <h1 class="text-4xl font-bold text-base-color tracking-tight">typemeter</h1>
        <div class="flex flex-col items-center gap-2">
          <!-- バージョン番号（吹き出しの基準要素、button の自然幅に合わせる） -->
          <div class="relative w-fit">
            <!-- 吹き出しの位置決めラッパー（absolute はここに付け、SpeechBubble 自身は relative のまま保つ） -->
            <div
              v-if="updateCheckDone"
              :class="[
                'absolute bottom-full whitespace-nowrap',
                updateInfo ? 'mb-1 ml-5' : 'ml-15',
                { 'cursor-pointer': updateInfo },
              ]"
              @click="updateInfo && openUrl(updateInfo.releaseUrl)"
            >
              <SpeechBubble
                :variant="updateInfo ? 'alert' : 'normal'"
                tail="left"
                :class="[
                  'shadow-lg rotate-4 duration-300 ease-in-out',
                  updateInfo ? 'hover:-translate-y-0.5' : '',
                ]"
              >
                <template v-if="updateInfo">
                  <BadgeAlert :size="12" />
                  <span class="translate-y-0.75">Update Available! Click to Install</span>
                </template>
                <template v-else>
                  <CircleCheck :size="12" />
                  <span class="translate-y-0.75">Latest</span>
                </template>
              </SpeechBubble>
            </div>

            <!-- 現在バージョン（クリックでリリースページを開く） -->
            <button
              v-if="version"
              class="text-sm text-sub-color hover:underline cursor-pointer bg-transparent border-0 p-0"
              @click="openRelease(version)"
            >
              v{{ version }}
            </button>
          </div>
        </div>
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
