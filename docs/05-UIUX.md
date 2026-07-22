# Water Me — UI/UX Design Specification (V1)

> **Humans need watering, too.**
>
> 这份文档回答:**Water Me 长什么样、动起来什么样、怎么用、为什么这样设计。**
>
> 设计原则:产品哲学、Personality、Copy 语调统一遵循 [00-Vision.md](./00-Vision.md);功能边界遵循 [01-PRD.md](./01-PRD.md);模块拆分遵循 [02-Architecture.md](./02-Architecture.md);版本路线遵循 [03-Roadmap.md](./03-Roadmap.md)。

---

# 0. 设计一句话

> **像一本摊开的植物志,不是另一个冷冰冰的提醒 App。**
>
> Water Me 的 UI 不是"通知",而是一位坐在你屏幕前、等你浇水的伙伴。

---

# 1. 设计语言总览

## 1.1 调性关键词

```
Herbal         草本的
Botanical      植物志的
Hand-written   手写感
Patient        耐心的
Gentle         温柔的
Never blame    永不指责
```

## 1.2 调性反例(不要做的事)

| 不要 | 为什么 |
|---|---|
| 鲜艳的纯红 `#FF0000` | 警告色、命令感强,违背 "Never blame" |
| 紫色渐变白底 | AI 通用审美,千篇一律 |
| Inter / Roboto / 系统默认 | 缺品牌独特性,字太"工程" |
| 圆角 24px+ 卡片 | 偏"消费 App",不够"植物志" |
| "你今天没喝水,失败" | 违背 Personality,直接 Out |
| 排行榜/积分/连击数 | 违背 Respect the User |
| 弹窗/Toast/Snackbar 三角箭头 | 工具味太重 |
| 动效时间 > 500ms | 打断工作,违背 "Less is More" |

## 1.3 调性正例(要做的事)

| 要做 | 例子 |
|---|---|
| 嫩叶绿、沙土色、米白纸张 | 主色三色 |
| Fraunces + 思源宋体 SC | 衬线手账感 |
| 圆角 4–8px | 微圆角,植物志书页 |
| 弱化边框,使用纸纹/分栏线 | 手账本分栏感 |
| "去给自己接一杯水" | 邀请,不是命令 |
| 错峰出现 80ms 间隔 | 呼吸感 |
| 水彩植物插画(描线 + 半透明色块) | Overlay 主角 |
| 默认静音 | "在图书馆陪伴" |

---

# 2. 设计 Token

所有 token 通过 CSS Variables 暴露,前端按 `prefers-color-scheme` 切换 Light/Dark。

## 2.1 Color

### 2.1.1 Light Mode (默认)

| Token | 值 | 用途 |
|---|---|---|
| `--bg-paper` | `#F6F2E9` | 主背景,米白纸张 |
| `--bg-overlay-mask` | `rgba(45, 58, 47, 0.42)` | Overlay 遮罩,墨绿半透明 |
| `--bg-card` | `#FBF8F0` | 卡片底,比 paper 略亮 |
| `--bg-elevated` | `#FFFFFF` | 浮层底(Settings 选项) |
| `--ink-primary` | `#2D3A2F` | 主文字,墨绿 |
| `--ink-secondary` | `#5C6B5E` | 次文字,灰绿 |
| `--ink-tertiary` | `#8C978D` | 辅助文字 |
| `--ink-inverse` | `#F6F2E9` | 暗底上的文字 |
| `--accent-leaf` | `#7CA982` | 嫩叶绿,主交互色 |
| `--accent-leaf-deep` | `#4F7A56` | 深绿,按下/激活 |
| `--accent-leaf-soft` | `#D4E4D6` | 浅绿,hover 底 |
| `--accent-water` | `#7AAFC4` | 水蓝,water activity |
| `--accent-soil` | `#B58A60` | 沙土,stand activity |
| `--accent-sun` | `#E8B860` | 暖光,Done 后高亮 |
| `--border-line` | `#D9D2C2` | 手账分栏线 |
| `--border-card` | `#E8E0CD` | 卡片描边(默认 1px,半透明) |
| `--shadow-card` | `0 2px 8px rgba(45, 58, 47, 0.06), 0 1px 2px rgba(45, 58, 47, 0.04)` | 卡片投影,极轻 |
| `--shadow-elevated` | `0 8px 24px rgba(45, 58, 47, 0.10), 0 2px 6px rgba(45, 58, 47, 0.06)` | 浮层投影 |
| `--paper-grain` | `url("/textures/paper-grain.svg")` | 纸张纹理叠加层(opacity 0.04) |

### 2.1.2 Dark Mode

| Token | 值 | 用途 |
|---|---|---|
| `--bg-paper` | `#1A211C` | 主背景,夜墨绿 |
| `--bg-overlay-mask` | `rgba(8, 12, 9, 0.62)` | Overlay 遮罩,更深 |
| `--bg-card` | `#222A23` | 卡片底,略亮 |
| `--bg-elevated` | `#2C352E` | 浮层底 |
| `--ink-primary` | `#E8E4D8` | 主文字,米色 |
| `--ink-secondary` | `#B5BDB0` | 次文字 |
| `--ink-tertiary` | `#7A8478` | 辅助文字 |
| `--ink-inverse` | `#1A211C` | 亮底上的文字 |
| `--accent-leaf` | `#8FB996` | 嫩叶绿(亮一档) |
| `--accent-leaf-deep` | `#A8C8AE` | 深绿激活 |
| `--accent-leaf-soft` | `#2E3A30` | 浅绿 hover 底 |
| `--accent-water` | `#8AB8CE` | 水蓝(亮) |
| `--accent-soil` | `#C7A085` | 沙土(亮) |
| `--accent-sun` | `#EFC477` | 暖光 |
| `--border-line` | `#3A4438` | 分栏线 |
| `--border-card` | `#3A4438` | 卡片描边 |
| `--shadow-card` | `0 2px 8px rgba(0, 0, 0, 0.30), 0 1px 2px rgba(0, 0, 0, 0.20)` | 卡片投影 |
| `--shadow-elevated` | `0 8px 24px rgba(0, 0, 0, 0.40), 0 2px 6px rgba(0, 0, 0, 0.25)` | 浮层投影 |

