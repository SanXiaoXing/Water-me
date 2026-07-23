//! History Store。append-only JSONL。See Architecture §4.2, ADR-0004。

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// History 单条记录。See Architecture §4.2, FR-080。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub activity: String,
    pub status: String, // Triggered | Completed | Deferred | Skipped
    pub triggered_at: String, // UTC ISO8601
    pub responded_at: Option<String>,
    pub working_duration_min: u32,
}

/// History 存储：append-only JSONL。写失败静默忽略（PRD §7）。
pub struct HistoryStore {
    path: PathBuf,
}

impl HistoryStore {
    pub fn new(data_dir: &std::path::Path) -> Self {
        Self {
            path: data_dir.join("history.jsonl"),
        }
    }

    pub fn append(&self, record: &HistoryRecord) {
        let line = serde_json::to_string(record).unwrap_or_default();
        let mut file = match fs::OpenOptions::new().create(true).append(true).open(&self.path) {
            Ok(f) => f,
            Err(_) => return, // 静默忽略，不阻塞主流程。See PRD §7。
        };
        let _ = writeln!(file, "{}", line);
    }

    pub fn now_utc() -> String {
        Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
}
