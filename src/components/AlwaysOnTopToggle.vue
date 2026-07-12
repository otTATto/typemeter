<!-- 常に最前面に表示するかどうかを切り替えるトグルスイッチ -->
<!-- App.vue ヘッダー右側に配置される -->
<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Pin, PinOff } from 'lucide-vue-next';
import { onMounted, ref } from 'vue';
import ToggleSwitch from '@/components/ToggleSwitch.vue';
import { getAlwaysOnTop, setAlwaysOnTop } from '@/lib/settings';

const isPinned = ref(false);

onMounted(async () => {
  isPinned.value = await getAlwaysOnTop();
});

const toggle = async () => {
  const prev = isPinned.value;
  const next = !prev;
  isPinned.value = next;
  try {
    await getCurrentWindow().setAlwaysOnTop(next);
    try {
      await setAlwaysOnTop(next);
    } catch (err) {
      // ストア保存に失敗した場合はウィンドウの最前面状態も元に戻す
      await getCurrentWindow().setAlwaysOnTop(prev);
      throw err;
    }
  } catch (err) {
    console.error('[AlwaysOnTopToggle] failed to set always-on-top:', err);
    isPinned.value = prev;
  }
};
</script>

<template>
  <ToggleSwitch
    :is-on="isPinned"
    :aria-label="isPinned ? '常に最前面に表示をオフにする' : '常に最前面に表示をオンにする'"
    @toggle="toggle"
  >
    <Pin v-if="isPinned" :size="12" />
    <PinOff v-else :size="12" />
  </ToggleSwitch>
</template>
