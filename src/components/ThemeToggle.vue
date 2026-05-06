<!-- ライト / ダークモード切り替えトグルスイッチ -->
<!-- 実際の切り替えは CSS の prefers-color-scheme media query に委ねており、このコンポーネントは見た目のみ -->
<!-- App.vue ヘッダー右側に配置される -->
<script setup lang="ts">
import { Sun } from 'lucide-vue-next';
</script>

<template>
  <button class="theme-toggle" aria-label="テーマ切り替え">
    <span class="toggle-track">
      <span class="toggle-thumb">
        <Sun :size="12" />
      </span>
    </span>
  </button>
</template>

<style scoped>
.theme-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 54px;
  height: 44px;
  border: none;
  background: transparent;
  cursor: pointer;
  padding: 0;
}

.toggle-track {
  display: flex;
  align-items: center;
  width: 54px;
  height: 28px;
  background-color: var(--sub-color);
  border-radius: 14px;
  padding: 2px;
}

.toggle-thumb {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background-color: var(--pond-color);
  border-radius: 50%;
  color: var(--base-color);
}
</style>