### 2.1.3 Activity 配色映射

| Activity | 主色 | 插画主色 | 用法 |
|---|---|---|---|
| `water` | `--accent-water` | 水蓝 | 喝水提醒 |
| `stand` | `--accent-soil` | 沙土 | 站立提醒 |

### 2.1.4 Overlay 角色文案色

| 状态 | Token | 说明 |
|---|---|---|
| 缺水 | `--ink-secondary` | 略灰,植物微垂 |
| 等待 | `--ink-primary` | 正常 |
| 刚浇过 | `--accent-leaf-deep` | 翠绿,植物立起 |

## 2.2 Typography

### 2.2.1 字体栈

```css
/* 中文衬线(主) */
--font-serif-cn: "Source Han Serif SC", "Noto Serif SC", "Songti SC", "STSong", serif;

/* 英文/数字衬线(主) */
--font-serif-en: "Fraunces", "Lora", "EB Garamond", "Iowan Old Style", Georgia, serif;

/* 中文无衬线(辅助,极少量) */
--font-sans-cn: "Source Han Sans SC", "Noto Sans SC", "PingFang SC", system-ui, sans-serif;

/* 英文无衬线(辅助) */
--font-sans-en: "Inter", "IBM Plex Sans", system-ui, sans-serif;

/* 组合简写 */
--font-display: var(--font-serif-cn), var(--font-serif-en);
--font-body: var(--font-serif-cn), var(--font-serif-en);
--font-mono: "JetBrains Mono", "SF Mono", Consolas, monospace;
```

> **取舍说明**:V1 仅引入 Fraunces(英文/数字衬线,可变字体)和 Noto Serif SC subset(思源宋体,subset 仅含实际使用字符)。如果包体受限,Noto Serif SC 改为 fallback 到系统衬线。

### 2.2.2 Type Scale

| Token | Size | Line-height | Weight | 用法 |
|---|---|---|---|---|
| `--text-display` | 32px / 2rem | 1.3 | 500 | Overlay 标题 |
| `--text-h1` | 24px / 1.5rem | 1.35 | 500 | Onboard 标题 |
| `--text-h2` | 20px / 1.25rem | 1.4 | 500 | Settings 章节 |
| `--text-h3` | 17px / 1.0625rem | 1.5 | 500 | 卡片标题 |
| `--text-body-lg` | 16px / 1rem | 1.6 | 400 | 主文案 |
| `--text-body` | 15px / 0.9375rem | 1.6 | 400 | 正文 |
| `--text-caption` | 13px / 0.8125rem | 1.5 | 400 | 描述/辅助 |
| `--text-micro` | 12px / 0.75rem | 1.4 | 400 | 最小提示、隐私角标 |

### 2.2.3 数字/时间字

数字统一使用 Fraunces 衬线,带 old-style figures(`font-variant-numeric: oldstyle-nums`)。

```css
.duration {
  font-family: var(--font-serif-en);
  font-variant-numeric: oldstyle-nums;
  font-feature-settings: "ss01", "tnum" 0;
}
```

## 2.3 Spacing

| Token | 值 | 用法 |
|---|---|---|
| `--space-1` | 4px | 微调 |
| `--space-2` | 8px | 行内 |
| `--space-3` | 12px | 元素内边距 |
| `--space-4` | 16px | 卡片内边距 |
| `--space-5` | 20px | 区块间距 |
| `--space-6` | 24px | 章节间距 |
| `--space-8` | 32px | 大间距 |
| `--space-10` | 40px | 页面级 |
| `--space-12` | 48px | Overlay 卡片内边距 |

## 2.4 Radius

| Token | 值 | 用法 |
|---|---|---|
| `--radius-xs` | 2px | 微圆角,纸页边角 |
| `--radius-sm` | 4px | 按钮、输入框 |
| `--radius-md` | 6px | 卡片(主) |
| `--radius-lg` | 8px | 浮层(大) |
| `--radius-pill` | 999px | 状态徽章 |

> **核心原则**:最大圆角不超过 8px,贯彻"植物志书页"调性,远离"消费 App 大圆角"。

## 2.5 Motion

### 2.5.1 Timing Tokens

| Token | 值 | 用法 |
|---|---|---|
| `--ease-out-natural` | `cubic-bezier(0.22, 0.61, 0.36, 1)` | 入场,自然减速 |
| `--ease-in-soft` | `cubic-bezier(0.55, 0.06, 0.68, 0.19)` | 退场,轻柔加速 |
| `--ease-in-out-soft` | `cubic-bezier(0.65, 0, 0.35, 1)` | 循环微动 |
| `--dur-instant` | 80ms | hover 微变 |
| `--dur-fast` | 180ms | 按钮反馈 |
| `--dur-base` | 300ms | Overlay 入场 |
| `--dur-slow` | 500ms | 章节切换 |
| `--dur-loop` | 2400ms | 植物摇曳循环 |

### 2.5.2 命名动效

