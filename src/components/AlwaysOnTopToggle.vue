<!-- 常に最前面に表示するかどうかを切り替えるトグルスイッチ -->
<!-- App.vue ヘッダー右側に配置される -->
<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Pin, PinOff } from 'lucide-vue-next';
import { onMounted, ref } from 'vue';
import { getAlwaysOnTop, setAlwaysOnTop } from '@/lib/settings';

const isPinned = ref(false);

onMounted(async () => {
  isPinned.value = await getAlwaysOnTop();
});

const toggle = async () => {
  const next = !isPinned.value;
  isPinned.value = next;
  try {
    await getCurrentWindow().setAlwaysOnTop(next);
    await setAlwaysOnTop(next);
  } catch (err) {
    console.error('[AlwaysOnTopToggle] failed to set always-on-top:', err);
    isPinned.value = !next;
  }
};
</script>

<template>
  <button
    class="flex items-center justify-center w-13.5 h-11 border-0 bg-transparent cursor-pointer p-0"
    :aria-label="isPinned ? '常に最前面に表示をオフにする' : '常に最前面に表示をオンにする'"
    :aria-pressed="isPinned"
    @click="toggle"
  >
    <span class="flex items-center w-13.5 h-7 bg-sub-color rounded-[14px] p-0.5">
      <span
        class="flex items-center justify-center size-6 bg-pond-color rounded-full text-base-color transition-transform duration-200 ease-in-out"
        :class="isPinned ? 'translate-x-[26px]' : 'translate-x-0'"
      >
        <Pin v-if="isPinned" :size="12" />
        <PinOff v-else :size="12" />
      </span>
    </span>
  </button>
</template>
