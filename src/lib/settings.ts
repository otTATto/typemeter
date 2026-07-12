import { load, type Store } from '@tauri-apps/plugin-store';
import type { UnlistenFn } from '@tauri-apps/api/event';

export type Theme = 'light' | 'dark';

/**
 * 設定ストアのスキーマ
 *
 * - theme       : ライト / ダークモード（キー不在 = OS 設定に追従）
 * - alwaysOnTop : メインウィンドウを常に最前面に表示するか（キー不在 = false）
 */
type SettingsSchema = {
  theme: Theme;
  alwaysOnTop: boolean;
};

const SETTINGS_FILE = 'settings.json';
const DEFAULT_ALWAYS_ON_TOP = false;

let storePromise: Promise<Store> | null = null;

/**
 * @function 設定ストア（settings.json）をロードする
 *
 * NOTE: モジュール内でシングルトンとして管理し、複数回呼ばれても同じ Store を返す
 */
const getSettingsStore = (): Promise<Store> => {
  if (!storePromise) {
    storePromise = load(SETTINGS_FILE);
  }
  return storePromise;
};

/**
 * @function 設定ストアから値を取得する
 *
 * @param key 取得するキー
 * @returns 保存されている値。キー不在の場合は undefined
 */
const getValue = async <K extends keyof SettingsSchema>(
  key: K,
): Promise<SettingsSchema[K] | undefined> => {
  const store = await getSettingsStore();
  return store.get<SettingsSchema[K]>(key);
};

/**
 * @function 設定ストアへ値を保存する
 *
 * @param key 保存するキー
 * @param value 保存する値
 */
const setValue = async <K extends keyof SettingsSchema>(
  key: K,
  value: SettingsSchema[K],
): Promise<void> => {
  const store = await getSettingsStore();
  await store.set(key, value);
  await store.save();
};

/**
 * @function 設定ストアのキー変更を購読する
 *
 * @param key 購読するキー
 * @param cb 変更時に呼び出されるコールバック（キー不在の場合は undefined が渡される）
 * @returns クリーンアップ関数（onUnmounted 等で呼び出す）
 */
const subscribeValue = async <K extends keyof SettingsSchema>(
  key: K,
  cb: (value: SettingsSchema[K] | undefined) => void,
): Promise<UnlistenFn> => {
  const store = await getSettingsStore();
  return store.onKeyChange<SettingsSchema[K]>(key, cb);
};

/**
 * @function 保存されているテーマ設定を取得する
 * @returns テーマ。未設定の場合は null（OS 設定に追従）
 */
export const getTheme = async (): Promise<Theme | null> => (await getValue('theme')) ?? null;

/**
 * @function テーマ設定を保存する
 * @param theme 保存するテーマ
 */
export const setTheme = (theme: Theme): Promise<void> => setValue('theme', theme);

/**
 * @function テーマ設定の変更を購読する
 *
 * @param cb 変更時に呼び出されるコールバック（未設定になった場合は null）
 * @returns クリーンアップ関数（onUnmounted 等で呼び出す）
 */
export const onThemeChange = (cb: (theme: Theme | null) => void): Promise<UnlistenFn> =>
  subscribeValue('theme', (value) => cb(value ?? null));

/**
 * @function 常に最前面に表示する設定を取得する
 * @returns 未設定の場合は false
 */
export const getAlwaysOnTop = async (): Promise<boolean> =>
  (await getValue('alwaysOnTop')) ?? DEFAULT_ALWAYS_ON_TOP;

/**
 * @function 常に最前面に表示する設定を保存する
 * @param alwaysOnTop 保存する値
 */
export const setAlwaysOnTop = (alwaysOnTop: boolean): Promise<void> =>
  setValue('alwaysOnTop', alwaysOnTop);

/**
 * @function 常に最前面に表示する設定の変更を購読する
 *
 * @param cb 変更時に呼び出されるコールバック
 * @returns クリーンアップ関数（onUnmounted 等で呼び出す）
 */
export const onAlwaysOnTopChange = (cb: (alwaysOnTop: boolean) => void): Promise<UnlistenFn> =>
  subscribeValue('alwaysOnTop', (value) => cb(value ?? DEFAULT_ALWAYS_ON_TOP));
