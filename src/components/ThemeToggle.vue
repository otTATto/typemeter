<!-- ライト / ダークモード切り替えトグルスイッチ -->
<!-- App.vue ヘッダー右側に配置される -->
<script setup lang="ts">
import { Moon, Sun } from 'lucide-vue-next';
import { onMounted, ref } from 'vue';

const isDark = ref(false);

const applyTheme = (dark: boolean) => {
  document.documentElement.classList.toggle('dark', dark);
  localStorage.setItem('theme', dark ? 'dark' : 'light');
};

const toggle = () => {
  isDark.value = !isDark.value;
  applyTheme(isDark.value);
};

onMounted(() => {
  const saved = localStorage.getItem('theme');
  isDark.value =
    saved === 'dark' || saved === 'light'
      ? saved === 'dark'
      : window.matchMedia('(prefers-color-scheme: dark)').matches;
  applyTheme(isDark.value);
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
