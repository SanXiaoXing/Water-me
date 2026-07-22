//! 持久化存储：Settings（JSON）+ History（JSONL append-only）。
//! See Architecture §4, ADR-0004.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use chrono::Utc;
use serde::{Deserialize, Serialize};

const SETTINGS_VERSION: u32 = 1;

/// 用户设置。Schema 见 Architecture §4.1。
/// `first_launch` / `paused` 是运行态与引导态标志，一并持久化。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub version: u32,
    pub water_interval_min: u32,
    pub stand_interval_min: u32,
    pub idle_threshold_min: u32,
    pub snooze_interval_min: u32,
    pub autostart: bool,
    pub fullscreen_reminder: bool,
    pub first_launch: bool,
    pub paused: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: SETTINGS_VERSION,
            water_interval_min: 60,
            stand_interval_min: 45,
            idle_threshold_min: 5,
            snooze_interval_min: 10,
            autostart: false,
            fullscreen_reminder: true,
            first_launch: true,
            paused: false,
        }
    }
}

/// History 单条记录。See Architecture §4.2, FR-080。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub activity: String,
    pub status: String, // Triggered | Completed | Deferred | Skipped
    pub triggered_at: String, // UTC ISO8601
    pub responded_at: Option<String>,
    pub working_duration_min: u32,
}

/// 设置存储：内存缓存 + 文件落盘。
pub struct SettingsStore {
    inner: Mutex<Settings>,
    path: PathBuf,
}

impl SettingsStore {
    pub fn load(data_dir: &Path) -> Self {
        let path = data_dir.join("settings.json");
        let settings = match fs::read_to_string(&path) {
            Ok(text) => {
                match serde_json::from_str::<Settings>(&text) {
                    Ok(mut s) => {
                        if s.version > SETTINGS_VERSION {
                            // 降级不支持：回退默认 + 备份。See PRD §7。
                            let _ = fs::rename(&path, data_dir.join("settings.json.bak"));
                            Settings::default()
                        } else {
                            // version < CURRENT 走迁移（V1 只有 v1，无需迁移）。
                            s.version = SETTINGS_VERSION;
                            s
                        }
                    }
                    Err(_) => {
                        // 解析失败：回退默认 + 备份损坏文件。See PRD §7。
                        let _ = fs::rename(&path, data_dir.join("settings.json.bak"));
                        Settings::default()
                    }
                }
            }
            Err(_) => Settings::default(),
        };
        Self {
            inner: Mutex::new(settings),
            path,
        }
    }

    pub fn get(&self) -> Settings {
        self.inner.lock().unwrap().clone()
    }

    /// 合并更新并落盘。返回更新后的完整设置。
    pub fn update(&self, patch: &serde_json::Value) -> Settings {
        let mut current = serde_json::to_value(self.get()).unwrap_or_default();
        if let serde_json::Value::Object(map) = &mut current {
            if let serde_json::Value::Object(patch_map) = patch {
                for (k, v) in patch_map {
                    map.insert(k.clone(), v.clone());
                }
            }
            map.insert("version".into(), SETTINGS_VERSION.into());
        }
        let updated: Settings =
            serde_json::from_value(current).unwrap_or_else(|_| Settings::default());
        *self.inner.lock().unwrap() = updated.clone();
        let _ = fs::write(&self.path, serde_json::to_vec_pretty(&updated).unwrap_or_default());
        updated
    }
}

/// History 存储：append-only JSONL。写失败静默忽略（PRD §7）。
pub struct HistoryStore {
    path: PathBuf,
}

impl HistoryStore {
    pub fn new(data_dir: &Path) -> Self {
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
