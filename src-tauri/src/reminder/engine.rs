//! Reminder Engine 核心。Working Timer（基于真实工作时长，非时钟）。
//! See Architecture §3.2-3.3, PRD §4.2-4.4, ADR-0001/0002/0006。

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::activity::{self, ActivityState};
use crate::platform::fullscreen;
use crate::store::{HistoryRecord, HistoryStore, SettingsStore};

use super::activity::{ActivityConfig, ActivityInfo, ActivityRuntime, ACTIVITIES, interval_for};
use super::state::RState;

/// reminder-triggered 事件载荷。See Architecture §5.2。
#[derive(Clone, Serialize)]
pub struct ReminderTriggered {
    pub activities: Vec<ActivityInfo>,
    pub title: String,
    pub title_en: String,
    pub duration_min: u32,
}

pub struct EngineState {
    pub activities: Vec<ActivityRuntime>,
    pub current_state: ActivityState,
    pub overlay_active: bool,
    pub cooldown_until: Option<Instant>,
    pub current_payload: Option<ReminderTriggered>,
}

impl EngineState {
    pub fn new() -> Self {
        Self {
            activities: ACTIVITIES
                .iter()
                .map(|cfg| ActivityRuntime {
                    cfg: *cfg,
                    accumulated_sec: 0,
                    state: RState::Pending,
                    snooze_until: None,
                    triggered_at_iso: None,
                    working_min_at_trigger: 0,
                })
                .collect(),
            current_state: ActivityState::Idle,
            overlay_active: false,
            cooldown_until: None,
            current_payload: None,
        }
    }

    /// 当前工作时长（分钟），取所有 Pending 活动的最大累计。
    pub fn working_duration_min(&self) -> u32 {
        self.activities
            .iter()
            .map(|a| (a.accumulated_sec / 60) as u32)
            .max()
            .unwrap_or(0)
    }
}

pub type SharedEngine = Arc<Mutex<EngineState>>;

/// 启动引擎后台循环。在 setup 中调用。
pub fn spawn(
    app: AppHandle,
    settings: Arc<SettingsStore>,
    history: Arc<HistoryStore>,
    state: SharedEngine,
) {
    eprintln!("[water-me] reminder engine spawned");
    std::thread::spawn(move || engine_loop(app, settings, history, state));
}

fn engine_loop(
    app: AppHandle,
    settings: Arc<SettingsStore>,
    history: Arc<HistoryStore>,
    state: SharedEngine,
) {
    const TICK: Duration = Duration::from_secs(2);
    let mut tick_count: u64 = 0;
    loop {
        std::thread::sleep(TICK);
        let s = settings.get();

        // DND：全屏应用时 Activity/Working 继续，仅 Reminder 暂停弹出（FR-014~018）。
        let dnd = s.fullscreen_reminder && fullscreen::is_fullscreen_dnd();
        // idle_threshold_min 是分钟，poll_state 期望秒，需 ×60。
        let new_state = activity::poll_state(s.idle_threshold_min * 60);

        let mut st = state.lock().unwrap();

        // 状态变化 → 发事件（前端 Settings 可显示当前状态）。
        if st.current_state != new_state {
            let prev = st.current_state;
            st.current_state = new_state;
            let _ = app.emit(
                "activity-state-changed",
                serde_json::json!({
                    "from": prev.as_str(),
                    "to": new_state.as_str(),
                    "working_duration_min": st.working_duration_min(),
                }),
            );
        }

        // Working 时累加 Pending 活动的工作时长（FR-057/058）。
        if new_state == ActivityState::Working {
            for a in st.activities.iter_mut() {
                if a.state == RState::Pending {
                    a.accumulated_sec += TICK.as_secs();
                }
            }
        }

        // 触发判定：无 Overlay、未暂停、非 DND、冷却已过。
        let now = Instant::now();
        let can_trigger =
            !st.overlay_active && !s.paused && !dnd && st.cooldown_until.map_or(true, |t| now > t);

        // 诊断日志：每 5 tick（约 10 秒）打印一次进度，方便排查"为何不触发"。
        tick_count += 1;
        if tick_count % 5 == 0 {
            eprintln!(
                "[water-me] state={} dnd={} can_trigger={} paused={} | water={}/{}min stand={}/{}min",
                new_state.as_str(),
                dnd,
                can_trigger,
                s.paused,
                st.activities[0].accumulated_sec / 60,
                s.water_interval_min,
                st.activities[1].accumulated_sec / 60,
                s.stand_interval_min,
            );
        }

        if can_trigger {
            // (cfg, working_min_at_trigger) 对，供 build_payload 取真实工作时长。
            let mut triggered: Vec<(ActivityConfig, u32)> = Vec::new();
            for a in st.activities.iter_mut() {
                let interval_sec = (interval_for(a.cfg.id, &s) as u64) * 60;
                let fire = match a.state {
                    RState::Pending => a.accumulated_sec >= interval_sec,
                    RState::Deferred => a.snooze_until.map_or(false, |t| now >= t),
                    RState::Triggered => false,
                };
                if fire {
                    a.state = RState::Triggered;
                    let triggered_at = HistoryStore::now_utc();
                    a.triggered_at_iso = Some(triggered_at.clone());
                    a.working_min_at_trigger = (a.accumulated_sec / 60) as u32;
                    let cfg = a.cfg;
                    let working_min = a.working_min_at_trigger;
                    triggered.push((cfg, working_min));
                    // FR-083：已触发即写 History（status: Triggered）。
                    history.append(&HistoryRecord {
                        activity: cfg.id.to_string(),
                        status: "Triggered".to_string(),
                        triggered_at,
                        responded_at: None,
                        working_duration_min: working_min,
                    });
                }
            }

            if !triggered.is_empty() {
                eprintln!(
                    "[water-me] TRIGGERED: {:?}",
                    triggered.iter().map(|(c, m)| format!("{}@{}min", c.id, m)).collect::<Vec<_>>()
                );
                let payload = build_payload(&triggered);
                st.overlay_active = true;
                st.current_payload = Some(payload.clone());
                drop(st);
                show_overlay(&app, &payload);
                let _ = app.emit("reminder-triggered", &payload);
                continue;
            }
        }
    }
}

