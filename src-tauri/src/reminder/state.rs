//! Reminder 状态机。See Architecture §3.2。

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RState {
    Pending,
    Triggered,
    Deferred,
}
