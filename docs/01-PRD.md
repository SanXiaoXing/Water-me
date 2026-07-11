# Water Me — PRD（V1 MVP）

> **Humans need watering, too.**

---

# 1. MVP Scope

V1 目标：**让用户安装后，不需要任何学习，就能开始使用。**

不开复杂设置。没有桌宠。没有统计。没有账号。

先把最重要的一条链路跑通：

```
启动 → 监听用户活动 → 累计工作时间 → 达到提醒阈值 → 弹出 Overlay → 用户响应 → 重新计时
```

## 目标平台

**Windows 优先。** V1 不做跨平台。

先把 Windows 做好。很多独立开发者最大的问题就是 Windows / macOS / Linux 一起做，最后三个都不好。

## 核心模块

```
Water Me
│
├── Activity Monitor    — 活动监测
├── Reminder Engine     — 提醒引擎
├── Reminder Scheduler  — 提醒调度器
├── Notification Window — 提醒窗口（Overlay）
├── Settings            — 设置
└── Tray                — 系统托盘
```

---

# 2. Product Boundary

Water Me **不是**：

- 医疗软件
- 健康诊断软件
- 时间管理软件
- 番茄钟
- 效率工具
- 任务管理器

Water Me 的唯一目标：**帮助用户建立健康工作习惯。**

以后任何需求超出这个边界，直接拒绝。例如：能不能加 Todo？→ Product Boundary。结束。

---

# 3. User Stories

## 核心流程

| # | As a... | I want to... | So that... |
|---|---------|-------------|------------|
| US-01 | 用户 | 安装后无需配置就能开始使用 | 零学习成本 |
| US-02 | 用户 | 只在我真正工作时计时 | 不会被摸鱼时间污染 |
| US-03 | 用户 | 工作太久时收到温柔提醒 | 不会忘记照顾自己 |
| US-04 | 用户 | 选择"我完成了"或"稍后提醒"或"跳过" | 我有控制权 |
| US-05 | 用户 | 手动记录一次喝水 | 不等提醒也能记录 |

## 活动监测

| # | As a... | I want to... | So that... |
|---|---------|-------------|------------|
| US-06 | 用户 | 离开电脑后计时暂停 | 不会因为不在电脑前而被"虚假计时" |
| US-07 | 用户 | 锁屏/睡眠时计时暂停 | 数据准确 |
| US-08 | 用户 | 全屏应用时不被打扰 | 游戏和演示不会被中断 |

## 提醒交互

| # | As a... | I want to... | So that... |
|---|---------|-------------|------------|
| US-09 | 用户 | 多个提醒合并为一个 | 不会被连续轰炸 |
| US-10 | 用户 | 提醒只在 Done/Snooze/Skip 时关闭 | 不会被忽略 |
| US-11 | 用户 | 选择 Snooze 后过一会儿再提醒 | 正忙的时候可以延后 |

## 系统

| # | As a... | I want to... | So that... |
|---|---------|-------------|------------|
| US-12 | 用户 | 开机自启 | 不用每次手动打开 |
| US-13 | 用户 | 通过托盘控制 | 不需要主窗口 |
| US-14 | 用户 | 暂停提醒但继续监测 | 开会时不被打扰但数据不丢 |
| US-15 | 用户 | 知道软件不会侵犯我的隐私 | 放心使用 |

---

# 4. Functional Requirements

## 4.1 Activity Monitor（活动监测）

### 监听范围

- 鼠标：检测移动和点击（不记录坐标）
- 键盘：检测按键事件（不记录按键内容）
- 锁屏：Windows Session Lock
- 睡眠：系统 Sleep
- 唤醒：系统 Wake

### 输出状态

```
Idle     — 无鼠标键盘活动
Working  — 有鼠标键盘活动
Locked   — 屏幕锁定
Sleeping — 系统休眠
```

### Working Session 定义

```
Working
↓
检测到 Mouse / Keyboard 活动
↓
Idle > 阈值
↓
End Session
```

Session 是统计数据的基本单位。以后统计最长工作时间、连续工作，都会基于 Session。

### Idle 阈值

- 默认 5 分钟无鼠标键盘活动 → Idle
- 用户可在 Settings 中调整

### 全屏应用勿扰

V1 不识别软件类型（不区分 VSCode / YouTube / Steam）。

只判断窗口状态：

```
Exclusive Fullscreen → 自动进入勿扰
Presentation Mode    → 自动进入勿扰
```

