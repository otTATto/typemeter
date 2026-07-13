<!-- トグルスイッチの見た目とアクセシビリティ属性だけを持つ汎用コンポーネント -->
<!-- オン / オフの状態管理や保存処理は親コンポーネントが担い、クリックは toggle イベントで通知する -->
<!-- ノブ内のアイコンはスロットで受け取る（親が isOn に応じたアイコンを描画する） -->
<!-- aria-label は親が fallthrough 属性として渡す（単一ルートの button に自動継承される） -->
<script setup lang="ts">
defineProps<{
  /** トグルがオン状態かどうか（ノブの位置と aria-pressed に反映される） */
  isOn: boolean;
  /** 操作を無効化するかどうか（非同期処理中の連打防止などに使う） */
  disabled?: boolean;
}>();

defineEmits<{
  /** トグルがクリックされたときに発火する（状態の切り替えは親が行う） */
  toggle: [];
}>();
</script>

<template>
  <button
    class="flex items-center justify-center w-13.5 h-11 border-0 bg-transparent cursor-pointer p-0 disabled:opacity-50 disabled:cursor-not-allowed"
    :aria-pressed="isOn"
    :disabled="disabled"
    @click="$emit('toggle')"
  >
    <span class="flex items-center w-13.5 h-7 bg-sub-color rounded-[14px] p-0.5">
      <span
        class="flex items-center justify-center size-6 bg-pond-color rounded-full text-base-color transition-transform duration-200 ease-in-out"
        :class="isOn ? 'translate-x-[26px]' : 'translate-x-0'"
      >
        <slot />
      </span>
    </span>
  </button>
</template>
