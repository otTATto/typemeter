<!-- メインウィンドウを常に最前面に表示するかどうかを切り替えるトグルスイッチ -->
<!-- メインウィンドウのヘッダー右側と Settings ウィンドウの Always on Top 行の両方に配置される -->
<!-- どのウィンドウに置かれても操作対象はメインウィンドウであり、ストアの変更購読で互いに同期する -->
<script setup lang="ts">
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { Pin, PinOff } from 'lucide-vue-next';
import { onMounted, onUnmounted, ref } from 'vue';
import ToggleSwitch from '@/components/ToggleSwitch.vue';
import { getAlwaysOnTop, onAlwaysOnTopChange, setAlwaysOnTop } from '@/lib/settings';

const isPinned = ref(false);

/**
 * @function メインウィンドウの最前面表示状態を適用する
 *
 * @param isAlwaysOnTop 適用する最前面表示状態
 *
 * NOTE: Settings ウィンドウに置かれた場合も対象はメインウィンドウであるため、
 *       getCurrentWindow ではなくラベル指定で取得する
 */
const applyToMainWindow = async (isAlwaysOnTop: boolean): Promise<void> => {
  const mainWindow = await WebviewWindow.getByLabel('main');
  if (!mainWindow) {
    throw new Error('main window not found');
  }
  await mainWindow.setAlwaysOnTop(isAlwaysOnTop);
};

let isDisposed = false;
let unlistenAlwaysOnTopChange: (() => void) | null = null;

onMounted(async () => {
  const saved = await getAlwaysOnTop();
  if (isDisposed) return;
  isPinned.value = saved;

  // 別ウィンドウのトグルで変更された場合に表示を同期する
  const unlisten = await onAlwaysOnTopChange((isAlwaysOnTop) => {
    isPinned.value = isAlwaysOnTop;
  });
  // 購読完了前に cleanup が呼ばれていた場合は即座に unlisten する
  if (isDisposed) {
    unlisten();
  } else {
    unlistenAlwaysOnTopChange = unlisten;
  }
});

onUnmounted(() => {
  isDisposed = true;
  unlistenAlwaysOnTopChange?.();
});

const toggle = async () => {
  const prev = isPinned.value;
  const next = !prev;
  isPinned.value = next;
  try {
    await applyToMainWindow(next);
    try {
      await setAlwaysOnTop(next);
    } catch (err) {
      // ストア保存に失敗した場合はウィンドウの最前面状態も元に戻す
      await applyToMainWindow(prev);
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
