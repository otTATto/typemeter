use std::sync::{Arc, Mutex};

use chrono::Local;
use rusqlite::{params, Connection};

pub struct DbPath(pub String);

/// SQLite データベースを初期化する
///
/// `db_path` が存在しない場合はファイルを作成し、`keystroke_logs` テーブルを作成する。
/// 既にテーブルが存在する場合は何もしない。
///
/// # Panics
/// DB のオープンまたはテーブル作成に失敗した場合パニックする（起動時の必須処理のため）。
pub fn init_db(db_path: &str) {
    let conn = Connection::open(db_path).expect("failed to open database");
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS keystroke_logs (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            recorded_at  TEXT    NOT NULL,
            minute_count INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_keystroke_logs_recorded_at
            ON keystroke_logs (recorded_at);",
    )
    .expect("failed to initialize database schema");
}

/// `minute_count` の現在値を SQLite に保存し、保存成功時に `today_db_count` を加算する
///
/// # Parameters
/// * `minute_count` - 直近の未保存キーストローク数
/// * `today_db_count` - 今日の DB 保存済み合計（書き込み成功時にインクリメントされる）
/// * `db_path` - データベースファイルのパス
///
/// # Behavior
/// * `minute_count` が 0 の場合は書き込みをスキップする
/// * ロック取得と同時にカウントを 0 にして「書き込み権」を確保し、IO 中の二重保存を防ぐ
/// * DB 書き込み失敗時はカウントを加算して戻す
/// * 1 分タイマーとアプリ終了時の両方から呼び出される
pub fn flush_minute_count(
    minute_count: &Arc<Mutex<u64>>,
    today_db_count: &Arc<Mutex<u64>>,
    db_path: &str,
) {
    let count = {
        let mut lock = minute_count.lock().unwrap();
        let count = *lock;
        if count == 0 {
            return;
        }
        *lock = 0;
        count
    };

    if let Ok(conn) = Connection::open(db_path) {
        let recorded_at = Local::now().to_rfc3339();
        let result = conn.execute(
            "INSERT INTO keystroke_logs (recorded_at, minute_count) VALUES (?1, ?2)",
            params![recorded_at, count as i64],
        );
        if result.is_ok() {
            *today_db_count.lock().unwrap() += count;
        } else {
            *minute_count.lock().unwrap() += count;
        }
    } else {
        *minute_count.lock().unwrap() += count;
    }
}

/// ある日付の1時間ごとのキーストローク数を SQLite から取得する
///
/// # Parameters
/// * `db_path` - データベースファイルのパス
/// * `date` - 対象日付（`YYYY-MM-DD` 形式）
///
/// # Returns
/// インデックス = 時（0〜23）、値 = その時間帯の合計キーストローク数の配列。
/// DB 接続・クエリ失敗時はすべて `0`。
///
/// # Behavior
/// `recorded_at` は `chrono::Local::now().to_rfc3339()` 形式（`YYYY-MM-DDTHH:MM:SS+TZ`）で保存される。
/// `strftime('%H', ...)` は UTC 変換してしまうため、位置 12〜13 文字目（ローカル時）を `substr` で直接抽出する。
pub fn query_hourly_counts(db_path: &str, date: &str) -> [u64; 24] {
    let mut counts = [0u64; 24];
    let Ok(conn) = Connection::open(db_path) else {
        return counts;
    };
    let Ok(mut stmt) = conn.prepare(
        "SELECT CAST(substr(recorded_at, 12, 2) AS INTEGER), SUM(minute_count)
         FROM keystroke_logs
         WHERE recorded_at LIKE ?1
         GROUP BY substr(recorded_at, 12, 2)",
    ) else {
        return counts;
    };
    let rows = stmt.query_map(params![format!("{}%", date)], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
    });
    if let Ok(rows) = rows {
        for (hour, total) in rows.flatten() {
            if (0..24).contains(&hour) {
                counts[hour as usize] = total.max(0) as u64;
            }
        }
    }
    counts
}

/// 今日の日付で保存された `keystroke_logs` の合計を SQLite から取得する
///
/// # Returns
/// 今日保存済みのキーストローク数の合計。取得失敗時は `0`。
pub fn query_today_db_count(db_path: &str) -> u64 {
    let Ok(conn) = Connection::open(db_path) else {
        return 0;
    };
    let today = Local::now().format("%Y-%m-%d").to_string();
    conn.query_row(
        "SELECT COALESCE(SUM(minute_count), 0) FROM keystroke_logs WHERE recorded_at LIKE ?1",
        params![format!("{}%", today)],
        |row| row.get::<_, i64>(0),
    )
    .unwrap_or(0)
    .max(0) as u64
}
