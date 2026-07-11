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

**Windows 10 21H2+**。V1 不做跨平台。See ADR-0001。

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

## Feature Lifecycle

每个功能从想法到上线经历以下阶段：

```
Idea → Draft → Approved → In Development → Implemented → Released
```

V1 功能状态：

| 功能 | Status |
|------|--------|
| Activity Monitor | Approved |
| Reminder Engine | Approved |
| Reminder Scheduler | Approved |
| Overlay | Approved |
| Settings | Approved |
| Tray | Approved |
| Onboarding | Approved |
| Activity History | Approved |
| Desktop Pet | Idea |
| Plugin System | Idea |
| Dashboard | Idea |

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

| # | As a... | I want to... | So that... | Acceptance Criteria |
|---|---------|-------------|------------|---------------------|
| US-01 | 用户 | 安装后无需配置就能开始使用 | 零学习成本 | ✓ 首次启动显示引导页 ✓ 使用默认配置即可正常工作 |
| US-02 | 用户 | 只在我真正工作时计时 | 不会被摸鱼时间污染 | ✓ Idle 超过阈值暂停计时 ✓ 回来继续累计 ✓ Timer 不重置 |
| US-03 | 用户 | 工作太久时收到温柔提醒 | 不会忘记照顾自己 | ✓ 工作达到阈值弹出 Overlay ✓ Overlay 显示工作时长 ✓ 文案符合产品人格 |
| US-04 | 用户 | 选择"我完成了"或"稍后提醒"或"跳过" | 我有控制权 | ✓ Done 记录完成并重置 ✓ Snooze N 分钟后再提醒 ✓ Skip 跳过并重置 |
| US-05 | 用户 | 手动记录一次喝水 | 不等提醒也能记录 | ✓ Tray 菜单有"记录一次喝水" ✓ 记录 Completed + 重置计时器 |

## 活动监测

| # | As a... | I want to... | So that... | Acceptance Criteria |
|---|---------|-------------|------------|---------------------|
| US-06 | 用户 | 离开电脑后计时暂停 | 不会被虚假计时 | ✓ Idle > 阈值 → 暂停 ✓ 回来 → 从暂停处继续 |
| US-07 | 用户 | 锁屏/睡眠时计时暂停 | 数据准确 | ✓ 锁屏 → 暂停 ✓ 唤醒 → 从暂停处继续 ✓ 不补偿 |
| US-08 | 用户 | 全屏应用时不被打扰 | 游戏和演示不会被中断 | ✓ Exclusive Fullscreen → DND ✓ Presentation Mode → DND ✓ Activity 和 Working 继续 |

## 提醒交互

| # | As a... | I want to... | So that... | Acceptance Criteria |
|---|---------|-------------|------------|---------------------|
| US-09 | 用户 | 多个提醒合并为一个 | 不会被连续轰炸 | ✓ 同时到期 → 一个 Overlay ✓ 文案合并 ✓ Done 全部标记完成 |
| US-10 | 用户 | 提醒只在 Done/Snooze/Skip 时关闭 | 不会被忽略 | ✓ 不自动关闭 ✓ 不响应 ESC ✓ 不响应点击外部 |
| US-11 | 用户 | 选择 Snooze 后过一会儿再提醒 | 正忙的时候可以延后 | ✓ N 分钟后再次弹出 ✓ 不重置计时器 |

## 系统

| # | As a... | I want to... | So that... | Acceptance Criteria |
|---|---------|-------------|------------|---------------------|
| US-12 | 用户 | 开机自启 | 不用每次手动打开 | ✓ 引导页可勾选 ✓ Settings 可开关 |
| US-13 | 用户 | 通过托盘控制 | 不需要主窗口 | ✓ 托盘常驻 ✓ 菜单功能完整 |
| US-14 | 用户 | 暂停提醒但继续监测 | 开会时不被打扰但数据不丢 | ✓ Activity 继续 ✓ Working 继续 ✓ Reminder 暂停 |
| US-15 | 用户 | 知道软件不会侵犯我的隐私 | 放心使用 | ✓ 引导页显示隐私声明 ✓ 官网显示隐私声明 |

