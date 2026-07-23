//! Health Activity 数据模型。See Architecture §3.2, PRD §4.2 FR-031/032, ADR-0007。

use std::time::Instant;

use serde::Serialize;

use crate::store::Settings;

use super::state::RState;

/// Health Activity 静态配置。
#[derive(Clone, Copy)]
pub struct ActivityConfig {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub message_zh: &'static str,
    pub message_en: &'static str,
    pub action: &'static str,
    pub priority: u32,
}

pub const ACTIVITIES: &[ActivityConfig] = &[
    ActivityConfig {
        id: "water",
        name: "喝水",
        icon: "💧",
        message_zh: "去给自己接一杯水。",
        message_en: "May I trouble you for a glass of water?",
        action: "我喝了",
        priority: 1,
    },
    ActivityConfig {
        id: "stand",
        name: "站立",
        icon: "🧍",
        message_zh: "站起来活动一下。",
        message_en: "Please stretch a little.",
        action: "我站了",
        priority: 2,
    },
];

pub fn interval_for(id: &str, s: &Settings) -> u32 {
    match id {
        "water" => s.water_interval_min,
        "stand" => s.stand_interval_min,
        _ => 60,
    }
}

/// 传给前端的 Activity 信息（Overlay 渲染用）。
#[derive(Clone, Serialize)]
pub struct ActivityInfo {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub message: String,
    pub message_en: String,
    pub action: String,
    pub priority: u32,
}

/// 运行态：每个 Activity 的计时器与状态。
pub struct ActivityRuntime {
    pub cfg: ActivityConfig,
    pub accumulated_sec: u64,
    pub state: RState,
    pub snooze_until: Option<Instant>,
    pub triggered_at_iso: Option<String>,
    pub working_min_at_trigger: u32,
    /// 当前计时段的起始时间（上次 accumulated_sec 归零的时刻，本地未持久化）。
    /// 用于 Overlay 展示"起止时间"区间，方便统计。见 FR-080。
    pub started_at_iso: String,
}