| Motion | 时长 | 缓动 | 描述 |
|---|---|---|---|
| `m-sprout-in` | 320ms | `--ease-out-natural` | Overlay 出现:背景淡入 + 卡片从下浮入 16px |
| `m-sway-loop` | 2400ms | `--ease-in-out-soft` | 植物插画循环摇曳 ±2deg |
| `m-stagger-rise` | 300ms + 80ms × n | `--ease-out-natural` | 元素错峰出现(插画 → 标题 → 副文 → 按钮) |
| `m-leaf-lift` | 220ms | `--ease-out-natural` | Done 反馈:植物上弹 4px + 叶尖亮起 |
| `m-press-soft` | 120ms | `--ease-out-natural` | 按钮按下:scale(0.97) + 阴影减半 |
| `m-shimmer-soft` | 1200ms | `--ease-in-out-soft` | 滑块拖动时拇指的光晕 |
| `m-pulse-prompt` | 1800ms | `--ease-in-out-soft` | Overlay 出现时引导视线:主按钮光晕呼吸 |

### 2.5.3 错峰节奏(Stagger)

```
0ms      背景遮罩淡入 (m-sprout-in)
+80ms    插画植物上浮 (m-sprout-in)
+160ms   标题上浮
+240ms   副文上浮
+320ms   主按钮上浮 + 光晕呼吸开启 (m-pulse-prompt)
+400ms   次按钮上浮
```

## 2.6 图标系统

V1 不引入图标库,使用:

- **Emoji**(仅 Activity 类型):💧 🧍 — 来自 Unicode,无字体依赖。
- **SVG 植物插画**(主):Overlay / Onboard / Settings 装饰。
- **极简线性图标**(10 个以内,自绘 SVG,1.5px stroke,圆角端点):
  - close, check, clock, settings, leaf, drop, pause, play, chevron-down, chevron-up。

> **反例**:不用 lucide-react / heroicons(品牌独特性不足)。

## 2.7 纸张纹理 / 装饰

- 全局背景叠加 `--paper-grain` 纸张纹理(opacity 0.04,深色模式 0.06)。
- 卡片左上角可选手写日期戳(`Water Me · 2026.07.22`),font-style: italic,Fraunces 12px。
- 装饰元素:
  - 角落叶片水印(SVG,opacity 0.06)。
  - 分栏线 `--border-line`,1px,代替硬边框。
  - 段落之间用一短横线(40px × 1px)代替空行。

---

# 3. 全局布局规则

## 3.1 栅格

- 基线网格:8px。
- 卡片内边距:24px(Settings/Onboard)、48px(Overlay)。
- 卡片间距:24px。

## 3.2 窗口清单

| Window | 尺寸 | 永远置顶 | 出现时机 |
|---|---|---|---|
| Overlay | 全屏(覆盖当前活跃显示器) | Yes | Reminder Triggered |
| Onboard | 480 × 640 | No | 首次启动 |
| Settings | 480 × 640 | No | 托盘点击"设置" |
| (V3) Pet | 200 × 200 | Yes(可选) | 启动后常驻右下角 |

> **V1 没有主窗口**。所有 UI 都是独立窗口,通过 IPC 与后端通信。

## 3.3 焦点与可访问性

- 所有可交互元素 keyboard 可达(Tab/Shift+Tab/Enter/Space)。
- 焦点环:2px `--accent-leaf`,outline-offset 3px。
- 颜色对比度 WCAG AA(正文 4.5:1,大文字 3:1)。
- ARIA:`role="dialog"` + `aria-modal="true"`(Overlay);`role="alert"` 用于重要提示。
- 动效可减弱:`@media (prefers-reduced-motion: reduce)` → 所有循环动画停止,入场动画缩短至 100ms。
- 屏幕阅读器:Overlay 包含 `aria-label` 与 `aria-describedby` 指向主文案。

---

# 4. 组件库(最小集)

V1 只实现以下组件,**禁止新增未列出的抽象**(See 00-Vision YAGNI)。

## 4.1 Button

```css
.btn {
  font-family: var(--font-body);
  font-size: var(--text-body-lg);
  font-weight: 500;
  line-height: 1;
  padding: 14px 24px;          /* 大点击区:最小 44px 高 */
  border-radius: var(--radius-sm);
  border: 1px solid transparent;
  cursor: pointer;
  transition: background var(--dur-fast) var(--ease-out-natural),
              transform var(--dur-fast) var(--ease-out-natural),
              box-shadow var(--dur-fast) var(--ease-out-natural);
}

/* 主操作(Done) */
.btn--primary {
  background: var(--accent-leaf);
  color: var(--ink-inverse);
  box-shadow: var(--shadow-card);
}
.btn--primary:hover {
  background: var(--accent-leaf-deep);
  transform: translateY(-1px);
  box-shadow: var(--shadow-elevated);
}
.btn--primary:active {
  transform: translateY(0) scale(0.97);
  box-shadow: var(--shadow-card);
}

/* 次操作(Snooze) */
.btn--secondary {
  background: transparent;
  color: var(--ink-primary);
  border-color: var(--border-line);
}
.btn--secondary:hover {
  background: var(--accent-leaf-soft);
  border-color: var(--accent-leaf);
}

/* 三级操作(Skip) */
.btn--tertiary {
  background: transparent;
  color: var(--ink-secondary);
  padding: 10px 16px;
  font-size: var(--text-caption);
}
.btn--tertiary:hover {
  color: var(--ink-primary);
}

/* 禁用 */
.btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
  transform: none;
}
```

**变体**:
- `btn--primary` 主操作(Done)
- `btn--secondary` 次操作(Snooze)
- `btn--tertiary` 弱操作(Skip)

## 4.2 Card

