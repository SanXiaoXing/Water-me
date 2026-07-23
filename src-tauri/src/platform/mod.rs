//! Platform Layer。系统级功能通过 Platform Adapter 隔离。
//! See Architecture §6。

pub mod autostart;
pub mod fullscreen;
pub mod tray;

pub use tray::{build_tray, show_onboard, USER_QUIT};
