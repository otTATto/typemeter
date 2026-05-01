import { listen } from '@tauri-apps/api/event';

/**
 * @function キーストローク更新イベントを購読する
 *
 * @param callback 今日の合計キー数（DB 保存済み + 未保存分）を受け取るコールバック
 * @returns unlisten 関数（コンポーネントアンマウント時に呼び出すこと）
 *
 * NOTE:
 *   - イベントは Rust 側から 1 秒ごとに emit される
 *   - payload は今日の合計カウント（日付変更時に自動リセット）
 */
export const subscribeKeystrokeUpdate = (
  callback: (todayTotal: number) => void,
): Promise<() => void> => listen<number>('keystroke_update', (event) => callback(event.payload));
