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

/// Settings ウィンドウを表示する。既に表示中の場合はフォーカスする
///
/// # Behavior
/// Settings ウィンドウは起動時に `visible: false` で生成済みのため、
/// ここでは show・center・set_focus のみを行う。
fn open_settings_window_impl(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        if let Err(e) = window.show() {
            eprintln!("[typemeter] failed to show settings window: {e}");
        }
        if let Err(e) = window.center() {
            eprintln!("[typemeter] failed to center settings window: {e}");
        }
        if let Err(e) = window.set_focus() {
            eprintln!("[typemeter] failed to focus settings window: {e}");
        }
    }
}

/// Settings ウィンドウを開く Tauri コマンド（Windows/Linux のカスタムメニューから呼び出される）
#[tauri::command]
fn open_settings_window(app: tauri::AppHandle) {
    open_settings_window_impl(&app);
}

/// メインウィンドウを表示してフォーカスする
///
/// # Parameters
/// * `app` - アプリケーションハンドル
///
/// # Behavior
/// * トレイメニュー・トレイアイコンのクリック・二重起動検知の各コールバックから共通で呼び出される
/// * macOS では常駐中に Dock アイコンを消しているため、表示前に activation policy を
///   `Regular` に戻して Dock アイコンを復帰させる
/// * 二重起動検知のコールバックはメインスレッド外（tokio ワーカー）で呼ばれるため、
///   policy 切替と表示を 1 つのクロージャにまとめてメインスレッドへディスパッチする
///   （別々の非同期メッセージとして流すと AppKit 側の反映が追いつかず表示に失敗することがある）
fn show_main_window(app: &tauri::AppHandle) {
    let app_handle = app.clone();
    let dispatch_result = app.run_on_main_thread(move || {
        #[cfg(target_os = "macos")]
        if let Err(e) = app_handle.set_activation_policy(tauri::ActivationPolicy::Regular) {
            eprintln!("[typemeter] failed to set activation policy: {e}");
        }

        if let Some(window) = app_handle.get_webview_window("main") {
            if let Err(e) = window.show() {
                eprintln!("[typemeter] failed to show main window: {e}");
            }
            if let Err(e) = window.unminimize() {
                eprintln!("[typemeter] failed to unminimize main window: {e}");
            }
            if let Err(e) = window.set_focus() {
                eprintln!("[typemeter] failed to focus main window: {e}");
            }
        }
    });
    if let Err(e) = dispatch_result {
        eprintln!("[typemeter] failed to dispatch show_main_window to main thread: {e}");
    }
}

/// システムトレイのメニュー・アイコン・イベントハンドラを構築して登録する
///
/// # Parameters
/// * `app` - アプリケーションハンドル
///
/// # Behavior
/// * メニュー構成（Open typemeter / Settings… / Quit typemeter）は OS 共通
/// * アイコン・クリック挙動は `configure_tray_platform` に切り出した OS 固有部分で分岐する
/// * setup 中ではなく `RunEvent::Ready` から呼び出すこと（macOS ではイベントループ開始前の
///   トレイ構築によるタイミング起因の不具合が報告されているため）
///
/// # Errors
/// メニューやトレイアイコンの構築に失敗した場合、その `tauri::Error` を返す
fn build_tray(app: &tauri::AppHandle) -> tauri::Result<()> {
    use tauri::menu::MenuBuilder;
    use tauri::menu::MenuItemBuilder;
    use tauri::tray::TrayIconBuilder;

    let open_item = MenuItemBuilder::with_id("tray_open", "Open typemeter").build(app)?;
    let open_id = open_item.id().clone();
    let settings_item = MenuItemBuilder::with_id("tray_settings", "Settings…").build(app)?;
    let settings_id = settings_item.id().clone();
    let quit_item = MenuItemBuilder::with_id("tray_quit", "Quit typemeter").build(app)?;
    let quit_id = quit_item.id().clone();

    let menu = MenuBuilder::new(app)
        .item(&open_item)
        .item(&settings_item)
        .separator()
        .item(&quit_item)
        .build()?;

    let builder = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("typemeter")
        .on_menu_event(move |app_handle, event| {
            if event.id() == &open_id {
                show_main_window(app_handle);
            } else if event.id() == &settings_id {
                open_settings_window_impl(app_handle);
            } else if event.id() == &quit_id {
                app_handle.exit(0);
            }
        });

    configure_tray_platform(builder, app)?.build(app)?;

    Ok(())
}

/// トレイアイコンの見た目とクリック挙動のうち、OS 固有の部分だけを設定する
///
/// # Parameters
/// * `builder` - `build_tray` で共通設定済みの `TrayIconBuilder`
/// * `app` - アイコン取得に使うアプリケーションハンドル（Windows/Linux でのみ使用）
///
/// # Behavior
/// * Windows/Linux: アプリのウィンドウアイコンを流用し、左クリック = メインウィンドウ表示、
///   右クリック = メニュー表示という Windows の慣例に合わせる
///   （`show_menu_on_left_click(false)` としたうえで左クリックのみ `show_main_window` を呼ぶ）
/// * macOS: 黒 + 透過のテンプレートアイコンを使い、クリックでメニューを開く macOS の慣例に
///   合わせて `show_menu_on_left_click` はデフォルト（true）のままとする
///
/// # Errors
/// アイコン画像の読み込みに失敗した場合、その `tauri::Error` を返す
#[cfg(not(target_os = "macos"))]
fn configure_tray_platform(
    builder: tauri::tray::TrayIconBuilder<tauri::Wry>,
    app: &tauri::AppHandle,
) -> tauri::Result<tauri::tray::TrayIconBuilder<tauri::Wry>> {
    use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};

    Ok(builder
        .icon(
            app.default_window_icon()
                .expect("default window icon is not set")
                .clone(),
        )
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        }))
}

