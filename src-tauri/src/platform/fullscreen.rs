//! Foreground process detection（前台进程检测）。
//! See Architecture §6.3, PRD §6.3。
//!
//! ponytail: 原设计只检测"独占全屏"（窗口 rect 覆盖显示器），但 Windows 里
//! 游戏窗口模式、PPT 非全屏演示、最大化 IDE 都可能需要 DND。
//! 改为检测"前台进程"而非"全屏状态"——黑名单应用只要在前台就静默，
//! 不要求窗口尺寸。

use serde::Serialize;
use std::path::Path;

/// 可见窗口信息（给 UI 列表选择用）。
#[derive(Debug, Clone, Serialize)]
pub struct WindowInfo {
    /// 窗口标题（用户可读，用于列表识别）。
    pub title: String,
    /// 进程 exe basename（加入黑名单的实际值）。
    pub process: String,
}

/// 当前前台应用的进程 exe basename（如 `steam.exe`）。取不到 → None。
/// 给 engine 后台 DND 轮询用。
#[cfg(windows)]
pub fn current_foreground_process() -> Option<String> {
    use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_invalid() {
            return None;
        }
        process_name_of(hwnd)
    }
}

#[cfg(not(windows))]
pub fn current_foreground_process() -> Option<String> {
    None
}

/// 列出当前所有可见、非最小化、非自身进程的顶层窗口。
/// 给 Settings"添加"按钮用——弹列表让用户选要加入黑名单的应用。
#[cfg(windows)]
pub fn list_visible_windows() -> Vec<WindowInfo> {
    use std::sync::Mutex;
    use windows::Win32::UI::WindowsAndMessaging::EnumWindows;

    let collected: Mutex<Vec<WindowInfo>> = Mutex::new(Vec::new());

    unsafe {
        let _ = EnumWindows(
            Some(enum_proc),
            windows::Win32::Foundation::LPARAM(&collected as *const _ as isize),
        );
    }

    // ponytail: EnumWindows 回调里拿不到 self_pid（闭包捕获不兼容 raw pointer
    // LPARAM），改在收集后过滤自身进程 + 去重（同进程多窗口只留第一个有标题的）。
    let mut all = collected.into_inner().unwrap();
    all.retain(|w| {
        // 过滤无标题窗口（系统托盘、辅助窗口等）。
        !w.title.is_empty()
    });
    // 去重：同进程名只保留第一个（标题最长的优先，便于识别）。
    all.sort_by_key(|w| std::cmp::Reverse(w.title.len()));
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    all.retain(|w| seen.insert(w.process.to_lowercase()));
    // 过滤掉 Water Me 自身。
    all.retain(|w| !is_self_process(&w.process));

    extern "system" fn enum_proc(
        hwnd: windows::Win32::Foundation::HWND,
        lparam: windows::Win32::Foundation::LPARAM,
    ) -> windows::Win32::Foundation::BOOL {
        use windows::Win32::UI::WindowsAndMessaging::{IsIconic, IsWindowVisible, GetWindowTextLengthW};
        unsafe {
            if !IsWindowVisible(hwnd).as_bool() || IsIconic(hwnd).as_bool() {
                return windows::Win32::Foundation::BOOL(1);
            }
            if GetWindowTextLengthW(hwnd) == 0 {
                return windows::Win32::Foundation::BOOL(1);
            }
            let collected: &Mutex<Vec<WindowInfo>> =
                &*(lparam.0 as *const Mutex<Vec<WindowInfo>>);
            if let (Some(title), Some(process)) = (window_title_of(hwnd), process_name_of(hwnd)) {
                if let Ok(mut guard) = collected.lock() {
                    guard.push(WindowInfo { title, process });
                }
            }
        }
        windows::Win32::Foundation::BOOL(1)
    }

    all
}

#[cfg(not(windows))]
pub fn list_visible_windows() -> Vec<WindowInfo> {
    Vec::new()
}

/// Water Me 自身进程名（大小写不敏感比较）。
#[cfg(windows)]
fn is_self_process(process: &str) -> bool {
    let self_name = std::env::current_exe()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_lowercase()));
    match self_name {
        Some(name) => process.to_lowercase() == name,
        None => false,
    }
}

/// 由 HWND 取窗口标题。
#[cfg(windows)]
fn window_title_of(hwnd: windows::Win32::Foundation::HWND) -> Option<String> {
    use windows::Win32::UI::WindowsAndMessaging::GetWindowTextW;
    unsafe {
        let mut buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buf);
        if len <= 0 {
            return None;
        }
        Some(String::from_utf16_lossy(&buf[..len as usize]))
    }
}

/// 由 HWND 取所属进程的 exe basename。
#[cfg(windows)]
fn process_name_of(hwnd: windows::Win32::Foundation::HWND) -> Option<String> {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION,
    };
    use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;

    unsafe {
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return None;
        }
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buf = [0u16; 1024];
        let mut len = buf.len() as u32;
        let ok = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0),
            windows::core::PWSTR(buf.as_mut_ptr()),
            &mut len,
        )
        .is_ok();
        let _ = CloseHandle(handle);
        if !ok {
            return None;
        }
        let path = String::from_utf16_lossy(&buf[..len as usize]);
        Path::new(&path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
    }
}
