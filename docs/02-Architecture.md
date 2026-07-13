# Water Me — Architecture（V1）

> 系统架构设计。回答：软件如何组织。

---

# 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                  Desktop Application                     │
│              (Current: Tauri 2)                          │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │              Backend（Rust）                        │  │
│  │                                                    │  │
│  │  ┌──────────────┐  ┌──────────────┐               │  │
│  │  │  Activity     │  │  Reminder    │               │  │
│  │  │  Monitor      │  │  Engine      │               │  │
│  │  │              │  │              │               │  │
│  │  │  Idle        │  │  Rules       │               │  │
│  │  │  Working     │  │  Scheduler   │               │  │
│  │  │  Locked      │  │  State       │               │  │
│  │  │  Sleeping    │  │  History     │               │  │
│  │  └──────┬───────┘  └──────┬───────┘               │  │
│  │         │                  │                        │  │
│  │         └──────┬───────────┘                       │  │
│  │                │                                    │  │
│  │         ┌──────▼───────┐                            │  │
│  │         │  Event Bus   │                            │  │
│  │         └──────┬───────┘                            │  │
│  │                │                                    │  │
│  │  ┌─────────────▼─────────────────────────────────┐ │  │
│  │  │              Store Layer                       │ │  │
│  │  │  Settings Store  |  History Store              │ │  │
│  │  └───────────────────────────────────────────────┘ │  │
│  │                                                     │  │
│  │  ┌───────────────────────────────────────────────┐ │  │
│  │  │           Platform Layer                       │ │  │
│  │  │  Tray  |  AutoStart  |  Fullscreen  |  Hooks  │ │  │
│  │  └───────────────────────────────────────────────┘ │  │
│  └────────────────────────────────────────────────────┘  │
│                            │                              │
│                     Desktop IPC (Events)                  │
│                  (Current: Tauri IPC)                     │
│                            │                              │
│  ┌────────────────────────────────────────────────────┐  │
│  │              Frontend（React）                      │  │
│  │                                                    │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐          │  │
│  │  │ Overlay   │ │ Onboard  │ │ Settings │          │  │
│  │  │ Window    │ │ Window   │ │ Window   │          │  │
│  │  └──────────┘ └──────────┘ └──────────┘          │  │
│  └────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

V1 以桌面平台为目标。当前实现：Windows。其他平台将在架构保持不变的前提下逐步支持。

---

# 2. Module Separation

## 2.1 Backend Modules

```
src-tauri/
├── main.rs                 # App entry, setup
├── lib.rs                  # Module exports
│
├── activity/               # Activity Monitor
│   ├── mod.rs
│   ├── monitor.rs          # ActivityMonitor trait + orchestrator
│   ├── state.rs            # ActivityState enum + transitions
│   └── platform/           # Platform-specific implementation
│       ├── mod.rs           # Re-exports current platform
│       └── windows.rs       # Windows hooks (current)
│       └── (macos.rs)       # (future)
│       └── (linux.rs)       # (future)
│
├── reminder/               # Reminder Engine
│   ├── mod.rs
│   ├── engine.rs           # ReminderEngine (core orchestrator)
│   ├── scheduler.rs        # ReminderScheduler (Working Timer)
│   ├── rule.rs             # Rule evaluation
│   ├── activity.rs         # HealthActivity data model
│   ├── state.rs            # ReminderState enum + state machine
│   └── history.rs          # Activity history recording
│
├── eventbus/               # Event Bus
│   ├── mod.rs
│   └── bus.rs              # In-process event bus
│
├── store/                  # Persistent Storage
│   ├── mod.rs
│   ├── settings.rs         # Settings store + migration
│   └── history.rs          # History store
│
├── platform/               # Platform Layer
│   ├── mod.rs
│   ├── tray.rs             # System tray
│   ├── autostart.rs        # Auto-start on boot
│   ├── fullscreen.rs       # Fullscreen detection (trait + impl)
│   └── platform/           # Platform-specific implementation
│       ├── mod.rs
│       └── windows.rs       # Windows implementation (current)
│       └── (macos.rs)       # (future)
│       └── (linux.rs)       # (future)
│
└── commands/               # Desktop IPC commands
    ├── mod.rs
    ├── reminder.rs          # Reminder-related commands
    └── settings.rs          # Settings-related commands
```

