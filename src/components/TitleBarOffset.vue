<!-- カスタムタイトルバー表示有無に応じて直後の要素との上部余白を補正するスペーサー -->
<script setup lang="ts">
/**
 * - hasTitleBar : カスタムタイトルバーが表示されているか（デフォルト: true）
 *                 Windows/Linux でカスタムタイトルバーを使う場合は true（-mb-4）、
 *                 macOS でネイティブメニューバーを使う場合は false（-mb-2）
 */
withDefaults(defineProps<{ hasTitleBar?: boolean }>(), { hasTitleBar: true });
</script>

<template>
  <div :class="hasTitleBar ? '-mb-4' : '-mb-2'"></div>
</template>