/// 合并多 Activity 为单一 Overlay（FR-042~045）。按 priority 升序排序，主文案合并。
fn build_payload(triggered: &[(ActivityConfig, u32)]) -> ReminderTriggered {
    let mut items: Vec<(ActivityConfig, u32)> = triggered.to_vec();
    items.sort_by_key(|(a, _)| a.priority);

    let ids: Vec<&str> = items.iter().map(|(a, _)| a.id).collect();
    let has_water = ids.contains(&"water");
    let has_stand = ids.contains(&"stand");

    let (title, title_en) = if has_water && has_stand {
        (
            "去接杯水，顺便活动一下。".to_string(),
            "A glass of water, and a stretch.".to_string(),
        )
    } else {
        (
            items[0].0.message_zh.to_string(),
            items[0].0.message_en.to_string(),
        )
    };

    let duration_min = items.iter().map(|(_, m)| *m).max().unwrap_or(0);

    let activities = items
        .iter()
        .map(|(c, _)| ActivityInfo {
            id: c.id.to_string(),
            name: c.name.to_string(),
            icon: c.icon.to_string(),
            message: c.message_zh.to_string(),
            message_en: c.message_en.to_string(),
            action: c.action.to_string(),
            priority: c.priority,
        })
        .collect();

    ReminderTriggered {
        activities,
        title,
        title_en,
        duration_min,
    }
}

/// 创建 / 复用 Overlay 窗口并推送载荷。See ADR-0005（关闭即销毁）。
fn show_overlay(app: &AppHandle, payload: &ReminderTriggered) {
    if let Some(win) = app.get_webview_window("overlay") {
        let _ = win.emit("reminder-triggered", payload);
        return;
    }
    let label = "overlay".to_string();
    let result = WebviewWindowBuilder::new(app, &label, WebviewUrl::App("index.html".into()))
        .title("Water Me")
        .fullscreen(true)
        .always_on_top(true)
        .transparent(true)
        .decorations(false)
        .skip_taskbar(true)
        .resizable(false)
        .build();
    match result {
        Ok(win) => {
            let _ = win.emit("reminder-triggered", payload);
        }
        Err(e) => {
            eprintln!("[water-me] overlay build failed: {e}");
        }
    }
}

/// 关闭 Overlay 窗口（销毁）。
fn close_overlay(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("overlay") {
        let _ = win.close();
    }
}

// ============ 命令处理（被 commands/ 调用）============

/// 用户点击"我喝了/我站了"（Done）。重置计时器，写 History，设冷却（FR-039/046）。
pub fn complete(
    app: &AppHandle,
    history: &HistoryStore,
    state: &SharedEngine,
    activities: &[String],
) {
    let now = Instant::now();
    let now_iso = HistoryStore::now_utc();
    let mut st = state.lock().unwrap();
    for id in activities {
        if let Some(a) = st.activities.iter_mut().find(|a| a.cfg.id == id) {
            if a.state == RState::Triggered || a.state == RState::Deferred {
                let triggered_at = a.triggered_at_iso.clone().unwrap_or_else(|| now_iso.clone());
                history.append(&HistoryRecord {
                    activity: id.clone(),
                    status: "Completed".to_string(),
                    triggered_at,
                    responded_at: Some(now_iso.clone()),
                    working_duration_min: a.working_min_at_trigger,
                });
                a.state = RState::Pending;
                a.accumulated_sec = 0; // 重置计时器（FR-039）
                a.snooze_until = None;
                a.triggered_at_iso = None;
            }
        }
    }
    st.overlay_active = false;
    st.current_payload = None;
    st.cooldown_until = Some(now + Duration::from_secs(10)); // FR-046
    drop(st);
    close_overlay(app);
    let _ = app.emit("reminder-completed", serde_json::json!({ "activities": activities }));
}

