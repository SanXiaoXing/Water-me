//! Settings Store。内存缓存 + JSON 落盘。See Architecture §4.1。

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

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
    /// 全屏免打扰黑名单（进程 exe basename，大小写不敏感匹配）。
    /// fullscreen_reminder=true 时，仅前台全屏应用在此列表内才 DND。
    /// 空列表 = 全屏照常提醒（适合全屏工作的人）。
    #[serde(default)]
    pub fullscreen_blocklist: Vec<String>,
    /// 主题："system"（跟随系统）| "light" | "dark"。默认 system。
    #[serde(default = "default_theme")]
    pub theme: String,
    /// 提醒弹窗形态："fullscreen"（全屏遮罩）| "card"（独立卡片）| "toast"（右下通知条）。
    /// 默认 fullscreen。See PRD FR-033，原型 docs/design/overlay-mode-picker.html。
    #[serde(default = "default_overlay_mode")]
    pub overlay_mode: String,
    pub first_launch: bool,
    pub paused: bool,
}

fn default_theme() -> String {
    "system".to_string()
}

fn default_overlay_mode() -> String {
    "fullscreen".to_string()
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
            fullscreen_blocklist: Vec::new(),
            theme: default_theme(),
            overlay_mode: default_overlay_mode(),
            first_launch: true,
            paused: false,
        }
    }
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
