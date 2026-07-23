//! Water Me — 应用入口。组装后端模块、托盘、窗口与 IPC 命令。
//! See Architecture §2, §5, §6。

mod activity;
mod commands;
mod platform;
mod reminder;
mod store;

use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use tauri::{Manager, RunEvent};
use tauri_plugin_autostart::MacosLauncher;

use platform::{build_tray, show_onboard, USER_QUIT};
use reminder::SharedEngine;
use store::{HistoryStore, SettingsStore};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::pause_reminders,
            commands::settings::resume_reminders,
            commands::settings::complete_onboard,
            commands::settings::list_visible_windows,
            commands::reminder::reminder_complete,
            commands::reminder::reminder_defer,
            commands::reminder::reminder_skip,
            commands::reminder::record_manual,
            commands::reminder::get_current_reminder,
            commands::reminder::get_current_state,
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