勿扰期间：Activity 继续监测，Working 继续计时，Reminder 暂停弹出。

V2 再加入 App Detection。See ADR-0003。

### 锁屏 / 睡眠处理

- 锁屏 = 暂停计时（人不在电脑前）
- 唤醒 / 解锁 = 恢复计时，从暂停处继续（累加制）
- 锁屏期间不补偿——你确实没在工作

### 隐私约束（硬性要求）

- **只监听活动状态，不记录内容**
- 鼠标：只检测"有没有动"，不记录坐标
- 键盘：只检测"有没有按键"，不记录按键内容
- 不读取文件
- 不读取聊天内容
- 不联网
- 不上传任何数据

### 时间源

所有 Working Duration 使用 **Monotonic Clock**，不受系统时间修改影响。

See ADR-0006。

## 4.2 Reminder Engine（提醒引擎）

Reminder Engine 管理提醒规则、触发时机、提醒状态和提醒生命周期。

Health Activity 是它管理的对象，不是引擎本身。

### Health Activity 数据模型

每个 Health Activity 由以下属性定义：

| 属性 | 说明 | 示例 |
|------|------|------|
| id | 唯一标识 | "water" |
| name | 显示名称 | "喝水" |
| interval | 触发间隔（分钟） | 60 |
| priority | 优先级（数值越小越高） | 1 |
| icon | 图标 | 💧 |
| message | 提醒文案 | "去给自己接一杯水。" |
| action | 用户动作标签 | "我喝了" |

V1 预置两个 Activity：

| id | name | interval | priority | icon | message | action |
|----|------|----------|----------|------|---------|--------|
| water | 喝水 | 60 | 1 | 💧 | 去给自己接一杯水。 | 我喝了 |
| stand | 站立 | 45 | 2 | 🧍 | 站起来活动一下。 | 我站了 |

以后新增 Activity 只需增加一条数据，不需要改引擎。V5 插件化时直接就是 HealthActivity 数据。See ADR-0007。

### Reminder Rule

Engine 通过 Rule 决定何时触发 Activity：

```
Rule
├── condition    — 触发条件（例如：Working >= 60min）
├── activity     — 触发的 Health Activity
└── cooldown     — 冷却时间
```

V1 预置两条 Rule：

| condition | activity | cooldown |
|-----------|----------|----------|
| Working >= 60min | water | 10s |
| Working >= 45min | stand | 10s |

以后可扩展基于时间（Lunch Time）、基于累计（Continuous Working）等 Rule 类型。Engine 不用改。See ADR-0007。

### Reminder 状态

Engine 内部维护 5 个状态：

```
Pending    — 等待触发（计时中）
Triggered  — 已触发，等待用户响应
Completed  — 用户完成
Deferred   — 用户延后（Snooze）
Skipped    — 用户跳过（Skip）
```

UI 只有 3 个按钮：Done / Snooze / Skip。

映射关系：

| UI 按钮 | Engine 状态 | 计时器行为 |
|---------|-----------|----------|
| Done    | Completed | 重置计时器 |
| Snooze  | Deferred  | N 分钟后再次触发，不重置计时器 |
| Skip    | Skipped   | 重置计时器 |

### 提醒优先级（Reminder Priority）

当多个 Activity 同时到期时：

```
合并为一个 Overlay
↓
合并文案（例如："去接一杯水，顺便站起来活动一下。"）
↓
用户点 Done → 所有到期 Activity 一起标记 Completed
```

优先级按 Activity 的 priority 字段排序。V1 写死。V5 插件化时再开放配置。

### 提醒冷却（Reminder Cooldown）

用户点击 Done 后，至少 **10 秒** 内不弹第二个 Overlay。

避免连续弹窗的奇怪体验。

### 提醒合并细节

```
water 到期 + stand 到期
↓
一个 Overlay
↓
文案："去接一杯水，顺便站起来活动一下。"
↓
Done → 两个 Activity 都 Completed
```

## 4.3 Notification Window（Overlay）

### 显示行为

- **全屏覆盖**当前活跃显示器（鼠标最后所在的屏幕）
- 永远置顶
- 半透明背景
- 带动画（弹出 / 关闭）
- **只在 Done / Snooze / Skip 时关闭**，不自动关闭，不响应 ESC 或点击外部关闭

### Overlay 内容

```
💧

Water Me!!

You've been working for 60 minutes.

[Done]          [Snooze]          [Skip]
```

