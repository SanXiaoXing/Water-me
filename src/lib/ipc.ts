// Water Me — IPC 类型与封装。对齐 src-tauri/src/{store,reminder,commands}.rs。
// ponytail: 薄封装，不做额外抽象。命令名与 Rust #[tauri::command] 一一对应。

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

// ============ 类型（镜像 Rust 结构）============

/** 用户设置。镜像 store.rs `Settings`。 */
export interface Settings {
  version: number;
  water_interval_min: number;
  stand_interval_min: number;
  idle_threshold_min: number;
  snooze_interval_min: number;
  autostart: boolean;
  fullscreen_reminder: boolean;
  /** 全屏免打扰黑名单（进程 exe basename）。 */
  fullscreen_blocklist: string[];
  /** 主题："system" | "light" | "dark"。 */
  theme: string;
  /** 提醒弹窗形态："fullscreen" | "card" | "toast"。 */
  overlay_mode: OverlayMode;
  first_launch: boolean;
  paused: boolean;
}

/** 主题模式。 */
export type ThemeMode = "system" | "light" | "dark";

/** 提醒弹窗形态。 */
export type OverlayMode = "fullscreen" | "card" | "toast";

/**
 * 应用主题到 document。system 模式跟随系统 prefers-color-scheme。
 * 各窗口挂载时调一次，settings-changed 时再调。
 */
export function applyTheme(theme: string): void {
  const resolved: "light" | "dark" =
    theme === "dark" ? "dark" :
    theme === "light" ? "light" :
    window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  document.documentElement.dataset.theme = resolved;
}

/** 单个提醒活动信息。镜像 reminder.rs `ActivityInfo`。 */
export interface ActivityInfo {
  id: string;
  name: string;
  icon: string;
  message: string;
  message_en: string;
  action: string;
  priority: number;
}

/** reminder-triggered 事件载荷 / Overlay 渲染数据。镜像 reminder.rs `ReminderTriggered`。 */
export interface ReminderTriggered {
  activities: ActivityInfo[];
  title: string;
  title_en: string;
  duration_min: number;
  /** 当前计时段起始时间（上次归零时刻，UTC ISO8601）。 */
  started_at_iso: string;
  /** 本次触发时间（UTC ISO8601）。 */
  triggered_at_iso: string;
}

/** 活动状态字符串："Idle" | "Working"。镜像 activity.rs `ActivityState::as_str`。 */
export type ActivityStateName = "Idle" | "Working";

/** get_current_state 返回。 */
export interface CurrentStatus {
  activity_state: ActivityStateName;
  working_duration_min: number;
}

// ============ 命令封装 ============

export const getSettings = (): Promise<Settings> => invoke("get_settings");

export const updateSettings = (patch: Partial<Settings>): Promise<Settings> =>
  invoke("update_settings", { patch });

export const reminderComplete = (activities: string[]): Promise<void> =>
  invoke("reminder_complete", { activities });

export const reminderDefer = (activities: string[]): Promise<void> =>
  invoke("reminder_defer", { activities });

export const reminderSkip = (activities: string[]): Promise<void> =>
  invoke("reminder_skip", { activities });

export const recordManual = (activity: string): Promise<void> =>
  invoke("record_manual", { activity });

export const pauseReminders = (): Promise<void> => invoke("pause_reminders");

export const resumeReminders = (): Promise<void> => invoke("resume_reminders");

/** Overlay 挂载时拉取当前载荷，避免错过挂载前发出的事件。 */
export const getCurrentReminder = (): Promise<ReminderTriggered | null> =>
  invoke("get_current_reminder");

export const getCurrentState = (): Promise<CurrentStatus> =>
  invoke("get_current_state");

/** Onboard 完成：写入选定设置 + 标记 first_launch=false。 */
export const completeOnboard = (patch: Partial<Settings>): Promise<Settings> =>
  invoke("complete_onboard", { patch });

/** 可见窗口信息（给黑名单选择用）。镜像 fullscreen.rs `WindowInfo`。 */
export interface WindowInfo {
  /** 窗口标题（用户可读）。 */
  title: string;
  /** 进程 exe basename（加入黑名单的实际值）。 */
  process: string;
}

/** 列出当前所有可见、非自身进程的顶层窗口。 */
export const listVisibleWindows = (): Promise<WindowInfo[]> =>
  invoke("list_visible_windows");

// ============ 事件监听 ============

/** 监听 reminder-triggered 事件。返回取消监听函数。 */
export const onReminderTriggered = (
  handler: (payload: ReminderTriggered) => void
): Promise<UnlistenFn> => listen<ReminderTriggered>("reminder-triggered", (e) => handler(e.payload));

export const onSettingsChanged = (
  handler: (settings: Settings) => void
): Promise<UnlistenFn> => listen<Settings>("settings-changed", (e) => handler(e.payload));

export const onReminderCompleted = (
  handler: (activities: string[]) => void
): Promise<UnlistenFn> =>
  listen<{ activities: string[] }>("reminder-completed", (e) => handler(e.payload.activities));

export const onReminderDeferred = (
  handler: (activities: string[]) => void
): Promise<UnlistenFn> =>
  listen<{ activities: string[] }>("reminder-deferred", (e) => handler(e.payload.activities));

export const onReminderSkipped = (
  handler: (activities: string[]) => void
): Promise<UnlistenFn> =>
  listen<{ activities: string[] }>("reminder-skipped", (e) => handler(e.payload.activities));

export const onActivityStateChanged = (
  handler: (payload: { from: ActivityStateName; to: ActivityStateName; working_duration_min: number }) => void
): Promise<UnlistenFn> =>
  listen<{ from: ActivityStateName; to: ActivityStateName; working_duration_min: number }>(
    "activity-state-changed",
    (e) => handler(e.payload)
  );