## 2.2 Frontend Modules

```
src/
├── App.tsx
├── main.tsx
│
├── windows/
│   ├── overlay/             # Overlay window
│   │   ├── Overlay.tsx
│   │   └── Overlay.test.tsx
│   ├── onboard/             # Onboarding window
│   │   ├── Onboard.tsx
│   │   ├── Step1Welcome.tsx
│   │   └── Step2Config.tsx
│   └── settings/            # Settings window
│       ├── Settings.tsx
│       └── SettingItem.tsx
│
├── hooks/
│   ├── useReminder.ts       # Listen to reminder events
│   ├── useActivity.ts       # Listen to activity state
│   └── useSettings.ts       # Read/write settings
│
└── lib/
    └── events.ts            # Event type definitions
```

---

# 3. Core Modules

## 3.1 Activity Monitor

### Responsibility

监听用户活动，输出统一的 ActivityState。

### Interface

```
ActivityMonitor
├── start()                    # 开始监听
├── stop()                     # 停止监听
├── on_state_change(callback)  # 状态变化回调
└── get_state() -> ActivityState
```

### State Machine

```
         Mouse/Keyboard
Idle ───────────────────→ Working
  ↑                           │
  │     Idle > threshold      │ Lock/Sleep
  │◄──────────────────────────┤
  │                           │
  │                           ▼
  │                      Paused (Locked/Sleeping)
  │                           │
  │          Unlock/Wake      │
  │◄──────────────────────────┘
  │         (resume from pause point)
  │
  │    Fullscreen detected
  │         ┌──────────┐
  │         │    DND    │ (Working continues, Reminder paused)
  │         └──────────┘
  │              │
  │     Exit fullscreen
  │              │
  └──────────────┘
```

### Platform Adapter

`ActivityMonitor` 定义为 trait，平台实现隔离：

| Platform | Implementation |
|----------|---------------|
| Windows (current) | `WindowsActivityMonitor` |
| macOS (future) | `MacOSActivityMonitor` |
| Linux (future) | `LinuxActivityMonitor` |

测试用 `MockActivityMonitor` 注入任意状态序列。

### Platform API Mapping (Current: Windows)

| Event | Platform API | Detail |
|-------|-------------|--------|
| Mouse move/click | `SetWindowsHookEx(WH_MOUSE_LL)` | Low-level mouse hook, only check timestamp |
| Keyboard press | `SetWindowsHookEx(WH_KEYBOARD_LL)` | Low-level keyboard hook, only check timestamp |
| Session Lock | `WTSRegisterSessionNotification` | `WTS_SESSION_LOCK` / `WTS_SESSION_UNLOCK` |
| Sleep / Wake | `PowerRegisterSuspendResumeNotification` | `PBT_APMSUSPEND` / `PBT_APMRESUMEAUTOMATIC` |

## 3.2 Reminder Engine

### Responsibility

管理提醒规则、触发时机、提醒状态和提醒生命周期。

### Interface

```
ReminderEngine
├── start()                           # 启动引擎
├── stop()                            # 停止引擎
├── on_activity_state_change(state)   # 接收 ActivityState 变化
├── complete(activity_id)             # 用户 Done
├── defer(activity_id)               # 用户 Snooze
├── skip(activity_id)                # 用户 Skip
├── record_manual(activity_id)        # 手动记录（Tray "记录一次喝水"）
├── pause_reminders()                 # 勿扰模式开
├── resume_reminders()                # 勿扰模式关
└── get_pending_reminders() -> Vec    # 获取待处理提醒
```

### Internal Components