多 Activity 合并时：

```
💧

去接一杯水，顺便站起来活动一下。

[Done]          [Snooze]          [Skip]
```

### 多显示器

Overlay 显示在当前活跃显示器（鼠标最后所在屏幕）。桌宠以后也用这个逻辑。

## 4.4 Reminder Scheduler（提醒调度器）

Working Timer，不是 Clock Timer。See ADR-0002。

```
开始工作 → 15min → 离开 → 暂停 → 回来 → 继续 → 45min → 触发 Reminder
```

只有 Working 状态累加时间。Idle / Locked / Sleeping 暂停。

## 4.5 Settings（设置）

### 设置项

| 设置项 | 类型 | 默认值 | 范围 |
|-------|------|-------|------|
| 喝水间隔 | 数字 | 60 min | 15–120 min |
| 站立间隔 | 数字 | 45 min | 15–120 min |
| 闲置暂停阈值 | 数字 | 5 min | 1–30 min |
| Snooze 间隔 | 数字 | 10 min | 5–30 min |
| Snooze 上限 | 数字 | 无限 | 1–无限 |
| 开机启动 | 开关 | OFF | — |
| 全屏提醒 | 开关 | ON | — |

不超过 7 个设置。

### 持久化

设置使用 Persistent Storage 存储，支持配置版本 Migration。See Architecture-02。

### 配置版本

Settings 必须包含 `version` 字段。版本升级时自动执行 Migration，避免 Settings 越来越难维护。

## 4.6 Tray（系统托盘）

### 菜单

```
Water Me
────────────
暂停提醒
恢复提醒
────────────
记录一次喝水
────────────
设置
退出
```

### "记录一次喝水"行为

用户点击 = 记录一次 Completed + 重置 GetWater 计时器。

语义是"记录"，不是"提醒"。

### 暂停提醒（勿扰模式）

```
Activity   → 继续（监测不停）
Working    → 继续（计时不停）
Reminder   → 暂停（不弹 Overlay）
```

恢复后从暂停处继续。

## 4.7 首次启动引导

### 两步引导

**第一页：理念 + 隐私声明**

```
Water Me 🌱

Humans need watering, too.

我们只监测你是否在使用电脑。
不会读取文件。
不会记录键盘内容。
不会上传任何数据。
不会联网。
```

**第二页：默认配置 + 开机启动**

```
默认设置

喝水提醒    60 分钟
站立提醒    45 分钟
闲置暂停    5 分钟

☐ 开机时自动启动

[开始使用]
```

用户可直接点"开始使用"，或调整默认值。

首次启动后，之后静默启动，不再显示引导。

## 4.8 Activity History（行为记录）

V1 即记录，为 V2 统计铺路。存储使用 Persistent Storage。See Architecture-02。

### 记录字段

| 字段 | 说明 |
|------|------|
| activity | Health Activity id |
| status | Completed / Deferred / Skipped / Triggered |
| triggered_at | 触发时间（UTC） |
| responded_at | 用户响应时间（UTC） |
| working_duration_min | 触发时累计工作时长 |

### 状态映射

| Engine 状态 | 记录值 |
|------------|-------|
| Completed  | "Completed" |
| Deferred   | "Deferred" |
| Skipped    | "Skipped" |
| 触发但无响应（用户关机等） | "Triggered" |

## 4.9 Event Contract

Reminder Engine 向 UI 层发布统一事件。事件格式语言无关，具体实现见 Tech Design。

### 事件列表

| 事件 | 说明 | 核心字段 |
|------|------|---------|
| ReminderTriggered | 提醒触发 | activities, title, message, duration |
| ReminderCompleted | 用户完成 | activity, duration |
| ReminderDeferred | 用户延后 | activity, snooze_until |
| ReminderSkipped | 用户跳过 | activity |
| ActivityStateChanged | 活动状态变化 | from, to, working_duration |
| SettingsChanged | 设置变更 | settings (full snapshot) |

See Tech Design for struct definitions.

---

# 5. State Machine

## 系统级状态

