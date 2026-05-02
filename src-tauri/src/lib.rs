use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use chrono::Local;
use rusqlite::{params, Connection};
use tauri::{Emitter, Manager};

// 他アプリへのキー入力監視に必要な Input Monitoring 権限を要求する
#[cfg(target_os = "macos")]
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGRequestListenEventAccess() -> bool;
    fn CGEventTapCreate(
        tap: u32,
        place: u32,
        options: u32,
        events_of_interest: u64,
        callback: unsafe extern "C" fn(*mut c_void, u32, *mut c_void, *mut c_void) -> *mut c_void,
        user_info: *mut c_void,
    ) -> *mut c_void;
}

#[cfg(target_os = "macos")]
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFMachPortCreateRunLoopSource(
        allocator: *const c_void,
        port: *mut c_void,
        order: isize,
    ) -> *mut c_void;
    fn CFRunLoopAddSource(rl: *mut c_void, source: *mut c_void, mode: *const c_void);
    fn CFRunLoopGetCurrent() -> *mut c_void;
    fn CFRunLoopRun();
    fn CFRelease(cf: *const c_void);
    static kCFRunLoopCommonModes: *const c_void;
}

struct DbPath(String);

#[cfg(target_os = "macos")]
struct TapUserData {
    minute_count: Arc<Mutex<u64>>,
    today_db_count: Arc<Mutex<u64>>,
    app_handle: tauri::AppHandle,
}

/// CGEventTap コールバック
///
/// kCGEventKeyDown のみカウントする。TSM API（TSMGetInputSourceProperty 等）は
/// 呼び出さないため、macOS Tahoe (26) での dispatch_assert_queue クラッシュは発生しない。
#[cfg(target_os = "macos")]
unsafe extern "C" fn tap_callback(
    _proxy: *mut c_void,
    event_type: u32,
    event: *mut c_void,
    user_info: *mut c_void,
) -> *mut c_void {
    const CG_EVENT_KEY_DOWN: u32 = 10;
    if event_type == CG_EVENT_KEY_DOWN {
        let data = &*(user_info as *const TapUserData);
        *data.minute_count.lock().unwrap() += 1;
        let today_total = *data.today_db_count.lock().unwrap() + *data.minute_count.lock().unwrap();
        let _ = data.app_handle.emit("keystroke_update", today_total);
    }
    event
}

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
fn flush_minute_count(
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

    // macOS: CGEventTap はイベントループ起動後（RunEvent::Ready）で登録する
    #[cfg(target_os = "macos")]
    let minute_count_ready = minute_count.clone();
    #[cfg(target_os = "macos")]
    let today_db_count_ready = today_db_count.clone();

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let db_path = if let Some(path) = std::env::var("TYPEMETER_DB_PATH")
                .ok()
                .filter(|s| !s.is_empty())
            {
                let p = std::path::Path::new(&path);
                if let Some(parent) = p.parent() {
                    if !parent.as_os_str().is_empty() {
                        std::fs::create_dir_all(parent)?;
                    }
                }
                path
            } else {
                let db_dir = app
                    .path()
                    .app_data_dir()
                    .expect("failed to get app data dir");
                std::fs::create_dir_all(&db_dir)?;
                db_dir.join("keystroke.db").to_string_lossy().into_owned()
            };

            init_db(&db_path);
            *today_db_count_s.lock().unwrap() = query_today_db_count(&db_path);
            app.manage(DbPath(db_path.clone()));

            // グローバルキーボードイベントのリスニング（キー押下時に即 emit）
            // macOS: CGEventTap を直接使用（TSM API を呼ばない最小限のコールバック）
            //        RunEvent::Ready でイベントループ起動後に登録する
            // その他: rdev をそのまま使用する
            #[cfg(not(target_os = "macos"))]
            {
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
            }

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
        match event {
            #[cfg(target_os = "macos")]
            tauri::RunEvent::Ready => {
                // Input Monitoring 権限がなければシステム設定への誘導ダイアログを表示する
                unsafe { CGRequestListenEventAccess() };

                let data = Box::new(TapUserData {
                    minute_count: minute_count_ready.clone(),
                    today_db_count: today_db_count_ready.clone(),
                    app_handle: app_handle.clone(),
                });
                let data_ptr = Box::into_raw(data) as *mut c_void;

                let tap = unsafe {
                    CGEventTapCreate(
                        2, // kCGAnnotatedSessionEventTap
                        1, // kCGTailAppendEventTap
                        1, // kCGEventTapOptionListenOnly
                        1 << 10, // kCGEventKeyDown mask
                        tap_callback,
                        data_ptr,
                    )
                };

                if tap.is_null() {
                    let _ = app_handle.emit("listener_error", "入力監視の開始に失敗しました");
                    unsafe { drop(Box::from_raw(data_ptr as *mut TapUserData)) };
                    return;
                }

                // *mut c_void は Send でないため usize で渡す（アドレス値のみ保持）
                let tap_addr = tap as usize;
                thread::spawn(move || {
                    let tap = tap_addr as *mut c_void;
                    let source =
                        unsafe { CFMachPortCreateRunLoopSource(std::ptr::null(), tap, 0) };
                    let run_loop = unsafe { CFRunLoopGetCurrent() };
                    unsafe {
                        CFRunLoopAddSource(run_loop, source, kCFRunLoopCommonModes);
                        CFRelease(source as *const c_void);
                        CFRunLoopRun();
                    }
                });
            }
            // macOS では赤×でウィンドウを閉じてもプロセスが残り Exit が来ないため、
            // CloseRequested でフラッシュしてから明示的に終了する
            tauri::RunEvent::WindowEvent {
                event: tauri::WindowEvent::CloseRequested { .. },
                ..
            } => {
                let db_path = app_handle.state::<DbPath>();
                flush_minute_count(&minute_count, &today_db_count, &db_path.0);
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
