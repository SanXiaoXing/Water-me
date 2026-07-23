//! ActivityState 枚举与轮询。See Architecture §3.1。
//!
//! ponytail: V1 将 Locked/Sleeping 统一归入 Idle（行为一致：暂停计时）。

use serde::{Deserialize, Serialize};

/// 活动状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActivityState {
    Idle,
    Working,
}

impl ActivityState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActivityState::Idle => "idle",
            ActivityState::Working => "working",
        }
    }
}

/// 查询当前是否在工作（距上次输入 < idle_threshold_sec）。
/// 非 Windows 平台恒返回 Working（V1 仅目标 Windows，见 PRD §1）。
#[cfg(windows)]
pub fn poll_state(idle_threshold_sec: u32) -> ActivityState {
    use windows::Win32::UI::Input::KeyboardAndMouse::GetLastInputInfo;
    use windows::Win32::UI::Input::KeyboardAndMouse::LASTINPUTINFO;

    unsafe {
        let mut info = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };
        if GetLastInputInfo(&mut info).as_bool() {
            // dwTime 是 GetTickCount 量纲（ms），与 GetTickCount 低 32 位对齐。
            let now = windows::Win32::System::SystemInformation::GetTickCount();
            let idle_ms = now.wrapping_sub(info.dwTime);
            if idle_ms / 1000 < idle_threshold_sec {
                return ActivityState::Working;
            }
        }
    }
    ActivityState::Idle
}

#[cfg(not(windows))]
pub fn poll_state(_idle_threshold_sec: u32) -> ActivityState {
    ActivityState::Working
}