```
ReminderEngine
│
├── RuleEvaluator      # 评估 Rule 条件
│   └── rules: Vec<Rule>
│
├── ReminderScheduler  # Working Timer
│   ├── working_duration: Duration
│   ├── timers: HashMap<ActivityId, Timer>
│   └── uses Monotonic Clock
│
├── StateManager       # Reminder 状态机
│   └── states: HashMap<ActivityId, ReminderState>
│
└── HistoryRecorder    # 写入 History
    └── records to History Store
```

### Rule Evaluation Flow

```
ActivityStateChanged(Working)
↓
RuleEvaluator.evaluate(working_duration)
↓
Match Rule condition? → Yes → Check cooldown
↓                         ↓
No                    Cooldown expired?
↓                     ↓         ↓
Continue timing       Yes        No
↓                     ↓          ↓
                  Transition    Wait
                  to Triggered
                      ↓
                  Emit ReminderTriggered event
                  Record to History (status: "Triggered")
```

### State Transitions

```
Pending ──[condition met]──→ Triggered
Triggered ──[Done]──→ Completed + reset timer
Triggered ──[Snooze]──→ Deferred → [after N min] → Triggered
Triggered ──[Skip]──→ Skipped + reset timer
```

Invalid transitions (must be rejected):
- Pending → Completed (skip Triggered)
- Completed → Triggered (already done)
- Skipped → Deferred (already skipped)

### Merge Logic

```
Multiple Rules triggered simultaneously
↓
Group by current time window
↓
Select highest priority Activity as primary
↓
Merge messages
↓
Emit single ReminderTriggered with activities list
↓
User Done → All activities in list → Completed
```

## 3.3 Reminder Scheduler

### Responsibility

基于 Working Time 计时，不是 Clock Timer。See ADR-0001.

### Interface

```
ReminderScheduler
├── start()                       # 开始计时
├── pause()                       # 暂停（Idle/Locked/Sleeping/DND）
├── resume()                      # 恢复（从暂停处继续）
├── get_working_duration() -> Duration
├── reset_timer(activity_id)      # 重置特定 Activity 的计时器
└── update_interval(activity_id, interval)  # 更新间隔（下次生效）
```

### Timer Per Activity

每个 Activity 有独立计时器。See ADR-0002.

### Working Duration vs Activity Timer

Working Duration 是全局累计工作时间，用于 Session 统计。

Activity Timer 是每个 Activity 独立的倒计时，基于 Working Duration 累加。

两者共享同一个 Monotonic Clock，但独立追踪。

## 3.4 Event Bus

### Responsibility

Backend 内部模块间解耦通信。See ADR-0003.

### Design

In-process event bus，基于 `tokio::sync::broadcast`。

```
EventBus
├── publish(event: Event)
├── subscribe(event_type: EventType) -> Receiver<Event>
```

### Internal Events

| Event | Publisher | Subscriber |
|-------|-----------|------------|
| ActivityStateChanged | ActivityMonitor | ReminderEngine |
| ReminderTriggered | ReminderEngine | Frontend (via IPC) |
| ReminderCompleted | ReminderEngine | Frontend, HistoryRecorder |
| ReminderDeferred | ReminderEngine | Frontend, HistoryRecorder |
| ReminderSkipped | ReminderEngine | Frontend, HistoryRecorder |
| SettingsChanged | SettingsStore | Frontend, ReminderEngine |
| FullscreenChanged | FullscreenDetector | ActivityMonitor |

### Event Flow: Full Cycle

```
[1] ActivityMonitor detects mouse/keyboard
    → publishes ActivityStateChanged(Idle → Working)

[2] ReminderEngine receives state change
    → starts/resumes Working Timer

[3] Timer reaches threshold (e.g., 60min)
    → RuleEvaluator matches Rule
    → StateManager transitions Pending → Triggered
    → publishes ReminderTriggered

[4] Frontend receives ReminderTriggered via IPC
    → shows Overlay

[5] User clicks Done
    → Frontend calls IPC command: reminder_complete("water")

[6] ReminderEngine.complete("water")
    → StateManager transitions Triggered → Completed
    → Scheduler resets timer
    → HistoryRecorder writes record
    → publishes ReminderCompleted

[7] Frontend receives ReminderCompleted
    → closes Overlay
```

