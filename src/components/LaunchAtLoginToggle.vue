<!-- OS ログイン時にアプリを自動起動するかどうかを切り替えるトグルスイッチ -->
<!-- Settings ウィンドウの Launch at Login 行に配置される -->
<!-- 状態は settings.json には保存せず、OS 側の自動起動エントリを真実の源とする -->
<script setup lang="ts">
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
import { Power, PowerOff } from 'lucide-vue-next';
import { onMounted, onUnmounted, ref } from 'vue';
import ToggleSwitch from '@/components/ToggleSwitch.vue';

const isAutostartEnabled = ref(false);

let isDisposed = false;

onMounted(async () => {
  try {
    const enabled = await isEnabled();
    // 取得完了前にアンマウントされていた場合は反映しない
    if (isDisposed) return;
    isAutostartEnabled.value = enabled;
  } catch (err) {
    console.error('[LaunchAtLoginToggle] failed to get autostart state:', err);
  }
});

onUnmounted(() => {
  isDisposed = true;
});

const toggle = async () => {
  const prev = isAutostartEnabled.value;
  const next = !prev;
  isAutostartEnabled.value = next;
  try {
    if (next) {
      await enable();
    } else {
      await disable();
    }
  } catch (err) {
    console.error('[LaunchAtLoginToggle] failed to set autostart:', err);
    isAutostartEnabled.value = prev;
  }
};
</script>

<template>
  <ToggleSwitch
    :is-on="isAutostartEnabled"
    :aria-label="
      isAutostartEnabled ? 'ログイン時に自動起動をオフにする' : 'ログイン時に自動起動をオンにする'
    "
    @toggle="toggle"
  >
    <Power v-if="isAutostartEnabled" :size="12" />
    <PowerOff v-else :size="12" />
  </ToggleSwitch>
</template>
