# Roadmap

| 版本 | 目标 | 开发重点 | 预计代码占比 |
| --- | --- | --- | --- |
| **V1** | 可用的健康提醒工具 | 活动监测 + Activity Engine + Overlay + 托盘 | **60%** |
| **V1.5** | 提升体验 | 配置持久化、动画、开机启动、日志 | **10%** |
| **V2** | 数据价值 | 每日统计、历史记录、连续打卡 | **10%** |
| **V3** | 产品特色 | 桌宠模式、情绪状态、角色系统 | **15%** |
| **V4** | 用户留存 | 成长系统、成就、主题与皮肤 | **5%** |

---

## V1 — 健康提醒

目标：让用户安装后，不需要任何学习，就能开始使用。

不开复杂设置。没有桌宠。没有统计。没有账号。

先把最重要的一条链路跑通。

### 六个模块

```
Water Me
│
├── Activity Monitor    — 活动监测
├── Activity Engine     — 健康行为引擎
├── Notification Window — 提醒窗口（Overlay）
├── Scheduler           — 工作计时器
├── Settings            — 设置
└── Tray                — 系统托盘
```

### Activity Monitor

监控：鼠标、键盘、锁屏、睡眠、唤醒。

输出统一状态：`Idle` / `Working` / `Locked` / `Sleeping`

闲置超过 5 分钟暂停计时，而不是继续倒计时。

### Activity Engine

决定什么时候触发 Health Activity。

```
Working → 60min → Get Water
Working → 45min → Stand Up
```

Health Activity 类型统一：

```rust
enum HealthActivity {
    GetWater,
    StandUp,
    EyeBreak,
}
```

后续可扩展 Walk / Lunch / Medicine / Stretch，不用改架构。

任何 Health Activity 都是插件。

### Notification Window

MVP 唯一需要展示给用户的窗口。自己做的 Overlay，不是系统通知。

永远置顶、带动画、半透明、自动关闭。

```
┌─────────────────────┐
  💧
  Water Me!!
  You've been working
  for 60 minutes.

  [Done]  [Snooze]
└─────────────────────┘
```

用户操作：

- **Done（我喝了）** — 完成
- **Snooze（10 分钟后提醒）** — 延期
- **Skip（今天跳过）** — 放弃

记录的是健康行为，不是机械计时。

### Scheduler

Working Timer，不是 Clock Timer。

```
开始工作 → 15min → 离开 → 暂停 → 回来 → 继续 → 45min → 触发 Activity
```

### Settings

不超过 6 个设置：喝水间隔、站立间隔、闲置暂停时间、开机启动、全屏提醒。

### Tray

系统托盘，右键菜单：暂停提醒、恢复提醒、立即喝水、设置、退出。

### 架构原则

从第一天就按「引擎 + UI」解耦：

```
┌──────────────────────────────┐
│ Activity Engine（纯 Rust）    │
│ • 活动监测                    │
│ • 工作时间统计                │
│ • Health Activity 规则        │
│ • 状态管理                    │
└──────────────┬───────────────┘
               │ Event
               ▼
┌──────────────────────────────┐
│ UI Layer（Tauri + React）     │
│ • Overlay 提醒                │
│ • 设置页面                    │
│ • 托盘                        │
│ • （未来）桌宠                │
└──────────────────────────────┘
```

未来把 Overlay 换成桌宠、增加统计页，甚至适配其他平台时，都不需要改动核心逻辑。

表现层（Presentation）和业务层（Business）严格分离：

```
Business
↓
Activity Engine
↓
Notification
↓
Desktop Pet（以后）
```

Notification 完全可以替换成：

```
Notification
↓
Desktop Pet
↓
Overlay
↓
System Notification
↓
Mobile Push
```

业务逻辑完全不用动。

---

## V1.5 — 提升体验

- 配置持久化
- 动画
- 开机启动
- 日志

---

## V2 — 数据统计

开始增加 Dashboard。

```
Today

喝水：6 次
──────────
站立：4 次
──────────
最长连续工作：2h15m
──────────
总工作：7h40m
```

这些数据 Activity Engine 已经拥有，只是以前没展示。

---

## V3 — 桌宠

整个软件开始拥有 IP。

```
🌱 一直待在右下角。
```

状态：

```
开心        🙂
喝水后      🙂
缺水        🥺
站太久      😭
完成今日目标 🎉
```

提醒变成桌宠表情，而不是普通窗口。

---

## V4 — 成长系统

游戏化。

```
第一天   🌱
第十天   🌿
第三十天 🌳
```

连续三天没喝水，植物开始枯萎 🍂。

用户为了植物，反而开始认真喝水。

---

## V5 — 插件化

Health Activity 不再固定。新增 Medicine / Stretch / Walk / Meditation / Read / Meeting / Pomodoro。

Activity Engine 统一：`Health Activity Plugin → Condition → Interval → Action`

任何 Health Activity 都是插件。

---

## V6 — 跨平台生态

- **macOS**：适配系统状态监听、通知中心和菜单栏应用。
- **Linux**：支持主流桌面环境（GNOME、KDE）的托盘与通知。
- **移动端联动（远期）**：电脑离开后，将提醒同步到手机。
