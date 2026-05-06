import { invoke } from '@tauri-apps/api/core';
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

/**
 * @function キーボードリスナーのエラーイベントを購読する
 *
 * @param callback エラーメッセージを受け取るコールバック
 * @returns unlisten 関数（コンポーネントアンマウント時に呼び出すこと）
 *
 * NOTE:
 *   - rdev のグローバルキーボード監視が開始できなかった場合に emit される
 *   - 一度 emit されたあとはキーカウントが更新されなくなる
 */
export const subscribeListenerError = (callback: (message: string) => void): Promise<() => void> =>
  listen<string>('listener_error', (event) => callback(event.payload));

/**
 * @function 指定日の時間帯別キーストローク数を取得する
 *
 * @param date 対象日付（`YYYY-MM-DD` 形式）
 * @returns 長さ 24 の配列。インデックス = 時（0〜23）、値 = その時間帯のキーストローク数
 */
export const fetchHourlyCounts = (date: string): Promise<number[]> =>
  invoke<number[]>('get_hourly_counts', { date });
