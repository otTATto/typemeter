<script setup lang="ts">
import type { LegacyDataStatus } from '@/lib/dataMigration';
import AppModal from '@/components/AppModal.vue';

const props = defineProps<{
  status: LegacyDataStatus;
  isDeleting: boolean;
  error: string | null;
}>();

const emit = defineEmits<{
  close: [];
  delete: [];
}>();
</script>

<template>
  <AppModal
    :is-open="true"
    :primary-button="
      props.status.canDelete
        ? {
            label: props.isDeleting ? 'Deleting…' : 'Delete Old Data',
            onClick: () => emit('delete'),
            disabled: props.isDeleting,
            type: 'alert',
          }
        : undefined
    "
    :close-on-backdrop="false"
    close-label="Keep Old Data"
    @close="emit('close')"
  >
    <h3 class="text-2xl font-bold">Previous Data Still Exists</h3>
    <div class="my-2 space-y-5">
      <p class="text-left">
        typemeter found the <span class="text-accent-color font-bold">previous</span> data folder.
        The old files are <span class="text-danger-color font-bold">no longer</span> used by this
        version.
      </p>

      <div class="flex flex-col gap-3 text-left text-sm">
        <div>
          <p class="m-0 font-bold text-sub-color ml-3">Previous Data Location</p>
          <p class="m-0 break-all rounded-xl bg-background-color p-3 font-mono text-xs">
            {{ props.status.previousLocation }}
          </p>
        </div>
        <div>
          <p class="m-0 font-bold text-sub-color ml-3">New Data Location</p>
          <p class="m-0 break-all rounded-xl bg-background-color p-3 font-mono text-xs">
            {{ props.status.newLocation }}
          </p>
        </div>
      </div>

      <p v-if="props.status.canDelete" class="text-left text-xs">
        All {{ props.status.oldRecordCount }} previous keystroke records are present in the new
        database, and all previous settings keys are present in the new settings file.
      </p>
      <p v-else class="m-0 text-xs text-danger-color">
        {{
          props.status.issue ?? 'The new data could not be verified. The old files will be kept.'
        }}
      </p>

      <p v-if="props.error" class="m-0 text-xs text-danger-color">{{ props.error }}</p>
    </div>
  </AppModal>
</template>
