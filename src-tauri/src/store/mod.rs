//! Persistent Storage：Settings（JSON）+ History（JSONL）。
//! See Architecture §4, ADR-0004。

pub mod history;
pub mod settings;

pub use history::{HistoryRecord, HistoryStore};
pub use settings::{Settings, SettingsStore};
