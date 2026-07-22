//! Desktop IPC 命令。See Architecture §5.2。
//! 前端 → 后端的命令边界。命令只做参数解包 + 调用引擎/存储，不含业务逻辑。

use std::sync::Arc;

use tauri::{AppHandle, Emitter, State};

use crate::reminder::{self, ReminderTriggered, SharedEngine};
use crate::store::{HistoryStore, Settings, SettingsStore};

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
    sync_autostart(&app, updated.autostart);
    let _ = app.emit("settings-changed", &updated);
    updated
}

#[tauri::command]
pub fn reminder_complete(
    app: AppHandle,
    history: State<'_, Arc<HistoryStore>>,
    state: State<'_, SharedEngine>,
    activities: Vec<String>,
) {
    reminder::complete(&app, history.inner(), state.inner(), &activities);
}

#[tauri::command]
pub fn reminder_defer(
    app: AppHandle,
    settings: State<'_, Arc<SettingsStore>>,
    history: State<'_, Arc<HistoryStore>>,
    state: State<'_, SharedEngine>,
    activities: Vec<String>,
) {
    reminder::defer(&app, settings.inner(), history.inner(), state.inner(), &activities);
}

#[tauri::command]
pub fn reminder_skip(
    app: AppHandle,
    history: State<'_, Arc<HistoryStore>>,
    state: State<'_, SharedEngine>,
    activities: Vec<String>,
) {
    reminder::skip(&app, history.inner(), state.inner(), &activities);
}

#[tauri::command]
pub fn record_manual(
    app: AppHandle,
    history: State<'_, Arc<HistoryStore>>,
    state: State<'_, SharedEngine>,
    activity: String,
) {
    reminder::record_manual(&app, history.inner(), state.inner(), &activity);
}

#[tauri::command]
pub fn pause_reminders(
    app: AppHandle,
    settings: State<'_, Arc<SettingsStore>>,
) {
    let updated = settings.update(&serde_json::json!({ "paused": true }));
    let _ = app.emit("settings-changed", &updated);
}

#[tauri::command]
pub fn resume_reminders(
    app: AppHandle,
    settings: State<'_, Arc<SettingsStore>>,
) {
    let updated = settings.update(&serde_json::json!({ "paused": false }));
    let _ = app.emit("settings-changed", &updated);
}

/// Overlay 挂载时拉取当前提醒载荷（避免事件竞态）。
#[tauri::command]
pub fn get_current_reminder(state: State<'_, SharedEngine>) -> Option<ReminderTriggered> {
    reminder::current_payload(state.inner())
}

#[tauri::command]
pub fn get_current_state(state: State<'_, SharedEngine>) -> serde_json::Value {
    reminder::current_status(state.inner())
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
    sync_autostart(&app, updated.autostart);
    let _ = app.emit("settings-changed", &updated);
    updated
}

/// 同步开机自启到系统（tauri-plugin-autostart）。FR-064。
fn sync_autostart(app: &AppHandle, enable: bool) {
    use tauri_plugin_autostart::ManagerExt;
    let result = if enable {
        app.autolaunch().enable()
    } else {
        app.autolaunch().disable()
    };
    if let Err(e) = result {
        eprintln!("[water-me] autostart sync failed: {e}");
    }
}
