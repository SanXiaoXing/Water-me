//! System Tray + 窗口管理。See Architecture §5.1, §6.1, UIUX §5.4, PRD §4.6。
//!
//! 无主窗口架构下，托盘是常驻入口。Onboard/Settings 窗口由此处创建。

use std::sync::atomic::AtomicBool;

use tauri::{
    image::Image,
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder,
};

use crate::reminder::{self, SharedEngine};
use crate::store::{HistoryStore, SettingsStore};

/// 托盘"暂停/恢复"菜单项句柄，用于切换文案。
pub struct PauseToggle(MenuItem<tauri::Wry>);

/// 用户主动退出标志。托盘"退出"设为 true，ExitRequested 据此放行。
/// 无主窗口架构下，窗口关闭默认会触发退出，需阻止；只有主动退出才放行。
pub static USER_QUIT: AtomicBool = AtomicBool::new(false);

/// 构建系统托盘 + 菜单。See UIUX §5.4, PRD §4.6。
pub fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let header = MenuItem::with_id(app, "wm-header", "🌱 Water Me", false, None::<&str>)?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let toggle = MenuItem::with_id(app, "wm-toggle-pause", "暂停提醒", true, None::<&str>)?;
    let water = MenuItem::with_id(app, "wm-water", "立即喝水 💧", true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let settings_item = MenuItem::with_id(app, "wm-settings", "设置", true, None::<&str>)?;
    let sep3 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "wm-quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[&header, &sep1, &toggle, &water, &sep2, &settings_item, &sep3, &quit],
    )?;

    // 托盘图标：复用应用默认窗口图标（来自 tauri.conf.json bundle.icon）。
    let icon = app
        .default_window_icon()
        .cloned()
        .unwrap_or_else(|| Image::new_owned(vec![0, 0, 0, 0], 1, 1));

    app.manage(PauseToggle(toggle));

    TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .tooltip("Water Me")
        .menu(&menu)
        .on_menu_event(on_menu_event)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_settings(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    use std::sync::atomic::Ordering;
    match event.id().as_ref() {
        "wm-toggle-pause" => {
            let store = app.state::<std::sync::Arc<SettingsStore>>();
            let was_paused = store.get().paused;
            let updated = store.update(&serde_json::json!({ "paused": !was_paused }));
            let _ = app.emit("settings-changed", &updated);
            // 切换菜单文案。
            if let Some(t) = app.try_state::<PauseToggle>() {
                let _ = t.0.set_text(if !was_paused { "恢复提醒" } else { "暂停提醒" });
            }
        }
        "wm-water" => {
            let history = app.state::<std::sync::Arc<HistoryStore>>();
            let engine = app.state::<SharedEngine>();
            reminder::record_manual(app, history.inner(), engine.inner(), "water");
        }
        "wm-settings" => show_settings(app),
        "wm-quit" => {
            // 标记主动退出，放行 ExitRequested。
            USER_QUIT.store(true, Ordering::SeqCst);
            app.exit(0);
        }
        _ => {}
    }
}

/// 打开 / 聚焦 Settings 窗口（480×640）。See UIUX §5.3。
pub fn show_settings(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("settings") {
        let _ = w.set_focus();
        return;
    }
    let _ = WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("index.html".into()))
        .title("Water Me · 设置")
        .inner_size(480.0, 640.0)
        .resizable(false)
        .center()
        .build();
}

/// 打开 Onboard 窗口（480×640）。See UIUX §5.2。
pub fn show_onboard(app: &AppHandle) {
    let _ = WebviewWindowBuilder::new(app, "onboard", WebviewUrl::App("index.html".into()))
        .title("Water Me")
        .inner_size(480.0, 640.0)
        .resizable(false)
        .center()
        .build();
}
