<script setup lang="ts">
import AppModal from '@/components/AppModal.vue';

const props = defineProps<{
  version: string;
  currentVersion: string;
  releaseUrl: string;
}>();

const emit = defineEmits<{
  dismiss: [];
}>();
</script>

<template>
  <AppModal
    :is-open="true"
    :primary-button="{ label: 'Open Install Page', href: props.releaseUrl }"
    :close-on-backdrop="false"
    close-label="Later"
    @close="emit('dismiss')"
  >
    <h3 class="text-2xl font-bold">Update Available</h3>
    <p class="m-0">
      New version
      <span class="font-bold text-accent-color">v{{ props.version }}</span>
      is available! 🎉
    </p>
    <p class="text-sub-color text-sm">
      current version:
      <span class="text-accent-color">v{{ props.currentVersion }}</span>
    </p>
  </AppModal>
</template>
