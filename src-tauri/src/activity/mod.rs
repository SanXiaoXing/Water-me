//! Activity Monitor。监听用户活动，输出统一状态。See Architecture §3.1。
//!
//! ponytail: V1 用 `GetLastInputInfo` 轮询（2s）而非全局钩子。
//! 一次 API 调用即可拿到"最后一次输入的时刻"，只检测"有没有动"，
//! 不记录坐标/按键内容，天然满足隐私约束（FR-024~029）。
//! 锁屏 / 睡眠时无输入 → 自然进入 Idle → 计时暂停，行为正确（FR-020/021）。
//! 升级路径：WTSRegisterSessionNotification + PowerRegisterSuspendResumeNotification
//! 以区分 Locked / Sleeping 状态标签。

pub mod state;

pub use state::{poll_state, ActivityState};
