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

/// About ウィンドウを表示する。既に表示中の場合はフォーカスする
///
/// # Behavior
/// About ウィンドウは起動時に `visible: false` で生成済みのため、
/// ここでは show・center・set_focus のみを行う。
fn open_about_window_impl(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("about") {
        if let Err(e) = window.show() {
            eprintln!("[typemeter] failed to show about window: {e}");
        }
        if let Err(e) = window.center() {
            eprintln!("[typemeter] failed to center about window: {e}");
        }
        if let Err(e) = window.set_focus() {
            eprintln!("[typemeter] failed to focus about window: {e}");
        }
    }
}

/// About ウィンドウを開く Tauri コマンド（Windows/Linux のカスタムメニューから呼び出される）
#[tauri::command]
fn open_about_window(app: tauri::AppHandle) {
    open_about_window_impl(&app);
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
        .plugin(tauri_plugin_os::init())
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

                // macOS: ネイティブメニューバーに typemeter > About typemeter を追加
                #[cfg(target_os = "macos")]
                {
                    use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
                    let about_item = MenuItemBuilder::with_id("about_typemeter", "About typemeter")
                        .build(app)?;
                    let about_id = about_item.id().clone();
                    let typemeter_submenu = SubmenuBuilder::new(app, "typemeter")
                        .item(&about_item)
                        .build()?;
                    let menu = MenuBuilder::new(app).item(&typemeter_submenu).build()?;
                    app.set_menu(menu)?;
                    app.on_menu_event(move |app_handle, event| {
                        if event.id() == &about_id {
                            open_about_window_impl(app_handle);
                        }
                    });
                }

                // Windows/Linux: ネイティブ装飾を除去してカスタムタイトルバーに切り替え
                #[cfg(not(target_os = "macos"))]
                {
                    for window in app.webview_windows().values() {
                        window.set_decorations(false)?;
                    }
                }

                // visible: false で起動しているためここで表示する（装飾変更後に表示することでちらつきを防ぐ）
                if let Some(window) = app.get_webview_window("main") {
                    window.show()?;
                }

                Ok(())
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_hourly_counts,
            open_about_window
        ])
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
            // CloseRequested で明示的に終了する（About などのサブウィンドウは対象外）
            tauri::RunEvent::WindowEvent {
                label,
                event: tauri::WindowEvent::CloseRequested { api, .. },
                ..
            } if label == "about" => {
                api.prevent_close();
                if let Some(window) = app_handle.get_webview_window("about") {
                    let _ = window.hide();
                }
            }
            tauri::RunEvent::WindowEvent {
                label,
                event: tauri::WindowEvent::CloseRequested { .. },
                ..
            } if label == "main" => {
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
