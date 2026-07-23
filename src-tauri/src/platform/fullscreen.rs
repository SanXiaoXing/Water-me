//! Fullscreen detection（独占全屏 → DND，FR-014）。
//! See Architecture §6.3, PRD §6.3。
//! 误判容忍：宁可漏判（不触发 DND），不可误判（错误进入 DND）。

#[cfg(windows)]
pub fn is_fullscreen_dnd() -> bool {
    use windows::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITOR_DEFAULTTONEAREST, MONITORINFO,
    };
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowRect};

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
