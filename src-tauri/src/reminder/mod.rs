//! Reminder Engine。管理提醒规则、触发时机、提醒状态和生命周期。
//! See Architecture §3.2-3.3, PRD §4.2-4.4, ADR-0001/0002/0006。
//!
//! ponytail: V1 单线程引擎循环，无独立 Event Bus（架构的 Event Bus 用于解耦，
//! V1 的解耦需求由"引擎 → Tauri 事件 → 前端"这条链路天然满足）。
//! Scheduler / Rule / History 子模块当前内联在 engine.rs，无独立实现不拆文件。
//! 升级路径：拆出 EventBus + 独立 Scheduler 线程，接口已对齐 Architecture §3.2。

pub mod activity;
pub mod engine;
pub mod state;

pub use engine::{
    complete, current_payload, current_status, defer, record_manual, skip, spawn, EngineState,
    ReminderTriggered, SharedEngine,
};