```
Application
│
├── Idle ──────────── 检测到鼠标/键盘活动 ──→ Working
│                                              │
│                     Idle > 阈值 ←────────────┘
│                         │
├── Working ──────────────┘ 累计工作时间
│     │
│     ├── 达到提醒阈值 ──→ Reminder Triggered
│     │
│     ├── 锁屏/睡眠 ────→ Paused
│     │
│     └── 全屏应用 ─────→ DND (Working)
│
├── Paused ─────────── 解锁/唤醒 ──→ Working（从暂停处继续）
│
├── DND ────────────── 退出全屏 ──→ Working
│     │                     │
│     │ Activity 继续       │ Reminder 暂停
│     │ Working 继续        │
│     └─────────────────────┘
│
└── Reminder Triggered ──→ Waiting User
                              │
                              ├── Done ──→ Working（重置计时器）
                              ├── Snooze → Working（N分钟后再触发）
                              └── Skip ──→ Working（重置计时器）
```

## Reminder 状态

```
Pending ──→ Triggered ──→ Completed
                  │
                  ├──→ Deferred ──→ Triggered（Snooze 后再触发）
                  │
                  └──→ Skipped
```

代码实现应基于 State Machine，不是 if/else。See Architecture-02。

---

# 6. Edge Cases（异常流程）

| 场景 | 行为 |
|------|------|
| 用户关机 | 已 Triggered 的 Reminder 写入 History（status: "Triggered"），下次启动重新计时 |
| 用户注销 | 同锁屏处理：Pause，重新登录后 Resume |
| 修改系统时间 | 使用 Monotonic Clock，无影响 |
| 修改时区 | 使用 Monotonic Clock，无影响；History 时间戳用 UTC |
| 睡眠 8 小时 | 不补偿 Reminder，唤醒后从暂停处继续累加 |
| Settings 修改间隔 | 当前正在进行的 Timer **不重置**，下次触发后使用新间隔 |
| Store 文件损坏 | 恢复默认设置，保留损坏文件为备份，提示用户 |
| History 写失败 | 静默忽略，不阻塞主流程，记录错误日志 |
| Overlay 弹出时用户锁屏 | Overlay 保持显示，解锁后仍在（等待用户响应） |
| 网络断开 | 无影响（不联网） |

---

# 7. Error Recovery（错误恢复）

## Settings 文件损坏

```
Settings 文件损坏
↓
加载默认设置
↓
损坏文件保留为备份
↓
用户下次打开 Settings 时提示"设置已恢复为默认值"
```

## History 写失败

```
History 写入失败
↓
静默忽略（不阻塞主流程）
↓
写入错误日志（文件路径 + 错误信息）
↓
不重试（避免无限循环）
```

## 进程异常退出

```
进程崩溃 / 被杀
↓
下次启动检测未完成的 Triggered 状态
↓
写入 History（status: "Triggered"）
↓
重新开始计时（不补偿）
```

---

# 8. Non-functional Requirements

## 8.1 性能指标

| 指标 | 目标 |
|------|------|
| 内存常驻 | ≤ 50MB |
| CPU（Idle） | ≤ 0.5% |
| CPU（提醒时） | ≤ 2% |
| 冷启动 | ≤ 2s 可交互 |
| Overlay 弹出 | ≤ 100ms |
| Store 读写 | ≤ 10ms |

## 8.2 质量指标

| 指标 | 目标 |
|------|------|
| Reminder Miss Rate | < 1%（应该触发但未触发的比例） |
| Activity Detection Accuracy | > 99%（正确识别 Working/Idle 的比例） |
| State Transition Correctness | 100%（状态转换不允许非法跳转） |

## 8.3 可测试性

Reminder Engine 与 UI 解耦，核心逻辑可单元测试。

Activity Monitor 可注入 Mock 数据，不依赖真实硬件。

## 8.4 架构约束

业务逻辑与表现层严格分离。所有模块必须支持：未来 Desktop Pet / Plugin / Dashboard / Mobile 替换 Presentation Layer，无需修改 Reminder Engine。See Architecture-02。

## 8.5 隐私

- 不联网
- 不上传数据
- 不记录键盘内容
- 不记录鼠标坐标
- 只监测活动状态（有没有在用电脑）

此声明应在官网第一页和首次引导中明确展示。

---

# 9. Success Metrics（V1 成功标准）

V1 不是"代码写完"就算完成，而是满足以下标准：

### 用户体验

- 用户无需阅读文档即可开始使用
- 用户一天至少响应一次 Reminder（Done 或 Snooze）

### 软件质量

- 提醒成功率 > 95%（软件运行稳定，不崩溃）
- Reminder Miss Rate < 1%
- Activity Detection Accuracy > 99%
- 软件一周稳定运行（无内存泄漏、无崩溃）

