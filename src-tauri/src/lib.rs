//! Water Me — 应用入口。组装后端模块、托盘、窗口与 IPC 命令。
//! See Architecture §2, §5, §6。

mod activity;
mod commands;
mod reminder;
mod store;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use tauri::{
    image::Image, menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem}, tray::{MouseButton,
    MouseButtonState, TrayIconBuilder, TrayIconEvent}, AppHandle, Emitter, Manager,
    RunEvent, WebviewUrl, WebviewWindowBuilder,
};

use tauri_plugin_autostart::MacosLauncher;

use reminder::SharedEngine;
use store::{HistoryStore, SettingsStore};

/// 托盘"暂停/恢复"菜单项句柄，用于切换文案。
struct PauseToggle(MenuItem<tauri::Wry>);

/// 用户主动退出标志。托盘"退出"设为 true，ExitRequested 据此放行。
/// 无主窗口架构下，窗口关闭默认会触发退出，需阻止；只有主动退出才放行。
static USER_QUIT: AtomicBool = AtomicBool::new(false);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .invoke_handler(tauri::generate_handler![
            commands::get_settings,
            commands::update_settings,
            commands::reminder_complete,
            commands::reminder_defer,
            commands::reminder_skip,
            commands::record_manual,
            commands::pause_reminders,
            commands::resume_reminders,
            commands::get_current_reminder,
            commands::get_current_state,
            commands::complete_onboard,
        ])
        .setup(|app| {
            // 数据目录：存放 settings.json / history.jsonl。
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;

            let settings = Arc::new(SettingsStore::load(&data_dir));
            let history = Arc::new(HistoryStore::new(&data_dir));
            let engine: SharedEngine = Arc::new(Mutex::new(reminder::EngineState::new()));

            // 首次启动：显示 Onboard（FR-074~078）。
            let first_launch = settings.get().first_launch;
            if first_launch {
                show_onboard(app.handle());
            }

            // 启动引擎后台循环。
            reminder::spawn(app.handle().clone(), settings.clone(), history.clone(), engine.clone());

            // 托盘。
            build_tray(app.handle())?;

            // 注册共享状态供命令访问。
            app.manage(settings);
            app.manage(history);
            app.manage(engine);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| {
            // 无主窗口架构：窗口关闭默认触发退出，需阻止以保持托盘常驻。
            // 只有托盘"退出"设了 USER_QUIT 标志才放行。
            if let RunEvent::ExitRequested { api, .. } = event {
                if !USER_QUIT.load(Ordering::SeqCst) {
                    api.prevent_exit();
                }
            }
        });
}

/// 构建系统托盘 + 菜单。See UIUX §5.4, PRD §4.6。
fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let header = MenuItem::with_id(app, "wm-header", "🌱 Water Me", false, None::<&str>)?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let toggle = MenuItem::with_id(app, "wm-toggle-pause", "暂停提醒", true, None::<&str>)?;
    let water = MenuItem::with_id(app, "wm-water", "立即喝水 💧", true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let settings_item = MenuItem::with_id(app, "wm-settings", "设置", true, None::<&str>)?;
    let sep3 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "wm-quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[&header, &sep1, &toggle, &water, &sep2, &settings_item, &sep3, &quit],
    )?;

    // 托盘图标：复用应用默认窗口图标（来自 tauri.conf.json bundle.icon）。
    let icon = app
        .default_window_icon()
        .cloned()
        .unwrap_or_else(|| Image::new_owned(vec![0, 0, 0, 0], 1, 1));

    app.manage(PauseToggle(toggle));

    TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .tooltip("Water Me")
        .menu(&menu)
        .on_menu_event(on_menu_event)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_settings(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "wm-toggle-pause" => {
            let store = app.state::<Arc<SettingsStore>>();
            let was_paused = store.get().paused;
            let updated = store.update(&serde_json::json!({ "paused": !was_paused }));
            let _ = app.emit("settings-changed", &updated);
            // 切换菜单文案。
            if let Some(t) = app.try_state::<PauseToggle>() {
                let _ = t.0.set_text(if !was_paused { "恢复提醒" } else { "暂停提醒" });
            }
        }
        "wm-water" => {
            let history = app.state::<Arc<HistoryStore>>();
            let engine = app.state::<SharedEngine>();
            reminder::record_manual(app, history.inner(), engine.inner(), "water");
        }
        "wm-settings" => show_settings(app),
        "wm-quit" => {
            // 标记主动退出，放行 ExitRequested。
            USER_QUIT.store(true, Ordering::SeqCst);
            app.exit(0);
        }
        _ => {}
    }
}

/// 打开 / 聚焦 Settings 窗口（480×640）。See UIUX §5.3。
fn show_settings(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("settings") {
        let _ = w.set_focus();
        return;
    }
    let _ = WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("index.html".into()))
        .title("Water Me · 设置")
        .inner_size(480.0, 640.0)
        .resizable(false)
        .center()
        .build();
}

/// 打开 Onboard 窗口（480×640）。See UIUX §5.2。
fn show_onboard(app: &AppHandle) {
    let _ = WebviewWindowBuilder::new(app, "onboard", WebviewUrl::App("index.html".into()))
        .title("Water Me")
        .inner_size(480.0, 640.0)
        .resizable(false)
        .center()
        .build();
}