#[cfg(target_os = "macos")]
fn configure_tray_platform(
    builder: tauri::tray::TrayIconBuilder<tauri::Wry>,
    _app: &tauri::AppHandle,
) -> tauri::Result<tauri::tray::TrayIconBuilder<tauri::Wry>> {
    let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/tray-macos.png"))?;
    Ok(builder.icon(icon).icon_as_template(true))
}

/// 起動時に保存済みの `alwaysOnTop` 設定をメインウィンドウへ適用する
///
/// # Behavior
/// * ストアの読み込みに失敗した場合はエラーを報告し、false 扱いとする
/// * `alwaysOnTop` キーが存在しない場合も false 扱いとする
/// * ちらつき防止のため、呼び出し側で `window.show()` より前に実行すること
fn apply_always_on_top_from_settings(app: &tauri::App) {
    use tauri_plugin_store::StoreExt;

    let store = match app.store("settings.json") {
        Ok(store) => store,
        Err(e) => {
            eprintln!("[typemeter] failed to load settings store: {e}");
            return;
        }
    };

    let is_always_on_top = store
        .get("alwaysOnTop")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);

    if !is_always_on_top {
        return;
    }

    if let Some(window) = app.get_webview_window("main") {
        if let Err(e) = window.set_always_on_top(true) {
            eprintln!("[typemeter] failed to set always-on-top: {e}");
        }
    }
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
        .plugin(tauri_plugin_single_instance::init(|app_handle, _args, _cwd| {
            show_main_window(app_handle);
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::default().build())
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

                // macOS: ネイティブメニューバーに typemeter > About typemeter / Settings… を追加
                #[cfg(target_os = "macos")]
                {
                    use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
                    let about_item = MenuItemBuilder::with_id("about_typemeter", "About typemeter")
                        .build(app)?;
                    let about_id = about_item.id().clone();
                    let settings_item = MenuItemBuilder::with_id("settings", "Settings…")
                        .accelerator("Cmd+,")
                        .build(app)?;
                    let settings_id = settings_item.id().clone();
                    let typemeter_submenu = SubmenuBuilder::new(app, "typemeter")
                        .item(&about_item)
                        .item(&settings_item)
                        .build()?;
                    let menu = MenuBuilder::new(app).item(&typemeter_submenu).build()?;
                    app.set_menu(menu)?;
                    app.on_menu_event(move |app_handle, event| {
                        if event.id() == &about_id {
                            open_about_window_impl(app_handle);
                        } else if event.id() == &settings_id {
                            open_settings_window_impl(app_handle);
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

                // 保存済みの alwaysOnTop 設定を適用する（ちらつき防止のため show() より前に行う）
                apply_always_on_top_from_settings(app);

                // visible: false で起動しているためここで表示する（装飾変更後に表示することでちらつきを防ぐ）
                if let Some(window) = app.get_webview_window("main") {
                    window.show()?;
                }

                Ok(())
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_hourly_counts,
            open_about_window,
            open_settings_window
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(move |app_handle, event| {
        match event {
            tauri::RunEvent::Ready => {
                // バックグラウンド常駐用のシステムトレイアイコンを構築する
                // （macOS のタイミング起因の不具合を避けるため、setup ではなく Ready で行う）
                if let Err(e) = build_tray(app_handle) {
                    eprintln!("[typemeter] failed to build tray icon: {e}");
                }
                keystroke::start_listening(
                    minute_count.clone(),
                    today_db_count.clone(),
                    app_handle.clone(),
                );
            }
            // macOS: 起動中に Finder や Dock のアプリアイコンから開き直されたときに
            // メインウィンドウを表示する（macOS では再起動は新プロセスにならず、
            // 既存プロセスへの reopen イベントとして届くため single-instance では拾えない）
            #[cfg(target_os = "macos")]
            tauri::RunEvent::Reopen { .. } => {
                show_main_window(app_handle);
            }
            // ウィンドウの × はプロセス終了ではなく非表示として扱う（バックグラウンド常駐）。
            // プロセスの終了はトレイメニューの Quit typemeter からのみ行う
            tauri::RunEvent::WindowEvent {
                label,
                event: tauri::WindowEvent::CloseRequested { api, .. },
                ..
            } if label == "main" || label == "about" || label == "settings" => {
                api.prevent_close();
                if let Some(window) = app_handle.get_webview_window(&label) {
                    if let Err(e) = window.hide() {
                        eprintln!("[typemeter] failed to hide {label} window: {e}");
                    }
                }

                // macOS: メインウィンドウを隠している間は Dock アイコンも消す
                #[cfg(target_os = "macos")]
                if label == "main" {
                    if let Err(e) =
                        app_handle.set_activation_policy(tauri::ActivationPolicy::Accessory)
                    {
                        eprintln!("[typemeter] failed to set activation policy: {e}");
                    }
                }
            }
            tauri::RunEvent::Exit => {
                let db_path = app_handle.state::<DbPath>();
                flush_minute_count(&minute_count, &today_db_count, &db_path.0);
            }
            _ => {}
        }
    });
}