```css
.card {
  background: var(--bg-card);
  border: 1px solid var(--border-card);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-card);
  padding: var(--space-6);
  position: relative;
}
.card::before {
  content: "";
  position: absolute;
  inset: 0;
  background: var(--paper-grain);
  opacity: 0.04;
  border-radius: inherit;
  pointer-events: none;
}
```

## 4.3 Switch(Toggle)

```css
.switch {
  position: relative;
  width: 44px;
  height: 24px;
  background: var(--border-line);
  border-radius: var(--radius-pill);
  cursor: pointer;
  transition: background var(--dur-fast) var(--ease-out-natural);
}
.switch::after {
  content: "";
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  background: var(--bg-paper);
  border-radius: 50%;
  box-shadow: var(--shadow-card);
  transition: transform var(--dur-fast) var(--ease-out-natural);
}
.switch[aria-checked="true"] {
  background: var(--accent-leaf);
}
.switch[aria-checked="true"]::after {
  transform: translateX(20px);
}
```

## 4.4 Slider

```css
.slider {
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: 4px;
  background: var(--border-line);
  border-radius: var(--radius-pill);
  outline: none;
}
.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 20px;
  height: 20px;
  background: var(--accent-leaf);
  border: 2px solid var(--bg-paper);
  border-radius: 50%;
  cursor: grab;
  box-shadow: var(--shadow-card);
  transition: transform var(--dur-instant) var(--ease-out-natural);
}
.slider::-webkit-slider-thumb:hover {
  transform: scale(1.15);
  box-shadow: 0 0 0 6px var(--accent-leaf-soft);
}
.slider::-webkit-slider-thumb:active {
  cursor: grabbing;
  transform: scale(1.1);
}
```

## 4.5 Stepper(数字加减)

```css
.stepper {
  display: inline-flex;
  align-items: center;
  gap: var(--space-3);
  padding: 8px 12px;
  background: var(--bg-elevated);
  border: 1px solid var(--border-line);
  border-radius: var(--radius-sm);
}
.stepper__btn {
  width: 28px;
  height: 28px;
  display: grid;
  place-items: center;
  border-radius: var(--radius-xs);
  color: var(--ink-secondary);
  transition: background var(--dur-instant) var(--ease-out-natural);
}
.stepper__btn:hover {
  background: var(--accent-leaf-soft);
  color: var(--ink-primary);
}
.stepper__value {
  font-family: var(--font-serif-en);
  font-variant-numeric: oldstyle-nums;
  min-width: 56px;
  text-align: center;
  font-size: var(--text-body-lg);
}
```

## 4.6 Progress Ring(工作时长)

仅 Overlay 装饰用,240×240,SVG:

```css
.progress-ring__track  { stroke: var(--border-line); }
.progress-ring__fill   { stroke: var(--accent-leaf); transition: stroke-dashoffset 600ms var(--ease-out-natural); }
.progress-ring__center { font-family: var(--font-serif-en); font-variant-numeric: oldstyle-nums; }
```

## 4.7 PlantIllustration

见第 7 节「插画规范」。

---

# 5. 窗口规范

## 5.1 Overlay(提醒窗口)

**目标**:用户工作 60/45 分钟后,温柔打断,不指责。

### 5.1.1 形态

- 覆盖当前活跃显示器(全屏,非全 OS 全屏)。
- 半透明遮罩 `--bg-overlay-mask`。
- 居中一张 600 × 420 的"植物志卡片"(`--bg-card` + 纸纹 + 圆角 8px + 极轻投影)。
- 卡片左上角手写日期戳,右下角活动类型 emoji。

### 5.1.2 内部结构(从上到下)

```
┌───────────────────────────── 600 × 420 ─────────────────────────┐
│  Water Me · 2026.07.22 14:30                       💧 water     │  ← 32px padding
│  ─────────────────────────────────────────────────────────      │
│                                                                  │
│            [ 植物插画(240×240)摇曳中 ]                          │
│                                                                  │
│              去给自己接一杯水。                                  │  ← display 32px
│              You've been working for an hour.                   │  ← caption italic
│                                                                  │
│              ────── 60 min ──────                                │  ← 分割短线
│                                                                  │
│         [ 我喝了 ]    [ 10分钟后 ]    [ 今天跳过 ]               │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### 5.1.3 文案规则

| Activity | 中文主文案 | 英文副文(可选) |
|---|---|---|
| `water` | 去给自己接一杯水。 | *May I trouble you for a glass of water?* |
| `stand` | 站起来,伸展一下。 | *Please stretch a little.* |
| 合并 | 去接杯水,顺便活动一下。 | *A glass of water, and a stretch.* |

> **永远不写**:"该喝水了" / "你工作太久了" / "你今天没喝水"。

### 5.1.4 按钮文案与映射

| 按钮 | 变体 | 文案 | 行为 |
|---|---|---|---|
| 主 | `btn--primary` | 我喝了 | Done(Activity 标记 Completed,计时器重置) |
| 次 | `btn--secondary` | 10 分钟后 | Snooze(10min 后再弹,计时器不重置) |
| 三级 | `btn--tertiary` | 今天跳过 | Skip(计时器重置) |

> **Snooze 文案动态化**:根据 Settings 中的 `snooze_interval_min` 显示对应分钟数。

### 5.1.5 交互

- 点击 `Done` → 卡片轻浮(leaf-lift 动效)→ 600ms 后淡出。
- 点击 `Snooze` → 卡片下沉 8px → 300ms 后淡出。
- 点击 `Skip` → 卡片直接淡出 200ms(快速,但不是"关闭")。
- **不响应**:ESC、点击遮罩外部、Alt+F4。
- 锁屏/睡眠时 Overlay 保持,不解锁不解。

### 5.1.6 入场动效(分镜)

```
t=0ms       遮罩 opacity 0 → 1 (300ms)
t=80ms      卡片 translateY 16px → 0, opacity 0 → 1 (300ms)
t=160ms     插画 opacity 0 → 1, scale 0.94 → 1 (300ms)
            并开启 m-sway-loop