---

# 4. Functional Requirements

## 4.1 Activity Monitor（活动监测）

### 监听范围

| ID | Requirement | Priority |
|----|------------|----------|
| FR-001 | 监听鼠标移动和点击，不记录坐标 | MUST |
| FR-002 | 监听键盘按键事件，不记录按键内容 | MUST |
| FR-003 | 监听 Windows Session Lock 事件 | MUST |
| FR-004 | 监听系统 Sleep / Wake 事件 | MUST |

### 输出状态

| ID | Requirement | Priority |
|----|------------|----------|
| FR-005 | 输出 Idle 状态（无鼠标键盘活动） | MUST |
| FR-006 | 输出 Working 状态（有鼠标键盘活动） | MUST |
| FR-007 | 输出 Locked 状态（屏幕锁定） | MUST |
| FR-008 | 输出 Sleeping 状态（系统休眠） | MUST |

### Working Session

| ID | Requirement | Priority |
|----|------------|----------|
| FR-009 | 检测到鼠标/键盘活动时开始 Session | MUST |
| FR-010 | Idle 超过阈值时结束 Session | MUST |
| FR-011 | Session 是统计数据的基本单位 | MUST |

### Idle 阈值

| ID | Requirement | Priority |
|----|------------|----------|
| FR-012 | 默认 5 分钟无鼠标键盘活动 → Idle | MUST |
| FR-013 | 用户可在 Settings 中调整 Idle 阈值 | SHOULD |

### 全屏应用勿扰

| ID | Requirement | Priority |
|----|------------|----------|
| FR-014 | Exclusive Fullscreen → 自动进入 DND | MUST |
| FR-015 | Presentation Mode → 自动进入 DND | SHOULD |
| FR-016 | DND 期间 Activity 继续监测 | MUST |
| FR-017 | DND 期间 Working 继续计时 | MUST |
| FR-018 | DND 期间 Reminder 暂停弹出 | MUST |
| FR-019 | 退出全屏 → 恢复正常 | MUST |

V1 不识别软件类型。V2 再加入 App Detection。See ADR-0003。

### 锁屏 / 睡眠处理

| ID | Requirement | Priority |
|----|------------|----------|
| FR-020 | 锁屏 → 暂停计时 | MUST |
| FR-021 | 睡眠 → 暂停计时 | MUST |
| FR-022 | 唤醒/解锁 → 恢复计时，从暂停处继续（累加制） | MUST |
| FR-023 | 锁屏/睡眠期间不补偿 | MUST |

### 隐私约束

| ID | Requirement | Priority |
|----|------------|----------|
| FR-024 | 只监听活动状态，不记录内容 | MUST |
| FR-025 | 鼠标只检测"有没有动"，不记录坐标 | MUST |
| FR-026 | 键盘只检测"有没有按键"，不记录按键内容 | MUST |
| FR-027 | 不读取文件 | MUST |
| FR-028 | 不联网 | MUST |
| FR-029 | 不上传任何数据 | MUST |

### 时间源

| ID | Requirement | Priority |
|----|------------|----------|
| FR-030 | 所有 Working Duration 使用 Monotonic Clock，不受系统时间修改影响 | MUST |

See ADR-0006。

## 4.2 Reminder Engine（提醒引擎）

Reminder Engine 管理提醒规则、触发时机、提醒状态和提醒生命周期。

Health Activity 是它管理的对象，不是引擎本身。

### Health Activity 数据模型

| ID | Requirement | Priority |
|----|------------|----------|
| FR-031 | Health Activity 由数据模型定义（id, name, interval, priority, icon, message, action） | MUST |
| FR-032 | V1 预置 water 和 stand 两个 Activity | MUST |
| FR-033 | 新增 Activity 只需增加数据，不需要改引擎 | SHOULD |

V1 预置 Activity：

| id | name | interval | priority | icon | message | action |
|----|------|----------|----------|------|---------|--------|
| water | 喝水 | 60 | 1 | 💧 | 去给自己接一杯水。 | 我喝了 |
| stand | 站立 | 45 | 2 | 🧍 | 站起来活动一下。 | 我站了 |

