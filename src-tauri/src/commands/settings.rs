//! Settings 相关 IPC 命令。See Architecture §5.2。

use std::sync::Arc;

use tauri::{AppHandle, Emitter, State};

use crate::platform::autostart;
use crate::store::{Settings, SettingsStore};

#[tauri::command]
pub fn get_settings(settings: State<'_, Arc<SettingsStore>>) -> Settings {
    settings.get()
}

/// 合并式更新设置并广播 settings-changed。返回更新后的完整设置。
#[tauri::command]
pub fn update_settings(
    app: AppHandle,
    settings: State<'_, Arc<SettingsStore>>,
    patch: serde_json::Value,
) -> Settings {
    let updated = settings.update(&patch);
    // autostart 同步到系统（FR-064）。
    autostart::sync_autostart(&app, updated.autostart);
    let _ = app.emit("settings-changed", &updated);
    updated
}

#[tauri::command]
pub fn pause_reminders(app: AppHandle, settings: State<'_, Arc<SettingsStore>>) {
    let updated = settings.update(&serde_json::json!({ "paused": true }));
    let _ = app.emit("settings-changed", &updated);
}

#[tauri::command]
pub fn resume_reminders(app: AppHandle, settings: State<'_, Arc<SettingsStore>>) {
    let updated = settings.update(&serde_json::json!({ "paused": false }));
    let _ = app.emit("settings-changed", &updated);
}

/// Onboard 完成：写入选定设置 + 标记 first_launch=false，广播设置变更。
#[tauri::command]
pub fn complete_onboard(
    app: AppHandle,
    settings: State<'_, Arc<SettingsStore>>,
    patch: serde_json::Value,
) -> Settings {
    let mut full = patch.clone();
    if let serde_json::Value::Object(map) = &mut full {
        map.insert("first_launch".into(), false.into());
    }
    let updated = settings.update(&full);
    autostart::sync_autostart(&app, updated.autostart);
    let _ = app.emit("settings-changed", &updated);
    updated
}