### 资源占用

- 内存 < 50MB
- CPU Idle < 0.5%

每个版本都有 Definition of Done。

---

# 10. Known Risks

| 风险 | 影响 | 缓解策略 |
|------|------|---------|
| Windows API 版本兼容 | 不同 Windows 版本的 Activity Detection API 行为可能不一致 | 明确支持 Windows 10 21H2+，降级路径测试 |
| 全屏判断误判 | 某些应用的全屏状态可能无法准确检测 | 记录日志，V2 加入 App Detection 兜底 |
| 睡眠恢复事件丢失 | 部分机器/驱动睡眠唤醒后事件丢失 | Monotonic Clock 做兜底检测（定期校验） |
| 企业电脑权限限制 | 某些企业策略限制键盘监听 | 引导页说明所需权限，检测失败时降级为仅鼠标监听 |
| 杀毒软件误报 | 全局键盘监听可能被标记为 keylogger | 隐私声明 + 引导页说明 + 申请白名单 |
| Tauri WebView 兼容性 | WebView2 运行时版本差异 | 打包时嵌入 WebView2 Bootstrapper |

---

# 11. Assumptions

| 假设 | 如果失效 |
|------|---------|
| 用户每天使用电脑 | 需要考虑"隔天"场景：Timer 状态是否跨天 |
| 用户有喝水条件（有水杯/饮水机） | 某些场景下无法立即喝水，Snooze 提供缓冲 |
| 用户默认接受全屏 Overlay | 未来可能需要提供非全屏提醒选项 |
| 用户运行 Windows 10 21H2+ | 旧版本需要降级方案或放弃支持 |
| 单用户使用（非多人共用电脑） | 多用户场景需要按用户隔离 Settings 和 History |
| 用户不会频繁修改系统时间 | Monotonic Clock 兜底 |

---

# 12. Out of Scope（V1 不做）

| 功能 | 原因 | 计划版本 |
|------|------|---------|
| 跨平台（macOS / Linux） | 先做好 Windows | V6 |
| App Detection（识别软件类型） | V1 不需要，全屏勿扰足够 | V2 |
| 桌宠 | 属于表现层，V1 先做 Overlay | V3 |
| 数据统计 / Dashboard | 数据先记，V2 再展示 | V2 |
| 成长系统 / 游戏化 | 不是 V1 核心链路 | V4 |
| 账号 / 云同步 | 本地优先 | 远期 |
| 插件系统 | V1 只有 water 和 stand | V5 |
| 自定义 Health Activity | 插件系统的前提 | V5 |
| 排行榜 | 违反 Respect the User 原则 | 不做 |
| 惩罚机制 | 违反产品人格 | 不做 |

---

# 13. Glossary

| 术语 | 定义 |
|------|------|
| Activity Monitor | 活动监测模块，监听鼠标/键盘/锁屏/睡眠/唤醒事件 |
| Working | 用户正在使用电脑（有鼠标/键盘活动）的状态 |
| Idle | 用户未使用电脑（无鼠标/键盘活动超过阈值）的状态 |
| Locked | 屏幕锁定状态 |
| Sleeping | 系统休眠状态 |
| DND | Do Not Disturb，勿扰模式，Reminder 暂停弹出 |
| Session | 一次连续 Working 的时间段，从检测到活动开始，到 Idle 超过阈值结束 |
| Health Activity | 健康行为，如喝水、站立，由数据模型定义（id, name, interval, priority...） |
| Reminder Engine | 提醒引擎，管理 Rule、Activity 状态和提醒生命周期 |
| Reminder Rule | 提醒规则，定义触发条件（condition）和对应的 Activity |
| Reminder Scheduler | 提醒调度器，基于 Working Time 计时，不是 Clock Timer |
| Overlay | 全屏半透明提醒窗口 |
| Pending | Reminder 状态：等待触发（计时中） |
| Triggered | Reminder 状态：已触发，等待用户响应 |
| Completed | Reminder 状态：用户完成 |
| Deferred | Reminder 状态：用户延后（Snooze） |
| Skipped | Reminder 状态：用户跳过 |
| Done | UI 按钮，映射到 Completed |
| Snooze | UI 按钮，映射到 Deferred，N 分钟后再次触发 |
| Skip | UI 按钮，映射到 Skipped |
| Monotonic Clock | 单调时钟，不受系统时间修改影响 |
| Persistent Storage | 持久化存储，具体实现见 Architecture |