---

# 4. Storage Layer

## 4.1 Settings Store

### Format

JSON file, managed by Tauri store plugin.

### Schema

```json
{
  "version": 1,
  "water_interval_min": 60,
  "stand_interval_min": 45,
  "idle_threshold_min": 5,
  "snooze_interval_min": 10,
  "snooze_max_count": null,
  "autostart": false,
  "fullscreen_reminder": true
}
```

### Migration

```
load_settings()
↓
read file
↓
parse JSON
↓
check version
↓
version == CURRENT? → use as-is
version < CURRENT?  → run migration → save → use
version > CURRENT?  → error (downgrade not supported)
parse failed?       → use defaults → backup old file → save defaults
```

## 4.2 History Store

### Format

JSONL file（每行一条记录），append-only。See ADR-0004.

### Schema (per line)

```json
{
  "activity": "water",
  "status": "Completed",
  "triggered_at": "2026-07-11T10:30:00Z",
  "responded_at": "2026-07-11T10:30:45Z",
  "working_duration_min": 60
}
```

### Write Strategy

- Append-only: 新记录追加到文件末尾
- 写失败: 静默忽略，记录错误日志，不阻塞主流程
- 不重试: 避免无限循环

---

# 5. Presentation Layer

## 5.1 Window Management

Water Me 没有主窗口。所有 UI 通过独立窗口呈现：

| Window | When | Size | Always on Top |
|--------|------|------|---------------|
| Overlay | Reminder Triggered | Fullscreen (one monitor) | Yes |
| Onboard | First launch | 480×600 | No |
| Settings | User clicks "设置" | 480×600 | No |

### Overlay Window

- 创建时全屏覆盖当前活跃显示器
- 透明背景 + 半透明遮罩
- 关闭时销毁窗口（不是隐藏），下次重新创建。See ADR-0005.
- 不出现在任务栏
- 不响应 Alt+F4

### Onboard Window

- 首次启动创建
- 完成后关闭
- 使用 `first_launch` flag 判断（存在 Settings Store 中）

### Settings Window

- 用户点击 Tray "设置" 时创建
- 如果已打开则 focus
- 关闭时销毁

## 5.2 Desktop IPC

### Backend → Frontend (Events)

| Event | Payload |
|-------|---------|
| `reminder-triggered` | `{ activities: string[], title: string, message: string, duration: number }` |
| `reminder-completed` | `{ activity: string, duration: number }` |
| `reminder-deferred` | `{ activity: string, snooze_until: number }` |
| `reminder-skipped` | `{ activity: string }` |
| `activity-state-changed` | `{ from: string, to: string, working_duration: number }` |
| `settings-changed` | `{ settings: object }` |

### Frontend → Backend (Commands)

| Command | Params | Returns |
|---------|--------|---------|
| `reminder_complete` | `activity_id: string` | `void` |
| `reminder_defer` | `activity_id: string` | `void` |
| `reminder_skip` | `activity_id: string` | `void` |
| `record_manual` | `activity_id: string` | `void` |
| `pause_reminders` | — | `void` |
| `resume_reminders` | — | `void` |
| `get_settings` | — | `Settings` |
| `update_settings` | `settings: Settings` | `void` |
| `get_current_state` | — | `{ activity_state, working_duration, pending_reminders }` |

---

# 6. Platform Layer

系统级功能通过 Platform Adapter 隔离。架构层只定义 trait 和行为，具体实现由各平台提供。

## 6.1 Tray

| Platform | Implementation |
|----------|---------------|
| Windows (current) | System tray, 右下角 |
| macOS (future) | Menu Bar |
| Linux (future) | StatusNotifier |

菜单项：

| Menu Item | Action |
|-----------|--------|
| Water Me (header) | — |
| 暂停提醒 | `pause_reminders()` command |
| 恢复提醒 | `resume_reminders()` command |
| 记录一次喝水 | `record_manual("water")` command |
| 设置 | Open Settings window |
| 退出 | `app.exit(0)` |

暂停/恢复：同一菜单项切换显示。

## 6.2 Auto Start

