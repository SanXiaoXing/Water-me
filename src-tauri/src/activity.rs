//! Activity Monitor。监听用户活动，输出统一状态。
//! See Architecture §3.1, PRD §4.1。
//!
//! ponytail: V1 用 `GetLastInputInfo` 轮询（2s）而非全局钩子。
//! 一次 API 调用即可拿到"最后一次输入的时刻"，只检测"有没有动"，
//! 不记录坐标/按键内容，天然满足隐私约束（FR-024~029）。
//! 锁屏 / 睡眠时无输入 → 自然进入 Idle → 计时暂停，行为正确（FR-020/021）。
//! 升级路径：WTSRegisterSessionNotification + PowerRegisterSuspendResumeNotification
//! 以区分 Locked / Sleeping 状态标签。

use serde::{Deserialize, Serialize};

/// 活动状态。Locked/Sleeping 在 V1 统一归入 Idle（行为一致：暂停计时）。
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
            // dwTime 是 GetTickCount 量纲（ms），与 GetTickCount64 低 32 位对齐。
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

/// 检测前台是否为独占全屏（用于 DND，FR-014）。
/// 误判容忍：宁可漏判（不触发 DND），不可误判（错误进入 DND）。See PRD §6.3。
#[cfg(windows)]
pub fn is_fullscreen_dnd() -> bool {
    use windows::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITOR_DEFAULTTONEAREST, MONITORINFO,
    };
    use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
    use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;

    unsafe {
        // windows 0.58: GetForegroundWindow 返回 HWND（非 Option），无效时为 null 句柄。
        let hwnd = GetForegroundWindow();
        if hwnd.is_invalid() {
            return false;
        }
        let mut wrect = windows::Win32::Foundation::RECT::default();
        if GetWindowRect(hwnd, &mut wrect).is_err() {
            return false;
        }
        let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        let mut mi = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if !GetMonitorInfoW(monitor, &mut mi).as_bool() {
            return false;
        }
        let m = mi.rcMonitor;
        // 前台窗口覆盖整块显示器即视为独占全屏。
        wrect.left <= m.left && wrect.top <= m.top && wrect.right >= m.right && wrect.bottom >= m.bottom
    }
}

#[cfg(not(windows))]
pub fn is_fullscreen_dnd() -> bool {
    false
}