/// 用户点击"N 分钟后"（Snooze）。不重置计时器，N 分钟后再触发（FR-040）。
pub fn defer(
    app: &AppHandle,
    settings: &SettingsStore,
    history: &HistoryStore,
    state: &SharedEngine,
    activities: &[String],
) {
    let snooze = Duration::from_secs((settings.get().snooze_interval_min as u64) * 60);
    let now = Instant::now();
    let now_iso = HistoryStore::now_utc();
    let snooze_until = now + snooze;
    let mut st = state.lock().unwrap();
    for id in activities {
        if let Some(a) = st.activities.iter_mut().find(|a| a.cfg.id == id) {
            if a.state == RState::Triggered {
                let triggered_at = a.triggered_at_iso.clone().unwrap_or_else(|| now_iso.clone());
                history.append(&HistoryRecord {
                    activity: id.clone(),
                    status: "Deferred".to_string(),
                    triggered_at,
                    responded_at: Some(now_iso.clone()),
                    working_duration_min: a.working_min_at_trigger,
                });
                a.state = RState::Deferred;
                a.snooze_until = Some(snooze_until);
            }
        }
    }
    st.overlay_active = false;
    st.current_payload = None;
    drop(st);
    close_overlay(app);
    let _ = app.emit("reminder-deferred", serde_json::json!({ "activities": activities }));
}

/// 用户点击"今天跳过"（Skip）。重置计时器（FR-041）。
pub fn skip(
    app: &AppHandle,
    history: &HistoryStore,
    state: &SharedEngine,
    activities: &[String],
) {
    let now_iso = HistoryStore::now_utc();
    let mut st = state.lock().unwrap();
    for id in activities {
        if let Some(a) = st.activities.iter_mut().find(|a| a.cfg.id == id) {
            if a.state == RState::Triggered || a.state == RState::Deferred {
                let triggered_at = a.triggered_at_iso.clone().unwrap_or_else(|| now_iso.clone());
                history.append(&HistoryRecord {
                    activity: id.clone(),
                    status: "Skipped".to_string(),
                    triggered_at,
                    responded_at: Some(now_iso.clone()),
                    working_duration_min: a.working_min_at_trigger,
                });
                a.state = RState::Pending;
                a.accumulated_sec = 0; // 重置计时器（FR-041）
                a.snooze_until = None;
                a.triggered_at_iso = None;
            }
        }
    }
    st.overlay_active = false;
    st.current_payload = None;
    drop(st);
    close_overlay(app);
    let _ = app.emit("reminder-skipped", serde_json::json!({ "activities": activities }));
}

/// Tray"立即喝水"：手动记录 Completed + 重置计时器（FR-071）。
pub fn record_manual(
    app: &AppHandle,
    history: &HistoryStore,
    state: &SharedEngine,
    activity_id: &str,
) {
    let now_iso = HistoryStore::now_utc();
    let mut st = state.lock().unwrap();
    if let Some(a) = st.activities.iter_mut().find(|a| a.cfg.id == activity_id) {
        history.append(&HistoryRecord {
            activity: activity_id.to_string(),
            status: "Completed".to_string(),
            triggered_at: now_iso.clone(),
            responded_at: Some(now_iso),
            working_duration_min: (a.accumulated_sec / 60) as u32,
        });
        a.state = RState::Pending;
        a.accumulated_sec = 0;
        a.snooze_until = None;
        a.triggered_at_iso = None;
    }
    drop(st);
    let _ = app.emit("reminder-completed", serde_json::json!({ "activity": activity_id }));
}

/// 获取当前 Overlay 应展示的载荷（Overlay 挂载时拉取，避免事件竞态）。
pub fn current_payload(state: &SharedEngine) -> Option<ReminderTriggered> {
    state.lock().unwrap().current_payload.clone()
}

/// 当前状态快照（给 Settings 显示用）。
pub fn current_status(state: &SharedEngine) -> serde_json::Value {
    let st = state.lock().unwrap();
    serde_json::json!({
        "activity_state": st.current_state.as_str(),
        "working_duration_min": st.working_duration_min(),
    })
}