See ADR-0007。

### Reminder Rule

| ID | Requirement | Priority |
|----|------------|----------|
| FR-034 | Engine 通过 Rule 决定何时触发 Activity（condition + activity + cooldown） | MUST |
| FR-035 | V1 支持 Working Duration 类型的 condition | MUST |
| FR-036 | 以后可扩展基于时间、基于累计等 Rule 类型，Engine 不用改 | MAY |

V1 预置 Rule：

| condition | activity | cooldown |
|-----------|----------|----------|
| Working >= 60min | water | 10s |
| Working >= 45min | stand | 10s |

See ADR-0007。

### Reminder 状态

| ID | Requirement | Priority |
|----|------------|----------|
| FR-037 | Engine 维护 5 个状态：Pending / Triggered / Completed / Deferred / Skipped | MUST |
| FR-038 | UI 提供 3 个按钮：Done / Snooze / Skip | MUST |
| FR-039 | Done → Completed，重置计时器 | MUST |
| FR-040 | Snooze → Deferred，N 分钟后再次触发，不重置计时器 | MUST |
| FR-041 | Skip → Skipped，重置计时器 | MUST |

### 提醒优先级

| ID | Requirement | Priority |
|----|------------|----------|
| FR-042 | 多个 Activity 同时到期时合并为一个 Overlay | MUST |
| FR-043 | 合并文案覆盖所有到期 Activity | MUST |
| FR-044 | Done 后所有到期 Activity 一起标记 Completed | MUST |
| FR-045 | 优先级按 Activity 的 priority 字段排序 | MUST |

### 提醒冷却

| ID | Requirement | Priority |
|----|------------|----------|
| FR-046 | 用户点击 Done 后至少 10 秒内不弹第二个 Overlay | MUST |

## 4.3 Notification Window（Overlay）

| ID | Requirement | Priority |
|----|------------|----------|
| FR-047 | Overlay 全屏覆盖当前活跃显示器 | MUST |
| FR-048 | Overlay 永远置顶 | MUST |
| FR-049 | Overlay 半透明背景 | SHOULD |
| FR-050 | Overlay 带弹出/关闭动画 | SHOULD |
| FR-051 | Overlay 只在 Done/Snooze/Skip 时关闭 | MUST |
| FR-052 | Overlay 不自动关闭 | MUST |
| FR-053 | Overlay 不响应 ESC | MUST |
| FR-054 | Overlay 不响应点击外部关闭 | MUST |
| FR-055 | Overlay 显示在鼠标最后所在屏幕 | SHOULD |

## 4.4 Reminder Scheduler（提醒调度器）

| ID | Requirement | Priority |
|----|------------|----------|
| FR-056 | 基于 Working Time 计时，不是 Clock Timer | MUST |
| FR-057 | 只有 Working 状态累加时间 | MUST |
| FR-058 | Idle / Locked / Sleeping 暂停计时 | MUST |

See ADR-0002。

## 4.5 Settings（设置）

| ID | Requirement | Priority |
|----|------------|----------|
| FR-059 | 喝水间隔：默认 60min，范围 15–120min | MUST |
| FR-060 | 站立间隔：默认 45min，范围 15–120min | MUST |
| FR-061 | 闲置暂停阈值：默认 5min，范围 1–30min | MUST |
| FR-062 | Snooze 间隔：默认 10min，范围 5–30min | SHOULD |
| FR-063 | Snooze 上限：默认无限 | MAY |
| FR-064 | 开机启动：默认 OFF | SHOULD |
| FR-065 | 全屏提醒：默认 ON | SHOULD |
| FR-066 | 设置不超过 7 项 | MUST |
| FR-067 | 设置使用 Persistent Storage | MUST |
| FR-068 | Settings 包含 version 字段，支持 Migration | SHOULD |
| FR-069 | Settings 修改间隔后当前 Timer 不重置，下次触发使用新间隔 | MUST |

## 4.6 Tray（系统托盘）