| Platform | Implementation |
|----------|---------------|
| Windows (current) | Registry `HKEY_CURRENT_USER\...\Run` or Task Scheduler |
| macOS (future) | LaunchAgent |
| Linux (future) | XDG autostart `.desktop` file |

## 6.3 Fullscreen Detection

`FullscreenDetector` trait，平台实现隔离：

| Platform | Implementation |
|----------|---------------|
| Windows (current) | `WindowsFullscreenDetector` |
| macOS (future) | `MacOSFullscreenDetector` |
| Linux (future) | `LinuxFullscreenDetector` |

Current (Windows): Polling every 2s，`GetForegroundWindow` + `GetWindowPlacement` + `GetWindowLong`。

误判容忍：宁可漏判（不触发 DND），不可误判（错误进入 DND）。

---

# 7. Data Flow

## 7.1 Normal Reminder Cycle

```
User types/moves
↓
ActivityMonitor (Platform Adapter)
↓ publish ActivityStateChanged
EventBus
↓
ReminderEngine
↓ timer reaches threshold
ReminderScheduler
↓
RuleEvaluator matches Rule
↓
StateManager: Pending → Triggered
↓ publish ReminderTriggered
EventBus
↓ Desktop IPC
Frontend (Overlay)
↓ user clicks Done
Frontend calls reminder_complete()
↓ Desktop IPC
ReminderEngine.complete()
↓
StateManager: Triggered → Completed
Scheduler.reset_timer()
HistoryRecorder.write()
↓ publish ReminderCompleted
EventBus
↓ Desktop IPC
Frontend (close Overlay)
```

## 7.2 Idle / Resume Cycle

```
No activity for 5min
↓
ActivityMonitor
↓ publish ActivityStateChanged(Working → Idle)
EventBus
↓
ReminderEngine
↓
ReminderScheduler.pause()
(working_duration stops accumulating)
(Activity timers stop advancing)

User moves mouse
↓
ActivityMonitor
↓ publish ActivityStateChanged(Idle → Working)
EventBus
↓
ReminderEngine
↓
ReminderScheduler.resume()
(working_duration continues from pause point)
(Activity timers continue from pause point)
```

## 7.3 Fullscreen DND Cycle

```
Exclusive Fullscreen detected
↓
FullscreenDetector (Platform Adapter)
↓ publish FullscreenChanged(true)
EventBus
↓
ActivityMonitor
↓ publish ActivityStateChanged(→ DND)
EventBus
↓
ReminderEngine
↓
Activity monitoring: continues
Working timer: continues
Reminder triggering: paused
(Queued reminders wait)

Exit fullscreen
↓
FullscreenDetector (Platform Adapter)
↓ publish FullscreenChanged(false)
EventBus
↓
ActivityMonitor
↓ publish ActivityStateChanged(→ Working)
EventBus
↓
ReminderEngine
↓
Check if any reminder should have triggered during DND
If yes → emit ReminderTriggered now
```

---

# 8. Dependencies

## 8.1 Rust (Backend)

| Crate | Purpose |
|-------|---------|
| `tauri` | Application framework (current) |
| `tauri-plugin-store` | Settings persistence |
| `tauri-plugin-autostart` | Boot auto-start |
| `tokio` | Async runtime + broadcast channel |
| `serde` / `serde_json` | Serialization |
| Platform crate (current: `windows`) | Platform-specific API |

## 8.2 Frontend

| Package | Purpose |
|---------|---------|
| `react` | UI framework |
| `@tauri-apps/api` | Desktop IPC bridge (current) |
| `framer-motion` | Overlay animations |

---

# 9. ADR References

| ADR | Title |
|-----|-------|
| ADR-0001 | Working Timer vs Clock Timer |
| ADR-0002 | Per-Activity Timer |
| ADR-0003 | Event Bus |
| ADR-0004 | Storage: JSONL for History |
| ADR-0005 | Overlay: Destroy on Close |
| ADR-0006 | Monotonic Clock |
| ADR-0007 | Health Activity Data Model + Rule System |
