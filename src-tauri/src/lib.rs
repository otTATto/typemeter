use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use chrono::Local;
use rusqlite::{params, Connection};
use tauri::{Emitter, Manager, State};

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

/// `minute_count` の現在値を SQLite に保存し、0 にリセットする
///
/// @param minute_count 直近 1 分間のカウントを保持する Mutex
/// @param db_path データベースファイルのパス
///
/// NOTE:
///   - count が 0 の場合は書き込みをスキップする
///   - 1 分タイマーとアプリ終了時の両方から呼び出される
fn flush_minute_count(minute_count: &Arc<Mutex<u64>>, db_path: &str) {
    let count = {
        let mut lock = minute_count.lock().unwrap();
        let c = *lock;
        *lock = 0;
        c
    };
    if count > 0 {
        if let Ok(conn) = Connection::open(db_path) {
            let recorded_at = Local::now().to_rfc3339();
            let _ = conn.execute(
                "INSERT INTO keystroke_logs (recorded_at, minute_count) VALUES (?1, ?2)",
                params![recorded_at, count as i64],
            );
        }
    }
}

/// 今日のキーストローク合計を SQLite から取得する（Tauri コマンド）
///
/// `recorded_at` が今日の日付で始まる行の `minute_count` を合計して返す。
///
/// # Returns
/// 今日保存済みのキーストローク数の合計。取得失敗時は `0`。
///
/// # Note
/// 現在のセッションで未保存の直近 1 分間分は含まない。
/// フロントエンドはこの値にセッションカウントを加算して今日の合計を表示する。
#[tauri::command]
fn get_today_count(db_path: State<DbPath>) -> u64 {
    let Ok(conn) = Connection::open(&db_path.0) else {
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
    let session_count = Arc::new(Mutex::new(0u64));
    let minute_count = Arc::new(Mutex::new(0u64));

    let session_count_s = session_count.clone();
    let minute_count_s = minute_count.clone();

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
            app.manage(DbPath(db_path.clone()));

            // rdev keyboard listener
            let sc_rdev = session_count_s.clone();
            let mc_rdev = minute_count_s.clone();
            thread::spawn(move || {
                rdev::listen(move |event| {
                    if matches!(event.event_type, rdev::EventType::KeyPress(_)) {
                        *sc_rdev.lock().unwrap() += 1;
                        *mc_rdev.lock().unwrap() += 1;
                    }
                })
                .expect("failed to start keyboard listener");
            });

            // emit session count to frontend every second
            let sc_emit = session_count_s.clone();
            let app_handle = app.handle().clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(1));
                let count = *sc_emit.lock().unwrap();
                let _ = app_handle.emit("keystroke_update", count);
            });

            // persist minute count to SQLite every minute
            let mc_save = minute_count_s.clone();
            let db_path_save = db_path.clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(60));
                flush_minute_count(&mc_save, &db_path_save);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_today_count])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(move |app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            let db_path = app_handle.state::<DbPath>();
            flush_minute_count(&minute_count, &db_path.0);
        }
    });
}
