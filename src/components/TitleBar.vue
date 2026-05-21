<!-- Windows/Linux 向けカスタムタイトルバー -->
<!-- decorations: false のウィンドウで使用する -->
<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Maximize2, Minimize2, Minus, X } from 'lucide-vue-next';
import { onMounted, onUnmounted, ref } from 'vue';

const appWindow = getCurrentWindow();
const isMenuOpen = ref(false);
const isMaximized = ref(false);

let unlistenResize: (() => void) | null = null;

onMounted(async () => {
  isMaximized.value = await appWindow.isMaximized();
  unlistenResize = await appWindow.onResized(async () => {
    isMaximized.value = await appWindow.isMaximized();
  });
  document.addEventListener('click', closeMenu);
});

onUnmounted(() => {
  unlistenResize?.();
  document.removeEventListener('click', closeMenu);
});

const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value;
};

const closeMenu = () => {
  isMenuOpen.value = false;
};

const openAbout = async () => {
  closeMenu();
  await invoke('open_about_window');
};

const minimize = () => appWindow.minimize();
const toggleMaximize = () => appWindow.toggleMaximize();
const close = () => appWindow.close();
</script>

<template>
  <div
    data-tauri-drag-region
    class="flex items-center justify-between w-full h-11 bg-pond-color select-none shrink-0"
  >
    <!-- 左：Typemeter メニュー -->
    <div class="relative h-full flex items-center">
      <button
        class="h-full px-4 text-sm text-base-color hover:bg-sub-color/20 transition-colors cursor-default"
        @click.stop="toggleMenu"
      >
        Typemeter
      </button>
      <Transition name="dropdown-fade">
        <div
          v-if="isMenuOpen"
          class="absolute top-full left-0 bg-pond-color border border-sub-color/20 shadow-lg rounded py-1 min-w-40 z-50"
          @click.stop
        >
          <button
            class="w-full text-left px-4 py-2 text-sm text-base-color hover:bg-sub-color/20 transition-colors cursor-default"
            @click="openAbout"
          >
            About Typemeter
          </button>
        </div>
      </Transition>
    </div>

    <!-- 右：ウィンドウ操作ボタン -->
    <div class="flex h-full">
      <button
        aria-label="最小化"
        class="flex items-center justify-center w-11 h-full text-base-color hover:bg-sub-color/20 transition-colors cursor-default"
        @click="minimize"
      >
        <Minus :size="12" />
      </button>
      <button
        aria-label="最大化を切り替え"
        class="flex items-center justify-center w-11 h-full text-base-color hover:bg-sub-color/20 transition-colors cursor-default"
        @click="toggleMaximize"
      >
        <Maximize2 v-if="!isMaximized" :size="12" />
        <Minimize2 v-else :size="12" />
      </button>
      <button
        aria-label="閉じる"
        class="close-btn flex items-center justify-center w-11 h-full text-base-color transition-colors cursor-default"
        @click="close"
      >
        <X :size="12" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.close-btn:hover {
  background-color: #c42b1c;
  color: white;
}

.dropdown-fade-enter-active,
.dropdown-fade-leave-active {
  transition: opacity 0.1s ease;
}

.dropdown-fade-enter-from,
.dropdown-fade-leave-to {
  opacity: 0;
}
</style>
