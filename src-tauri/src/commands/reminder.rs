//! Reminder 相关 IPC 命令。See Architecture §5.2。

use tauri::{AppHandle, State};

use crate::reminder::{self, ReminderTriggered, SharedEngine};
use crate::store::{HistoryStore, SettingsStore};

#[tauri::command]
pub fn reminder_complete(
    app: AppHandle,
    history: State<'_, std::sync::Arc<HistoryStore>>,
    state: State<'_, SharedEngine>,
    activities: Vec<String>,
) {
    reminder::complete(&app, history.inner(), state.inner(), &activities);
}

#[tauri::command]
pub fn reminder_defer(
    app: AppHandle,
    settings: State<'_, std::sync::Arc<SettingsStore>>,
    history: State<'_, std::sync::Arc<HistoryStore>>,
    state: State<'_, SharedEngine>,
    activities: Vec<String>,
) {
    reminder::defer(&app, settings.inner(), history.inner(), state.inner(), &activities);
}

#[tauri::command]
pub fn reminder_skip(
    app: AppHandle,
    history: State<'_, std::sync::Arc<HistoryStore>>,
    state: State<'_, SharedEngine>,
    activities: Vec<String>,
) {
    reminder::skip(&app, history.inner(), state.inner(), &activities);
}

#[tauri::command]
pub fn record_manual(
    app: AppHandle,
    history: State<'_, std::sync::Arc<HistoryStore>>,
    state: State<'_, SharedEngine>,
    activity: String,
) {
    reminder::record_manual(&app, history.inner(), state.inner(), &activity);
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