t=240ms     主文案 opacity 0 → 1, translateY 8px → 0
t=320ms     副文 + 分隔线 opacity 0 → 1
t=400ms     主按钮 opacity 0 → 1, translateY 6px → 0
            并开启 m-pulse-prompt
t=480ms     次按钮 opacity 0 → 1
t=560ms     三级按钮 opacity 0 → 1
```

### 5.1.7 失败案例(永远不做)

- ❌ 加倒计时自动关闭
- ❌ 屏蔽任务栏
- ❌ 多卡片堆叠(同时只 1 张)
- ❌ 显示"还剩几次今天没喝"
- ❌ 红色警告色

## 5.2 Onboard(首次启动引导)

**目标**:首次启动,2 步完成,零学习成本。

### 5.2.1 窗口属性

- 480 × 640,居中显示,不可调整大小。
- 浅米色背景,顶部一条 8px 渐变条(`--accent-leaf` → `--accent-water`)。
- 关闭按钮隐藏在标题栏(无 Alt+F4 阻断,因为这是临时窗口)。

### 5.2.2 Step 1 — 欢迎 + 隐私

```
┌─────────────────── 480 × 640 ───────────────────┐
│                                                    │
│           [ 植物插画 160×160 摇曳 ]                │
│                                                    │
│          Water Me                                  │  ← display
│          Humans need watering, too.                │  ← caption italic
│                                                    │
│          ────                                     │
│                                                    │
│          我会在你工作太久的时候,                  │  ← body
│          轻轻提醒你照顾自己。                      │
│                                                    │
│          隐私承诺                                  │  ← h3
│          ────                                     │
│          • 不联网                                  │
│          • 不上传任何数据                          │
│          • 不记录键盘内容                          │
│          • 不记录鼠标坐标                          │
│          • 只监测"有没有在用电脑"                  │
│                                                    │
│                                                    │
│              [ 下一步 → ]                          │
│                                                    │
└────────────────────────────────────────────────────┘
```

### 5.2.3 Step 2 — 默认配置

```
┌─────────────────── 480 × 640 ───────────────────┐
│                                                    │
│            [ 步骤指示:● ○ ]                        │
│                                                    │
│            已经准备好陪你。                        │  ← h1
│            ────                                   │
│                                                    │
│            默认配置(可直接开始使用):              │  ← body
│                                                    │
│            喝水间隔              60 min            │  ← 滑块
│            ────                                   │
│            站立间隔              45 min            │
│            ────                                   │
│            闲置暂停              5 min             │
│            ────                                   │
│                                                    │
│            ☐  开机自动启动                        │  ← Switch
│                                                    │
│                                                    │
│        [ ← 上一步 ]      [ 开始使用 ]              │
│                                                    │
└────────────────────────────────────────────────────┘
```

### 5.2.4 交互

- "下一步" / "开始使用" → 保存 `first_launch: true` 到 Settings → 关闭 Onboard → 启动 Reminder Engine。
- "上一步" / "下一步" 切换时,Step 内容横向滑动(stagger rise)。
- 任何滑块修改即时持久化,但不立即生效(下次触发使用)。

## 5.3 Settings(设置)

**目标**:手账本式布局,大点击区,**最多 7 项**(PRD 约束)。

### 5.3.1 窗口属性

- 480 × 640,居中,可最小化到托盘。
- 顶部固定:返回 + 标题"Water Me · 设置"。
- 底部固定:版本号 + 隐私链接。

### 5.3.2 布局(手账本)

```
┌─────────────────── 480 × 640 ───────────────────┐
│  ←  Water Me · 设置                              │  ← 固定头部 64px
├──────────────────────────────────────────────────┤
│                                                    │
│  提醒                                              │  ← 章节标题
│  ────                                             │  ← 40px 短线
│                                                    │
│  喝水间隔                          60 min   ▸     │  ← 可点击行
│  每工作 60 分钟提醒一次。                          │  ← 描述 caption
│  ────────────────────────────────────────────     │
│                                                    │
│  站立间隔                          45 min   ▸     │
│  每工作 45 分钟提醒一次。                          │
│  ────────────────────────────────────────────     │
│                                                    │
│  闲置暂停                          5 min    ▸     │
│  多久没动就暂停计时。                              │
│  ────────────────────────────────────────────     │
│                                                    │
│  Snooze 间隔                      10 min   ▸     │
│  点"10 分钟后"时多久再提醒。                        │
│  ────────────────────────────────────────────     │
│                                                    │
│  系统                                              │
│  ────                                             │
│                                                    │
│  开机自动启动                              [  ]   │
│  ────────────────────────────────────────────     │
│                                                    │
│  全屏提醒                                  [ ●]   │
│  游戏中/演示中默认不打扰。                          │
│  ────────────────────────────────────────────     │
│                                                    │
│  …(可滚动)                                        │
│                                                    │
├──────────────────────────────────────────────────┤
│  v0.1.0  ·  隐私承诺                              │  ← 固定底部
└──────────────────────────────────────────────────┘
```

### 5.3.3 交互模式

- **行点击** → 弹出 Stepper 弹层(居中,240×200,带遮罩),点 +/- 调,实时预览;点空白处或回车保存。
- **Switch** → 直接切换,即时持久化。
- **滑块** → 同 Stepper,带光晕 shimmer 动效。
- **保存策略**:所有变更即时写入 Persistent Storage(Settings Store),无需"保存"按钮。

### 5.3.4 动效

- 行 hover → 背景 `--accent-leaf-soft`,padding 略增 → 平滑过渡 180ms。
- 弹层出现 → scale 0.96 → 1 + 透明度 0 → 1,300ms。
- 数值变化 → oldstyle 数字交错(tween 数字,200ms)。

## 5.4 Tray(系统托盘)

### 5.4.1 托盘图标

- 静态 SVG 图标:一株豆苗(2 像素叶 + 1 像素茎,墨绿色)。
- Hover 状态不变(V1)。
- 工作状态/缺水状态(V3 之后):可选替换为不同插画变体。V1 保持单一图标。

### 5.4.2 菜单

| 菜单项 | 快捷键 | 行为 |
|---|---|---|
| Water Me(标题,禁用) | — | — |
| ──── | — | — |
| 暂停提醒 / 恢复提醒 | — | 切换 `pause_reminders` |
| 立即喝水 | — | `record_manual("water")` |
| ──── | — | — |
| 设置 | — | 打开 Settings 窗口 |
| 退出 | — | `app.exit(0)` |

> **菜单字体**:跟随系统,不应用衬线字体(尊重 OS 习惯)。

## 5.5 (V3) Pet(桌宠预留)

**预留视觉方向**,V1 不实现。

- 位置:屏幕右下角,200×200,边缘内边距 16px。
- 形态:同 V1 植物插画体系(Lottie 动画)。
- 状态:5 个状态(normal / thirsty / just-watered / long-stand / celebrate)。
- 交互:可拖动(位置记忆);点击 → 弹迷你气泡(今天的"植物"小记);双击 → 打开 Settings。
- 提醒:代替 Overlay 弹出(植物从右下角游到屏幕中央),V1 的 Overlay 暂时保留作为兜底。

---

# 6. 动效系统

## 6.1 设计原则

| 原则 | 说明 |
|---|---|
| **呼吸感** | 所有循环动效 ≥ 2s,无急迫感 |
| **错峰出现** | 多元素入场 stagger 80ms,避免"啪一下" |
| **自然减速** | 入场用 `ease-out-natural`,不用 `linear` |
| **可减弱** | `prefers-reduced-motion` 全部降级 |
| **不打断** | 动效不阻塞 UI 响应,可被打断 |
| **不喧宾夺主** | 动效服务于内容,不抢戏 |

## 6.2 错峰出现(标准模式)

```css
@keyframes stagger-rise {
  from { opacity: 0; transform: translateY(8px); }
  to   { opacity: 1; transform: translateY(0); }
}

