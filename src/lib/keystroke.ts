import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

/**
 * @function 今日の保存済みキーストローク合計を取得する
 *
 * @returns 今日の SQLite に保存されたキーストローク数の合計
 *
 * NOTE:
 *   - 現在のセッション分（未保存の直近 1 分間）は含まない
 *   - DB 取得失敗時は Rust 側が 0 を返す
 */
export const fetchTodayCount = (): Promise<number> => invoke<number>('get_today_count');

/**
 * @function キーストローク更新イベントを購読する
 *
 * @param callback セッション開始からの累計キー数を受け取るコールバック
 * @returns unlisten 関数（コンポーネントアンマウント時に呼び出すこと）
 *
 * NOTE:
 *   - イベントは Rust 側から 1 秒ごとに emit される
 *   - payload はセッション開始時を 0 とした累計カウント
 */
export const subscribeKeystrokeUpdate = (
  callback: (sessionCount: number) => void,
): Promise<() => void> => listen<number>('keystroke_update', (event) => callback(event.payload));
