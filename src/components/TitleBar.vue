<!-- Windows/Linux 向けカスタムタイトルバー -->
<!-- decorations: false のウィンドウで使用する -->
<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Maximize2, Minimize2, Minus, X } from 'lucide-vue-next';
import { onMounted, onUnmounted, ref } from 'vue';

/**
 * - showMenu     : 左側の「typemeter」メニューを表示するか（デフォルト: true）
 * - showMaximize : 最大化/最小化ボタンを表示するか（デフォルト: true）
 * - title        : タイトルバー中央に表示するウィンドウ名（省略時は非表示）
 */
const props = withDefaults(
  defineProps<{ showMenu?: boolean; showMaximize?: boolean; title?: string }>(),
  { showMenu: true, showMaximize: true },
);

const appWindow = getCurrentWindow();
const isMenuOpen = ref(false);
const isMaximized = ref(false);

let unlistenResize: Promise<() => void> | null = null;

onMounted(() => {
  if (props.showMaximize) {
    unlistenResize = (async () => {
      isMaximized.value = await appWindow.isMaximized();
      return appWindow.onResized(async () => {
        isMaximized.value = await appWindow.isMaximized();
      });
    })();
  }
  document.addEventListener('click', closeMenu);
});

onUnmounted(async () => {
  (await unlistenResize)?.();
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
    class="relative flex items-center justify-between w-full h-11 bg-background-color select-none shrink-0"
  >
    <!-- 左：Typemeter メニュー -->
    <div class="relative h-full flex items-center">
      <template v-if="showMenu">
        <button
          aria-haspopup="true"
          :aria-expanded="isMenuOpen"
          :class="[
            'h-full px-4 text-sm text-base-color hover:bg-sub-color/20 transition-colors cursor-pointer',
            isMenuOpen ? 'bg-sub-color/20' : '',
          ]"
          @click.stop="toggleMenu"
        >
          typemeter
        </button>
        <Transition name="dropdown-fade">
          <div
            v-if="isMenuOpen"
            class="absolute top-full left-0 bg-pond-color border border-sub-color/20 shadow-lg rounded-2xl p-1 min-w-40 z-50"
            @click.stop
          >
            <button
              class="w-full text-left px-4 py-2 text-sm text-base-color hover:bg-sub-color/20 rounded-2xl transition-colors cursor-pointer"
              @click="openAbout"
            >
              About typemeter
            </button>
          </div>
        </Transition>
      </template>
    </div>

    <!-- 中央：ウィンドウタイトル -->
    <span
      v-if="title"
      class="absolute left-1/2 -translate-x-1/2 text-sm text-base-color pointer-events-none"
    >
      {{ title }}
    </span>

    <!-- 右：ウィンドウ操作ボタン -->
    <div class="flex h-full">
      <button
        aria-label="Minimize"
        class="flex items-center justify-center w-11 h-full text-base-color hover:bg-sub-color/20 transition-colors cursor-pointer"
        @click="minimize"
      >
        <Minus :size="12" />
      </button>
      <button
        v-if="showMaximize"
        aria-label="Toggle maximize"
        class="flex items-center justify-center w-11 h-full text-base-color hover:bg-sub-color/20 transition-colors cursor-pointer"
        @click="toggleMaximize"
      >
        <Maximize2 v-if="!isMaximized" :size="12" />
        <Minimize2 v-else :size="12" />
      </button>
      <button
        aria-label="Close"
        class="flex items-center justify-center w-11 h-full text-base-color hover:bg-danger-color hover:text-tm-white transition-colors cursor-pointer"
        @click="close"
      >
        <X :size="12" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.dropdown-fade-enter-active,
.dropdown-fade-leave-active {
  transition: opacity 0.1s ease;
}

.dropdown-fade-enter-from,
.dropdown-fade-leave-to {
  opacity: 0;
}
</style>