| ID | Requirement | Priority |
|----|------------|----------|
| FR-070 | 托盘常驻，包含暂停提醒/恢复提醒/记录一次喝水/设置/退出 | MUST |
| FR-071 | "记录一次喝水" = Completed + 重置计时器 | MUST |
| FR-072 | 暂停提醒：Activity 继续，Working 继续，Reminder 暂停 | MUST |
| FR-073 | 恢复提醒后从暂停处继续 | MUST |

## 4.7 首次启动引导

| ID | Requirement | Priority |
|----|------------|----------|
| FR-074 | 首次启动显示两步引导 | MUST |
| FR-075 | 第一页：理念 + 隐私声明 | MUST |
| FR-076 | 第二页：默认配置 + 开机启动勾选 | MUST |
| FR-077 | 用户可直接点"开始使用"，无需调整 | MUST |
| FR-078 | 非首次启动静默启动 | MUST |

## 4.8 Activity History（行为记录）

| ID | Requirement | Priority |
|----|------------|----------|
| FR-079 | V1 即记录 Activity 历史，为 V2 统计铺路 | MUST |
| FR-080 | 记录字段：activity, status, triggered_at, responded_at, working_duration_min | MUST |
| FR-081 | 时间戳使用 UTC | MUST |
| FR-082 | 存储使用 Persistent Storage | MUST |
| FR-083 | 已触发但无响应的 Reminder 记录为 status: "Triggered" | MUST |

## 4.9 Event Contract

Reminder Engine 向 UI 层发布统一事件。事件格式语言无关。具体实现见 Tech Design。

| ID | Requirement | Priority |
|----|------------|----------|
| FR-084 | Engine 向 UI 发布统一格式事件 | MUST |
| FR-085 | 事件格式语言无关 | SHOULD |

事件列表见 Appendix A。

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

# 6. Edge Cases

| 场景 | 行为 | Related |
|------|------|---------|
| 用户关机 | 已 Triggered 的 Reminder 写入 History（status: "Triggered"），下次启动重新计时 | FR-083 |
| 用户注销 | 同锁屏处理：Pause，重新登录后 Resume | FR-020, FR-022 |
| 修改系统时间 | Monotonic Clock，无影响 | FR-030 |
| 修改时区 | Monotonic Clock，无影响；History 时间戳用 UTC | FR-030, FR-081 |
| 睡眠 8 小时 | 不补偿 Reminder，唤醒后从暂停处继续累加 | FR-023 |
| Settings 修改间隔 | 当前 Timer 不重置，下次触发后使用新间隔 | FR-069 |
| Store 文件损坏 | 恢复默认设置，保留损坏文件为备份，提示用户 | — |
| History 写失败 | 静默忽略，不阻塞主流程，记录错误日志 | — |
| Overlay 弹出时用户锁屏 | Overlay 保持显示，解锁后仍在（等待用户响应） | FR-052 |
| 网络断开 | 无影响（不联网） | FR-028 |

---

# 7. Error Recovery

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
写入错误日志
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
| Reminder Miss Rate | < 1% |
| Activity Detection Accuracy | > 99% |
| State Transition Correctness | 100% |

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
- 只监测活动状态

此声明应在官网第一页和首次引导中明确展示。

---

# 9. Success Metrics

V1 Definition of Done：

### 用户体验

- 用户无需阅读文档即可开始使用
- 用户一天至少响应一次 Reminder

### 软件质量

- 提醒成功率 > 95%
- Reminder Miss Rate < 1%
- Activity Detection Accuracy > 99%
- 软件一周稳定运行（无内存泄漏、无崩溃）

### 资源占用

- 内存 < 50MB
- CPU Idle < 0.5%

---

# 10. Known Risks

| 风险 | 影响 | 缓解策略 |
|------|------|---------|
| Windows API 版本兼容 | 不同 Windows 版本的 Activity Detection API 行为不一致 | 明确支持 Windows 10 21H2+，降级路径测试 |
| 全屏判断误判 | 某些应用全屏状态无法准确检测 | 记录日志，V2 加入 App Detection 兜底 |
| 睡眠恢复事件丢失 | 部分机器/驱动唤醒后事件丢失 | Monotonic Clock 兜底检测（定期校验） |
| 企业电脑权限限制 | 某些企业策略限制键盘监听 | 引导页说明权限，检测失败降级为仅鼠标监听 |
| 杀毒软件误报 | 全局键盘监听被标记为 keylogger | 隐私声明 + 引导页说明 + 白名单申请 |
| Tauri WebView 兼容性 | WebView2 运行时版本差异 | 打包嵌入 WebView2 Bootstrapper |