.stagger > * {
  opacity: 0;
  animation: stagger-rise var(--dur-base) var(--ease-out-natural) forwards;
}
.stagger > *:nth-child(1) { animation-delay: 0ms; }
.stagger > *:nth-child(2) { animation-delay: 80ms; }
.stagger > *:nth-child(3) { animation-delay: 160ms; }
.stagger > *:nth-child(4) { animation-delay: 240ms; }
.stagger > *:nth-child(5) { animation-delay: 320ms; }
.stagger > *:nth-child(6) { animation-delay: 400ms; }
```

## 6.3 植物摇曳

```css
@keyframes sway {
  0%, 100% { transform: rotate(-2deg); }
  50%      { transform: rotate(2deg); }
}

.plant {
  transform-origin: 50% 90%;        /* 从根部摆动 */
  animation: sway var(--dur-loop) var(--ease-in-out-soft) infinite;
}
```

> **变体**:缺水状态摆动幅度 ±4deg,周期 1800ms(更急促);刚浇过状态静止,叶尖高光 800ms 后淡出。

## 6.4 Done 反馈

```css
@keyframes leaf-lift {
  0%   { transform: translateY(0)    rotate(0); }
  40%  { transform: translateY(-6px) rotate(0); }
  100% { transform: translateY(0)    rotate(0); }
}
.plant--done {
  animation: leaf-lift var(--dur-base) var(--ease-out-natural);
}
.plant--done::after {
  /* 叶尖亮起 */
  content: "";
  animation: sun-glow 1200ms var(--ease-in-out-soft);
}
@keyframes sun-glow {
  0%   { opacity: 0; filter: drop-shadow(0 0 0 var(--accent-sun)); }
  50%  { opacity: 1; filter: drop-shadow(0 0 12px var(--accent-sun)); }
  100% { opacity: 0; filter: drop-shadow(0 0 0 var(--accent-sun)); }
}
```

## 6.5 减弱模式

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 100ms !important;
  }
  .plant { animation: none; }     /* 摇曳停止 */
  .stagger > * { animation-delay: 0ms !important; }
}
```

---

# 7. 插画规范

## 7.1 风格

**手账水彩植物**:
- 描线:`--ink-primary`,1.5px stroke,圆角端点。
- 色块:半透明 0.85 opacity,允许 2px 偏移(手工感)。
- 高光:`--accent-sun`,小范围 0.3 opacity。
- 阴影:`--ink-primary`,5% opacity,偏右下。
- 纸纹叠加:整体 opacity 0.06,统一纹理感。
- 留白:插画占卡片 30–40%,周围留 ≥ 32px 空间。

## 7.2 V1 必备资产

