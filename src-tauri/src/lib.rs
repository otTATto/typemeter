use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use chrono::Local;
use rusqlite::{params, Connection};
use tauri::{Emitter, Manager};

struct DbPath(String);

/// SQLite データベースを初期化する
///
/// `db_path` が存在しない場合はファイルを作成し、`keystroke_logs` テーブルを作成する。
/// 既にテーブルが存在する場合は何もしない。
///
/// # Panics
/// DB のオープンまたはテーブル作成に失敗した場合パニックする（起動時の必須処理のため）。
fn init_db(db_path: &str) {
    let conn = Connection::open(db_path).expect("failed to open database");
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS keystroke_logs (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            recorded_at  TEXT    NOT NULL,
            minute_count INTEGER NOT NULL
        )",
    )
    .expect("failed to initialize database schema");
}

/// `minute_count` の現在値を SQLite に保存し、保存成功時に減算・`today_db_count` を加算する
///
/// # Parameters
/// * `minute_count` - 直近の未保存キーストローク数
/// * `today_db_count` - 今日の DB 保存済み合計（書き込み成功時にインクリメントされる）
/// * `db_path` - データベースファイルのパス
///
/// # Behavior
/// * `minute_count` が 0 の場合は書き込みをスキップする
/// * DB 書き込み成功後に保存した分だけ減算する（失敗時はカウントを保持）
/// * 書き込み中に増加した分は `saturating_sub` により正しく保持される
/// * 1 分タイマーとアプリ終了時の両方から呼び出される
fn flush_minute_count(
    minute_count: &Arc<Mutex<u64>>,
    today_db_count: &Arc<Mutex<u64>>,
    db_path: &str,
) {
    let count = *minute_count.lock().unwrap();
    if count > 0 {
        if let Ok(conn) = Connection::open(db_path) {
            let recorded_at = Local::now().to_rfc3339();
            let result = conn.execute(
                "INSERT INTO keystroke_logs (recorded_at, minute_count) VALUES (?1, ?2)",
                params![recorded_at, count as i64],
            );
            if result.is_ok() {
                let mut lock = minute_count.lock().unwrap();
                *lock = lock.saturating_sub(count);
                *today_db_count.lock().unwrap() += count;
            }
        }
    }
}

/// 今日の日付で保存された `keystroke_logs` の合計を SQLite から取得する
///
/// # Returns
/// 今日保存済みのキーストローク数の合計。取得失敗時は `0`。
fn query_today_db_count(db_path: &str) -> u64 {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let minute_count = Arc::new(Mutex::new(0u64));
    let today_db_count = Arc::new(Mutex::new(0u64));

    let minute_count_s = minute_count.clone();
    let today_db_count_s = today_db_count.clone();

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let db_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&db_dir)?;
            let db_path = db_dir.join("keystroke.db").to_string_lossy().into_owned();

            init_db(&db_path);
            *today_db_count_s.lock().unwrap() = query_today_db_count(&db_path);
            app.manage(DbPath(db_path.clone()));

            // グローバルキーボードイベントのリスニング（キー押下時に即 emit）
            let mc_rdev = minute_count_s.clone();
            let tdc_rdev = today_db_count_s.clone();
            let app_handle_rdev_emit = app.handle().clone();
            let app_handle_rdev_err = app.handle().clone();
            thread::spawn(move || {
                if let Err(e) = rdev::listen(move |event| {
                    if matches!(event.event_type, rdev::EventType::KeyPress(_)) {
                        *mc_rdev.lock().unwrap() += 1;
                        let today_total =
                            *tdc_rdev.lock().unwrap() + *mc_rdev.lock().unwrap();
                        let _ = app_handle_rdev_emit.emit("keystroke_update", today_total);
                    }
                }) {
                    let _ = app_handle_rdev_err.emit("listener_error", format!("{e:?}"));
                }
            });

            // ハートビート：初期表示・日付変更検知のため 1 秒ごとに emit
            let mc_emit = minute_count_s.clone();
            let tdc_emit = today_db_count_s.clone();
            let app_handle = app.handle().clone();
            thread::spawn(move || {
                let mut last_date = Local::now().format("%Y-%m-%d").to_string();
                loop {
                    thread::sleep(Duration::from_secs(1));
                    let current_date = Local::now().format("%Y-%m-%d").to_string();
                    if current_date != last_date {
                        // 日付が変わったら今日の DB 合計をリセット
                        *tdc_emit.lock().unwrap() = 0;
                        last_date = current_date;
                    }
                    let today_total = *tdc_emit.lock().unwrap() + *mc_emit.lock().unwrap();
                    let _ = app_handle.emit("keystroke_update", today_total);
                }
            });

            // 1 分ごとに SQLite へ保存
            let mc_save = minute_count_s.clone();
            let tdc_save = today_db_count_s.clone();
            let db_path_save = db_path.clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(60));
                flush_minute_count(&mc_save, &tdc_save, &db_path_save);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(move |app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            let db_path = app_handle.state::<DbPath>();
            flush_minute_count(&minute_count, &today_db_count, &db_path.0);
        }
    });
}