---

# 11. Assumptions

| 假设 | 如果失效 |
|------|---------|
| 用户每天使用电脑 | 需要考虑"隔天"场景 |
| 用户有喝水条件 | 某些场景无法立即喝水，Snooze 提供缓冲 |
| 用户默认接受全屏 Overlay | 未来可能需要非全屏提醒选项 |
| 用户运行 Windows 10 21H2+ | 旧版本需降级方案或放弃支持 |
| 单用户使用（非多人共用电脑） | 多用户需按用户隔离 Settings 和 History |
| 用户不会频繁修改系统时间 | Monotonic Clock 兜底 |

---

# 12. Out of Scope

| 功能 | 原因 | 计划版本 |
|------|------|---------|
| 跨平台（macOS / Linux） | 先做好 Windows | V6 |
| App Detection | V1 不需要，全屏勿扰足够 | V2 |
| 桌宠 | 属于表现层 | V3 |
| Dashboard | 数据先记，V2 再展示 | V2 |
| 成长系统 / 游戏化 | 不是 V1 核心链路 | V4 |
| 账号 / 云同步 | 本地优先 | 远期 |
| 插件系统 | V1 只有 water 和 stand | V5 |
| 自定义 Health Activity | 插件系统的前提 | V5 |
| 排行榜 | 违反 Respect the User 原则 | 不做 |
| 惩罚机制 | 违反产品人格 | 不做 |

---

# 13. Open Questions

| # | Question | Status | Owner |
|---|---------|--------|-------|
| OQ-001 | Borderless Fullscreen 是否算 DND？ | TODO | Yan |
| OQ-002 | Windows HDR 模式下 Overlay 是否异常？ | TODO | Yan |
| OQ-003 | 多用户共用电脑是否需要按用户隔离数据？ | TODO | Yan |
| OQ-004 | 远程桌面（RDP）场景下 Activity Detection 是否正常？ | TODO | Yan |

---

# Appendix A: Event Contract

Reminder Engine 向 UI 层发布的事件列表。事件格式语言无关，具体实现见 Tech Design。

| 事件 | 说明 | 核心字段 |
|------|------|---------|
| ReminderTriggered | 提醒触发 | activities, title, message, duration |
| ReminderCompleted | 用户完成 | activity, duration |
| ReminderDeferred | 用户延后 | activity, snooze_until |
| ReminderSkipped | 用户跳过 | activity |
| ActivityStateChanged | 活动状态变化 | from, to, working_duration |
| SettingsChanged | 设置变更 | settings (full snapshot) |

---

# Appendix B: Glossary

| 术语 | 定义 |
|------|------|
| Activity Monitor | 活动监测模块，监听鼠标/键盘/锁屏/睡眠/唤醒事件 |
| Working | 用户正在使用电脑（有鼠标/键盘活动）的状态 |
| Idle | 用户未使用电脑（无鼠标/键盘活动超过阈值）的状态 |
| Locked | 屏幕锁定状态 |
| Sleeping | 系统休眠状态 |
| DND | Do Not Disturb，勿扰模式，Reminder 暂停弹出 |
| Session | 一次连续 Working 的时间段，从检测到活动开始，到 Idle 超过阈值结束 |
| Health Activity | 健康行为，由数据模型定义（id, name, interval, priority, icon, message, action） |
| Reminder Engine | 提醒引擎，管理 Rule、Activity 状态和提醒生命周期 |
| Reminder Rule | 提醒规则，定义触发条件（condition）和对应的 Activity |
| Reminder Scheduler | 提醒调度器，基于 Working Time 计时 |
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
| MUST | 必须满足 |
| SHOULD | 应该满足，时间不够可延后 |
| MAY | 可选 |