| 资产 | 用途 | 状态 | 尺寸 |
|---|---|---|---|
| `plant-normal.svg` | Overlay / Onboard 默认 | 翠绿,直立 | 240×240 |
| `plant-thirsty.svg` | Overlay 缺水状态(可选,Snooze 后再用) | 略灰绿,微垂 | 240×240 |
| `plant-just-watered.svg` | Done 反馈 800ms 高亮 | 翠绿,叶尖亮 | 240×240 |
| `plant-onboard.svg` | Onboard 头图(可与 normal 同源) | 同 normal | 160×160 |
| `grain-paper.svg` | 纸纹 | — | 200×200 平铺 |

## 7.3 V3 桌宠预留(Lottie)

- 单个 Lottie 文件 `pet.lottie`,内部含 5 个状态 + 4 个转场。
- 使用 Lottie JSON,体积 < 80KB,60fps。
- 在 V1 阶段,先把 SVG 静态资源做齐,确保 V3 可以平移。

## 7.4 不要做

- ❌ 3D 渲染、复杂渐变、写实风格
- ❌ 高饱和荧光色
- ❌ 黑白线稿(违背"植物志有颜色"的调性)
- ❌ 拟人表情过度(眼睛要小,V1 阶段)

---

# 8. 文案系统(Copy)

## 8.1 总原则

> **永远不指责,永远邀请,永远留余地。**

## 8.2 文案清单

### Overlay 主文案

| 状态 | Activity | 中文 | 英文 |
|---|---|---|---|
| 缺水 | water | 去给自己接一杯水。 | *May I trouble you for a glass of water?* |
| 缺水 | stand | 站起来,伸展一下。 | *Please stretch a little.* |
| 合并 | both | 去接杯水,顺便活动一下。 | *A glass of water, and a stretch.* |

### Overlay 按钮文案

| 按钮 | 中文 | 英文 |
|---|---|---|
| Done | 我喝了 | *I drank* |
| Snooze | N 分钟后 | *In N min* |
| Skip | 今天跳过 | *Skip today* |

### Onboard 文案

| 位置 | 中文 |
|---|---|
| 标题 | Water Me |
| 副标 | *Humans need watering, too.* |
| 主文案 | 我会在你工作太久的时候,轻轻提醒你照顾自己。 |
| 隐私点 | 见 5.2.2 |
| Step 2 标题 | 已经准备好陪你。 |
| Step 2 副 | 默认配置(可直接开始使用) |
| 按钮 | 下一步 / 开始使用 |

### Settings 文案

| 项 | 标题 | 描述 |
|---|---|---|
| 喝水间隔 | 喝水间隔 | 每工作 60 分钟提醒一次。 |
| 站立间隔 | 站立间隔 | 每工作 45 分钟提醒一次。 |
| 闲置暂停 | 闲置暂停 | 多久没动就暂停计时。 |
| Snooze 间隔 | Snooze 间隔 | 点"N 分钟后"时多久再提醒。 |
| 开机启动 | 开机自动启动 | — |
| 全屏提醒 | 全屏提醒 | 游戏中/演示中默认不打扰。 |

### Tray 文案

| 项 | 中文 |
|---|---|
| 标题 | Water Me |
| 暂停 | 暂停提醒 |
| 恢复 | 恢复提醒 |
| 记录 | 立即喝水 |
| 设置 | 设置 |
| 退出 | 退出 |

### 错误 / 边界文案

| 场景 | 中文 |
|---|---|
| Settings 损坏 | 设置文件已恢复为默认值。 |
| History 写失败 | (静默,不弹) |
| 鼠标键盘监听失败 | 需要授权才能正常工作。 |
| 全屏提醒被屏蔽 | (不弹,Overlay 暂存) |

## 8.3 文案永远不写

- ❌ "请立即喝水"
- ❌ "你今天没喝水,失败"
- ❌ "工作狂"
- ❌ "警告"
- ❌ "你错过了一次提醒"
- ❌ "连击中断"
- ❌ "排行榜"
- ❌ 任何感叹号超过 1 个的句子

## 8.4 文案语气速查

```
温  柔   =  邀请,不是命令
耐  心   =  留余地,不催
不指责  =  永远是"我等你"
```

---

# 9. 设计验收清单(Acceptance Checklist)

每一项都必须在代码评审/视觉评审时勾选通过。

## 9.1 视觉

- [ ] Overlay 卡片圆角 ≤ 8px
- [ ] 没有任何纯红 / 警告色出现在主流程
- [ ] 衬线字体(Fraunces / Noto Serif SC)出现在 Overlay 标题与正文
- [ ] Light / Dark 两套配色 token 全部实现,跟系统切换
- [ ] 纸张纹理叠加在所有卡片背景
- [ ] 主交互色为 `--accent-leaf`(嫩叶绿)
- [ ] 焦点环 2px `--accent-leaf` + 3px offset

## 9.2 交互

- [ ] Overlay 不响应 ESC、不响应遮罩点击、不响应 Alt+F4
- [ ] Overlay 关闭只有三条路径:Done / Snooze / Skip
- [ ] Snooze 后计时器不重置(可在 Settings 看 timer_value 不变)
- [ ] Settings 任何变更实时持久化
- [ ] Settings 任何变更不会打断正在进行的 Reminder
- [ ] Onboard "开始使用" 后 `first_launch: true` 写入,不再弹出
- [ ] 全屏应用内 Overlay 不弹,但 Activity 继续,Working 继续

## 9.3 文案

- [ ] 全产品没有出现"请立即"、"警告"、"失败"、"你错过了"等词
- [ ] Done / Snooze / Skip 三个动作可被用户清楚区分
- [ ] 标题使用邀请句(去给自己...),不是命令句(请喝水)
- [ ] Tray 菜单项名称与 PRD 第 6.1 节完全一致

## 9.4 可访问性

