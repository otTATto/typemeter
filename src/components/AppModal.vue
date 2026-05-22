<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener';
import { onUnmounted, ref, watch } from 'vue';

/**
 * プライマリボタンの設定
 *
 * - label   : ボタンのラベル
 * - onClick : クリック時のコールバック（onClick と href は排他）
 * - href    : 外部リンク URL（onClick と href は排他）
 * - disabled: 非活性にするかどうか
 * - type    : 'normal'（アクセントカラー）| 'alert'（デンジャーカラー）
 */
type ButtonProps = {
  label?: string;
  onClick?: () => void;
  href?: string;
  disabled?: boolean;
  type?: 'normal' | 'alert';
};

const props = withDefaults(
  defineProps<{
    isOpen: boolean;
    primaryButton?: ButtonProps;
    closeLabel?: string;
    closeOnBackdrop?: boolean;
  }>(),
  { primaryButton: undefined, closeLabel: 'とじる', closeOnBackdrop: true },
);

const emit = defineEmits<{ close: [] }>();

const ANIMATION_DURATION = 300;

// DOM に出しておくかどうか（アンマウントはアニメーション完了後）
const isMounted = ref(props.isOpen);
// 開きアニメ / 閉じアニメの状態
const isOpened = ref(false);

let animTimer: ReturnType<typeof setTimeout> | null = null;

watch(
  () => props.isOpen,
  (val) => {
    if (animTimer !== null) {
      clearTimeout(animTimer);
      animTimer = null;
    }
    if (val) {
      isMounted.value = true;
      isOpened.value = false;
      // 次のフレームで isOpened=true にしてフェードイン開始
      animTimer = setTimeout(() => {
        isOpened.value = true;
        animTimer = null;
      }, 10);
    } else {
      isOpened.value = false;
      // アニメーション完了後にアンマウント
      animTimer = setTimeout(() => {
        isMounted.value = false;
        animTimer = null;
      }, ANIMATION_DURATION);
    }
  },
  { immediate: true },
);

onUnmounted(() => {
  if (animTimer !== null) clearTimeout(animTimer);
});

const handlePrimaryHrefClick = () => {
  if (props.primaryButton?.href) openUrl(props.primaryButton.href);
};
</script>

<template>
  <Teleport to="body">
    <div
      v-if="isMounted"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/20 backdrop-blur transition-opacity duration-300"
      :class="isOpened ? 'opacity-100' : 'opacity-0'"
      @click="props.closeOnBackdrop && emit('close')"
    >
      <div
        class="bg-pond-color p-[30px] w-[350px] rounded-xl transition-transform duration-300"
        :class="isOpened ? 'scale-100' : 'scale-[0.98]'"
        @click.stop
      >
        <div class="flex flex-col gap-y-[15px]">
          <!-- 本文 -->
          <div class="text-center">
            <slot />
          </div>

          <!-- Primary button: 外部リンク -->
          <button
            v-if="primaryButton?.href"
            type="button"
            :disabled="primaryButton.disabled"
            class="font-bold rounded-md h-[50px] flex items-center justify-center duration-300 ease-in-out disabled:opacity-40 disabled:cursor-not-allowed"
            :class="[
              primaryButton.type === 'alert'
                ? 'bg-danger-color text-white hover:opacity-80'
                : 'bg-accent-color text-tm-black hover:opacity-80',
              primaryButton.disabled ? '' : 'cursor-pointer',
            ]"
            @click="handlePrimaryHrefClick"
          >
            <span class="translate-y-0.75">{{ primaryButton.label ?? '' }}</span>
          </button>

          <!-- Primary button: アクション -->
          <button
            v-else-if="primaryButton?.onClick"
            type="button"
            :disabled="primaryButton.disabled"
            class="font-bold rounded-md h-[50px] flex items-center justify-center duration-300 ease-in-out disabled:opacity-40 disabled:cursor-not-allowed"
            :class="[
              primaryButton.type === 'alert'
                ? 'bg-danger-color text-white hover:opacity-80'
                : 'bg-accent-color text-tm-black hover:opacity-80',
              primaryButton.disabled ? '' : 'cursor-pointer',
            ]"
            @click="primaryButton.onClick()"
          >
            <span class="translate-y-0.75">{{ primaryButton.label ?? '' }}</span>
          </button>

          <!-- 閉じるボタン -->
          <button
            type="button"
            class="text-base-color font-bold bg-sub-color/20 hover:bg-sub-color/30 cursor-pointer duration-300 ease-in-out rounded-md h-[50px] flex items-center justify-center"
            @click="emit('close')"
          >
            <span class="translate-y-0.75">{{ props.closeLabel }}</span>
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
