use std::sync::{Arc, Mutex};
use std::thread;

use tauri::Emitter;

// ---- macOS ---------------------------------------------------------------

#[cfg(target_os = "macos")]
use std::ffi::c_void;

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
        let count = {
            let mut mc = data.minute_count.lock().unwrap();
            *mc += 1;
            *mc
        };
        let today_total = *data.today_db_count.lock().unwrap() + count;
        let _ = data.app_handle.emit("keystroke_update", today_total);
    }
    event
}

/// CGEventTap でグローバルキーストロークの監視を開始する
///
/// # Behavior
/// * Input Monitoring 権限がなければシステム設定への誘導ダイアログを表示する
/// * `TapUserData` はコールバックがアプリ終了まで呼ばれ続けるため意図的にリークする
/// * CGEventTap と RunLoop Source の参照は適切に解放する
#[cfg(target_os = "macos")]
fn start_listening_macos(
    minute_count: Arc<Mutex<u64>>,
    today_db_count: Arc<Mutex<u64>>,
    app_handle: tauri::AppHandle,
) {
    unsafe { CGRequestListenEventAccess() };

    let data = Box::new(TapUserData {
        minute_count,
        today_db_count,
        app_handle: app_handle.clone(),
    });
    let data_ptr = Box::into_raw(data) as *mut c_void;

    // セッションイベントをアプリ側に注釈付きで届けるタップポイント
    const KCG_ANNOTATED_SESSION_EVENT_TAP: u32 = 2;
    // イベントパイプラインの末尾に追加（リッスン専用タップに適切）
    const KCG_TAIL_APPEND_EVENT_TAP: u32 = 1;
    // イベントを変更しないリッスン専用モード（デフォルト = 0 はイベント変更可）
    const KCG_EVENT_TAP_OPTION_LISTEN_ONLY: u32 = 1;
    // kCGEventKeyDown (= 10) に対応するビットマスク
    const KCG_EVENT_KEY_DOWN_MASK: u64 = 1 << 10;

    let tap = unsafe {
        CGEventTapCreate(
            KCG_ANNOTATED_SESSION_EVENT_TAP,
            KCG_TAIL_APPEND_EVENT_TAP,
            KCG_EVENT_TAP_OPTION_LISTEN_ONLY,
            KCG_EVENT_KEY_DOWN_MASK,
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
        let source = unsafe { CFMachPortCreateRunLoopSource(std::ptr::null(), tap, 0) };
        let run_loop = unsafe { CFRunLoopGetCurrent() };
        unsafe {
            CFRunLoopAddSource(run_loop, source, kCFRunLoopCommonModes);
            // source はランループが retain 済みのため自分側の参照を解放する
            CFRelease(source as *const c_void);
            // source が tap を retain 済みのため自分側の参照を解放する
            CFRelease(tap as *const c_void);
            CFRunLoopRun();
        }
    });
}

// ---- macOS 以外（rdev） --------------------------------------------------

/// rdev でグローバルキーストロークの監視を開始する
#[cfg(not(target_os = "macos"))]
fn start_listening_rdev(
    minute_count: Arc<Mutex<u64>>,
    today_db_count: Arc<Mutex<u64>>,
    app_handle: tauri::AppHandle,
) {
    let app_handle_err = app_handle.clone();
    thread::spawn(move || {
        if let Err(e) = rdev::listen(move |event| {
            if matches!(event.event_type, rdev::EventType::KeyPress(_)) {
                let count = {
                    let mut mc = minute_count.lock().unwrap();
                    *mc += 1;
                    *mc
                };
                let today_total = *today_db_count.lock().unwrap() + count;
                let _ = app_handle.emit("keystroke_update", today_total);
            }
        }) {
            let _ = app_handle_err.emit("listener_error", format!("{e:?}"));
        }
    });
}

// ---- 共通インターフェース -------------------------------------------------

/// グローバルキーストロークの監視を開始する
///
/// # Parameters
/// * `minute_count` - 直近の未保存キーストローク数
/// * `today_db_count` - 今日の DB 保存済み合計
/// * `app_handle` - キーストローク検知時に `keystroke_update` イベントを emit するハンドル
///
/// # Behavior
/// * macOS: CGEventTap を使用（Input Monitoring 権限が必要）
/// * その他: rdev を使用
pub fn start_listening(
    minute_count: Arc<Mutex<u64>>,
    today_db_count: Arc<Mutex<u64>>,
    app_handle: tauri::AppHandle,
) {
    #[cfg(target_os = "macos")]
    start_listening_macos(minute_count, today_db_count, app_handle);

    #[cfg(not(target_os = "macos"))]
    start_listening_rdev(minute_count, today_db_count, app_handle);
}
