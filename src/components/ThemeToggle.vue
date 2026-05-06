<!-- ライト / ダークモード切り替えトグルスイッチ -->
<!-- 実際の切り替えは CSS の prefers-color-scheme media query に委ねており、このコンポーネントは見た目のみ -->
<!-- App.vue ヘッダー右側に配置される -->
<script setup lang="ts">
import { Sun } from 'lucide-vue-next';
</script>

<template>
  <button
    class="flex items-center justify-center w-13.5 h-11 border-0 bg-transparent cursor-pointer p-0"
    aria-label="テーマ切り替え"
  >
    <span class="flex items-center w-13.5 h-7 bg-sub-color rounded-[14px] p-0.5">
      <span
        class="flex items-center justify-center size-6 bg-pond-color rounded-full text-base-color"
      >
        <Sun :size="12" />
      </span>
    </span>
  </button>
</template>
