import { invoke } from '@tauri-apps/api/core';

export type LegacyDataStatus = {
  previousLocation: string;
  newLocation: string;
  oldRecordCount: number | null;
  matchingRecordCount: number | null;
  settingsKeysRetained: boolean;
  canDelete: boolean;
  issue: string | null;
};

export const getLegacyDataStatus = (): Promise<LegacyDataStatus | null> =>
  invoke<LegacyDataStatus | null>('get_legacy_data_status');

export const deleteLegacyData = (): Promise<void> => invoke('delete_legacy_data');
