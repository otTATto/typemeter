mod db;
mod keystroke;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use chrono::Local;
use tauri::{Emitter, Manager};

use db::{flush_minute_count, init_db, query_hourly_counts, query_today_db_count, DbPath};

/// 指定日の1時間ごとのキーストローク数を返す Tauri コマンド
///
/// # Parameters
/// * `date` - 対象日付（`YYYY-MM-DD` 形式）
///
/// # Returns
/// 長さ 24 の Vec。インデックス = 時（0〜23）、値 = その時間帯の合計キーストローク数。
#[tauri::command]
fn get_hourly_counts(date: String, db_path: tauri::State<DbPath>) -> Vec<u64> {
    query_hourly_counts(&db_path.0, &date).to_vec()
}

fn resolve_db_path(app: &tauri::App) -> String {
    if let Some(path) = std::env::var("TYPEMETER_DB_PATH")
        .ok()
        .filter(|s| !s.is_empty())
    {
        let p = std::path::Path::new(&path);
        if let Some(parent) = p.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).expect("failed to create db directory");
            }
        }
        path
    } else {
        let db_dir = app
            .path()
            .app_data_dir()
            .expect("failed to get app data dir");
        std::fs::create_dir_all(&db_dir).expect("failed to create app data dir");
        db_dir.join("keystroke.db").to_string_lossy().into_owned()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let minute_count = Arc::new(Mutex::new(0u64));
    let today_db_count = Arc::new(Mutex::new(0u64));

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup({
            let minute_count = minute_count.clone();
            let today_db_count = today_db_count.clone();
            move |app| {
                let db_path = resolve_db_path(app);

                init_db(&db_path);
                *today_db_count.lock().unwrap() = query_today_db_count(&db_path);
                app.manage(DbPath(db_path.clone()));

                // ハートビート：初期表示・日付変更検知のため 1 秒ごとに emit
                let mc_emit = minute_count.clone();
                let tdc_emit = today_db_count.clone();
                let app_handle = app.handle().clone();
                thread::spawn(move || {
                    let mut last_date = Local::now().format("%Y-%m-%d").to_string();
                    loop {
                        thread::sleep(Duration::from_secs(1));
                        let current_date = Local::now().format("%Y-%m-%d").to_string();
                        if current_date != last_date {
                            *tdc_emit.lock().unwrap() = 0;
                            last_date = current_date;
                        }
                        let today_total = *tdc_emit.lock().unwrap() + *mc_emit.lock().unwrap();
                        let _ = app_handle.emit("keystroke_update", today_total);
                    }
                });

                // 1 分ごとに SQLite へ保存
                let mc_save = minute_count.clone();
                let tdc_save = today_db_count.clone();
                let db_path_save = db_path.clone();
                thread::spawn(move || loop {
                    thread::sleep(Duration::from_secs(60));
                    flush_minute_count(&mc_save, &tdc_save, &db_path_save);
                });

                Ok(())
            }
        })
        .invoke_handler(tauri::generate_handler![get_hourly_counts])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(move |app_handle, event| {
        match event {
            tauri::RunEvent::Ready => {
                keystroke::start_listening(
                    minute_count.clone(),
                    today_db_count.clone(),
                    app_handle.clone(),
                );
            }
            // macOS では赤×でウィンドウを閉じてもプロセスが残り Exit が来ないため、
            // CloseRequested で明示的に終了する
            tauri::RunEvent::WindowEvent {
                event: tauri::WindowEvent::CloseRequested { .. },
                ..
            } => {
                app_handle.exit(0);
            }
            tauri::RunEvent::Exit => {
                let db_path = app_handle.state::<DbPath>();
                flush_minute_count(&minute_count, &today_db_count, &db_path.0);
            }
            _ => {}
        }
    });
}
