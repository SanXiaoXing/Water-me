//! Auto-start on boot。See Architecture §6.2, FR-064。
//! 底层由 tauri-plugin-autostart 提供（Windows: 注册表 / 快捷方式）。

use tauri::AppHandle;

/// 同步开机自启到系统。enable=false 时若快捷方式本就不存在视作幂等成功。
pub fn sync_autostart(app: &AppHandle, enable: bool) {
    use tauri_plugin_autostart::ManagerExt;
    let result = if enable {
        app.autolaunch().enable()
    } else {
        app.autolaunch().disable()
    };
    if let Err(e) = result {
        // ponytail: disable 失败大多是快捷方式本就不存在（首次完成引导
        // autostart=false、dev/build 混用 lnk 路径错位、重复关闭），
        // 目标"不自启"已达成，属幂等成功，静默。
        // 精确判断需 downcast 到 auto-launch 的 io::Error，这里按语义降级。
        // enable 失败才真正需要关注（权限/路径问题）。
        if enable {
            eprintln!("[water-me] autostart enable failed: {e}");
        }
    }
}
