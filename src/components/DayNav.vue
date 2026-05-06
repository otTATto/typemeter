<!-- 表示日を切り替える ‹ TODAY › ナビゲーション -->
<!-- App.vue ヘッダー中央に配置される -->
<script setup lang="ts">
import { ChevronLeft, ChevronRight } from 'lucide-vue-next';
import { computed } from 'vue';
import NavButton from '@/components/NavButton.vue';

const props = defineProps<{
  modelValue: string; // ターゲット日付（YYYY-MM-DD）
  todayDate: string; // 今日の日付（YYYY-MM-DD）
}>();

const emit = defineEmits<{
  'update:modelValue': [date: string];
}>();

const isToday = computed(() => props.modelValue === props.todayDate);

const addDays = (date: string, days: number): string => {
  const d = new Date(`${date}T00:00:00`);
  d.setDate(d.getDate() + days);
  return d.toLocaleDateString('en-CA');
};

const goPrev = () => emit('update:modelValue', addDays(props.modelValue, -1));
const goToday = () => {
  if (!isToday.value) emit('update:modelValue', props.todayDate);
};
const goNext = () => {
  if (!isToday.value) emit('update:modelValue', addDays(props.modelValue, 1));
};
</script>

<template>
  <div class="flex items-center gap-1">
    <NavButton aria-label="前の日" @click="goPrev">
      <ChevronLeft :size="20" />
    </NavButton>
    <NavButton :locked="isToday" @click="goToday"
      ><span class="translate-y-0.75">TODAY</span></NavButton
    >
    <NavButton :locked="isToday" aria-label="次の日" @click="goNext">
      <ChevronRight :size="20" />
    </NavButton>
  </div>
</template>
