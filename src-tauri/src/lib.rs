use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use chrono::Local;
use rusqlite::{params, Connection};
use tauri::{Emitter, Manager, State};

struct DbPath(String);

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
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let db_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&db_dir)?;
            let db_path = db_dir.join("keystroke.db").to_string_lossy().into_owned();

            init_db(&db_path);

            let session_count = Arc::new(Mutex::new(0u64));
            let minute_count = Arc::new(Mutex::new(0u64));

            app.manage(DbPath(db_path.clone()));

            // rdev keyboard listener
            let session_count_rdev = session_count.clone();
            let minute_count_rdev = minute_count.clone();
            thread::spawn(move || {
                rdev::listen(move |event| {
                    if matches!(event.event_type, rdev::EventType::KeyPress(_)) {
                        *session_count_rdev.lock().unwrap() += 1;
                        *minute_count_rdev.lock().unwrap() += 1;
                    }
                })
                .expect("failed to start keyboard listener");
            });

            // emit session count to frontend every second
            let session_count_emit = session_count.clone();
            let app_handle = app.handle().clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(1));
                let count = *session_count_emit.lock().unwrap();
                let _ = app_handle.emit("keystroke_update", count);
            });

            // persist minute count to SQLite every minute
            let minute_count_save = minute_count.clone();
            let db_path_save = db_path.clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(60));
                let count = {
                    let mut lock = minute_count_save.lock().unwrap();
                    let c = *lock;
                    *lock = 0;
                    c
                };
                if count > 0 {
                    if let Ok(conn) = Connection::open(&db_path_save) {
                        let recorded_at = Local::now().to_rfc3339();
                        let _ = conn.execute(
                            "INSERT INTO keystroke_logs (recorded_at, minute_count) VALUES (?1, ?2)",
                            params![recorded_at, count as i64],
                        );
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_today_count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