- [ ] Tab 顺序合理:主按钮 → 次按钮 → 三级按钮
- [ ] `prefers-reduced-motion: reduce` 下,所有循环动效停止,入场 ≤ 100ms
- [ ] 主文案 / 主按钮有 `aria-label` 或显式文本
- [ ] 颜色对比度 WCAG AA(可用 Chrome DevTools 验证)
- [ ] 键盘可达所有交互(无需鼠标)

## 9.5 性能

- [ ] Overlay 入场动画 ≤ 320ms,主流程 60fps(无 jank)
- [ ] CSS 变量切换主题不引发重排(repaint only)
- [ ] 不引入 icon library(只内联 SVG 或 emoji)
- [ ] 字体 subset / variable font,首屏 < 200KB

## 9.6 与 PRD / 架构一致性

- [ ] Overlay 600×420 居中,符合 02-Architecture 第 5.1 节
- [ ] Settings 480×640,符合 02-Architecture 第 5.1 节
- [ ] Onboard 480×640,符合 02-Architecture 第 5.1 节
- [ ] 三个按钮(Done / Snooze / Skip)与 01-PRD FR-038 一致
- [ ] 不出现 01-PRD § 12 "Out of Scope" 的任何功能(排行榜、自定义 Activity、桌面 Pet 实现)

---

# 10. 设计走查脚本(Design Review Walkthrough)

开发完成后,按下顺序逐一走查(每步用截图 + 录屏 1 分钟取证)。

| 步骤 | 操作 | 预期 |
|---|---|---|
| 1 | 首次启动(Onboard 1/2) | 隐私点 5 条全部可见,植物插画摇曳 |
| 2 | 滑块调到 30min,Switch 切到 ON | 数值实时变化,无"保存"按钮 |
| 3 | 点 "开始使用" | 窗口关闭,托盘出现一株豆苗图标 |
| 4 | 等 1min,触发 mock Reminder | Overlay 全屏罩 + 居中 600×420 卡片 |
| 5 | Overlay 入场 | 错峰 80ms,植物摇曳开始 |
| 6 | Hover 主按钮 | 上抬 1px,变深绿 |
| 7 | 按 ESC | **无反应** |
| 8 | 点击遮罩空白 | **无反应** |
| 9 | 点 "我喝了" | 植物 leaf-lift,卡片淡出,托盘可见 |
| 10 | 重新触发,点 "10 分钟后" | 10min 后再次弹出,计时器未重置 |
| 11 | 重新触发,点 "今天跳过" | 立即淡出 200ms,计时器重置 |
| 12 | 切到 Dark 主题 | 全产品换色,Overlay 重新出现时是深底 |
| 13 | 系统设置 "减少动效" ON | 摇曳停止,入场 ≤ 100ms |
| 14 | 打开游戏全屏 | Overlay 暂存,Working 继续 |
| 15 | 退出全屏 | 之前累积时间继续,可能立即触发 Overlay |

---

# 11. 引用与依赖

| 引用 | 路径 | 关系 |
|---|---|---|
| 产品哲学 | [00-Vision.md](./00-Vision.md) | Personality、Copy 语调、Principle 来源 |
| 功能边界 | [01-PRD.md](./01-PRD.md) | FR 与本规范字段一一对应 |
| 架构 | [02-Architecture.md](./02-Architecture.md) | 窗口尺寸、IPC 事件来源 |
| 路线 | [03-Roadmap.md](./03-Roadmap.md) | V1–V6 桌宠/插件/移动端方向 |
| MVP 早期设计 | [design/MVP.md](./design/MVP.md) | 早期方案、为什么不要桌宠 |

## 11.1 前端依赖(Package)

| 依赖 | 用途 | 备注 |
|---|---|---|
| `react` / `react-dom` | UI 框架 | 已在 `package.json` |
| `@tauri-apps/api` | Tauri IPC | 已在 `package.json` |
| `framer-motion` | Overlay 动效 | 备选,如不引入则用纯 CSS 关键帧 |

> **YAGNI**:V1 不引入 Tailwind / shadcn / radix / Material UI。**全部用 CSS Variables + 自写 CSS 模块**。V1 总 CSS 行数目标 ≤ 1500 行。

## 11.2 字体引入

```html
<!-- index.html <head> 中 -->
<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
<link
  href="https://fonts.googleapis.com/css2?family=Fraunces:opsz,wght@9..144,400;9..144,500&family=Noto+Serif+SC:wght@400;500&display=swap"
  rel="stylesheet"
/>
```

> **离线优先**:**font-display: swap** + 本地 fallback。V1.5 之后考虑把 Fraunces 内嵌到安装包,避免依赖网络字体。Water Me 不联网但首次 WebView 加载时仍可走 CDN。V2 之后改为本地资源。

## 11.3 纹理资源

- `public/textures/paper-grain.svg` — 200×200 平铺纸纹
- `public/illustrations/plant-normal.svg` — 240×240
- `public/illustrations/plant-thirsty.svg` — 240×240
- `public/illustrations/plant-just-watered.svg` — 240×240
- `public/illustrations/plant-onboard.svg` — 160×160
- `public/icons/tray.svg` — 16×16 托盘图标

> **V3 预留**:`public/lottie/pet.lottie` — Lottie 桌宠动画(占位,先不放)。

---

# 12. 变更记录

| 日期 | 版本 | 变更 | 作者 |
|---|---|---|---|
| 2026-07-22 | v0.1.0 | 初稿。V1 设计规范。 | — |

> **维护规则**:任何对 PRD / Architecture / Vision 的修订,必须同步检查本文件相关章节,并在变更记录追加一行。
