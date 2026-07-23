// Settings 设置窗口。手账布局：提醒间隔（点击行内编辑）+ 系统开关。
// See docs/05-UIUX.md §6.3, docs/01-PRD.md FR-047~056/064。
import { useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import gsap from "gsap";

import {
  getSettings,
  listVisibleWindows,
  onSettingsChanged,
  updateSettings,
  type OverlayMode,
  type Settings,
  type WindowInfo,
} from "../../lib/ipc";

const DEFAULTS: Settings = {
  version: 1,
  water_interval_min: 60,
  stand_interval_min: 45,
  idle_threshold_min: 5,
  snooze_interval_min: 10,
  autostart: false,
  fullscreen_reminder: true,
  fullscreen_blocklist: [],
  theme: "system",
  overlay_mode: "fullscreen",
  first_launch: false,
  paused: false,
};

// 提醒方式选项。label 显示在下拉与触发器；desc 是副标题；tag 是气泡角标。
const MODE_OPTIONS: { value: OverlayMode; label: string; desc: string; tag: string }[] = [
  { value: "fullscreen", label: "全屏遮罩", desc: "Fullscreen Overlay", tag: "强阻断" },
  { value: "card", label: "独立卡片", desc: "Card Window", tag: "可拖动" },
  { value: "toast", label: "Toast 通知", desc: "Toast Notification", tag: "自动消失" },
];

type IntervalField =
  | "water_interval_min"
  | "stand_interval_min"
  | "idle_threshold_min"
  | "snooze_interval_min";

const INTERVAL_ROWS: {
  field: IntervalField;
  title: string;
  desc: string;
  min: number;
  max: number;
  step: number;
}[] = [
  { field: "water_interval_min", title: "喝水间隔", desc: "每工作多久提醒一次。", min: 15, max: 180, step: 5 },
  { field: "stand_interval_min", title: "站立间隔", desc: "每工作多久提醒一次。", min: 10, max: 120, step: 5 },
  { field: "idle_threshold_min", title: "闲置暂停", desc: "多久没动就暂停计时。", min: 1, max: 30, step: 1 },
  { field: "snooze_interval_min", title: "Snooze 间隔", desc: "点「N 分钟后」时多久再提醒。", min: 5, max: 60, step: 5 },
];

export function Settings() {
  const [settings, setSettings] = useState<Settings>(DEFAULTS);
  const [editing, setEditing] = useState<IntervalField | null>(null);
  const [pickerOpen, setPickerOpen] = useState(false);
  const [windows, setWindows] = useState<WindowInfo[]>([]);
  // 提醒方式下拉开关 + 当前 hover 预览的模式（null = 不显示气泡）。
  const [modeDropdownOpen, setModeDropdownOpen] = useState(false);
  const [hoverMode, setHoverMode] = useState<OverlayMode | null>(null);
  const modeTriggerRef = useRef<HTMLButtonElement>(null);
  // 下拉面板的 fixed 定位坐标（相对于视口），在打开时由 trigger 的 bounding rect 计算。
  const [ddPos, setDdPos] = useState<{ top: number; left: number; width: number }>({ top: 0, left: 0, width: 180 });
  const root = useRef<HTMLDivElement>(null);

  useEffect(() => {
    getSettings().then(setSettings);
    // 监听后端设置变更（如托盘暂停切换），保持 UI 同步。
    const unlistenP = onSettingsChanged(setSettings);
    return () => {
      unlistenP.then((f) => f());
    };
  }, []);

  // 入场：head 淡入下移，section 错落冒出，foot 稍后淡入。与 Onboard 同款手感。
  // 用 useEffect 而非 useLayoutEffect：先让窗口快速渲染静态内容，再异步跑动画，
  // 避免动画计算阻塞首次绘制造成卡顿感。
  useEffect(() => {
    const ctx = gsap.context(() => {
      gsap.fromTo(".settings__head",
        { opacity: 0, y: -10 },
        { opacity: 1, y: 0, duration: 0.36, ease: "power3.out" });
      gsap.fromTo(".settings__section",
        { opacity: 0, y: 16 },
        { opacity: 1, y: 0, duration: 0.4, ease: "power3.out", stagger: 0.08, delay: 0.05 });
      gsap.fromTo(".settings__foot",
        { opacity: 0 },
        { opacity: 1, duration: 0.4, ease: "power2.out", delay: 0.2 });
    }, root);
    return () => ctx.revert();
  }, []);

  const update = (patch: Partial<Settings>) => {
    updateSettings(patch).then(setSettings);
  };

  const step = (field: IntervalField, delta: number, min: number, max: number) => {
    const next = Math.min(max, Math.max(min, settings[field] + delta));
    update({ [field]: next } as Partial<Settings>);
  };

  // 黑名单：点"添加"弹列表，从当前可见窗口中选一个加入。
  const openPicker = async () => {
    const list = await listVisibleWindows();
    // 标记已在黑名单中的，UI 灰显。
    setWindows(list);
    setPickerOpen(true);
  };

  const pickWindow = (w: WindowInfo) => {
    if (settings.fullscreen_blocklist.some((n) => n.toLowerCase() === w.process.toLowerCase())) {
      return; // 已在列表，忽略
    }
    update({ fullscreen_blocklist: [...settings.fullscreen_blocklist, w.process] });
    setPickerOpen(false);
  };

  const removeBlock = (name: string) => {
    update({ fullscreen_blocklist: settings.fullscreen_blocklist.filter((n) => n !== name) });
  };

  return (
    <>
    <div className="settings" ref={root}>
      <div className="settings__head">
        <h3 className="settings__title">Water Me · 设置</h3>
      </div>
      <div className="settings__body" onClick={() => { setEditing(null); setModeDropdownOpen(false); setHoverMode(null); }}>
        <div className="settings__scroll">
        <div className="settings__section">
          <h4 className="settings__section-title">提醒方式</h4>
          <div className="settings__section-rule" />
          <div className="settings__row" onClick={(e) => e.stopPropagation()}>
            <div className="settings__row-main">
              <div className="settings__row-title">弹窗形态</div>
              <div className="settings__row-desc">悬停选项可预览动画效果。</div>
            </div>
            <div className={"omode-dd" + (modeDropdownOpen ? " open" : "")}>
              <button
                ref={modeTriggerRef}
                className="omode-dd__trigger"
                onClick={() => {
                  const open = !modeDropdownOpen;
                  setModeDropdownOpen(open);
                  if (open && modeTriggerRef.current) {
                    const r = modeTriggerRef.current.getBoundingClientRect();
                    setDdPos({ top: r.bottom + 6, left: r.left, width: r.width });
                  }
                }}
                aria-haspopup="listbox"
                aria-expanded={modeDropdownOpen}
              >
                <span>{MODE_OPTIONS.find((m) => m.value === settings.overlay_mode)?.label ?? "全屏遮罩"}</span>
                <span className="omode-dd__caret">▾</span>
              </button>
            </div>
          </div>
        </div>

        <div className="settings__section">
          <h4 className="settings__section-title">提醒</h4>
          <div className="settings__section-rule" />
          {INTERVAL_ROWS.map((row) => (
            <div
              key={row.field}
              className="settings__row"
              onClick={(e) => {
                e.stopPropagation();
                setEditing(editing === row.field ? null : row.field);
              }}
            >
              <div className="settings__row-main">
                <div className="settings__row-title">{row.title}</div>
                <div className="settings__row-desc">{row.desc}</div>
              </div>
              {editing === row.field ? (
                <div className="stepper" onClick={(e) => e.stopPropagation()}>
                  <button
                    className="stepper__btn"
                    disabled={settings[row.field] <= row.min}
                    onClick={() => step(row.field, -row.step, row.min, row.max)}
                  >
                    −
                  </button>
                  <span className="stepper__value">{settings[row.field]} min</span>
                  <button
                    className="stepper__btn"
                    disabled={settings[row.field] >= row.max}
                    onClick={() => step(row.field, row.step, row.min, row.max)}
                  >
                    +
                  </button>
                </div>
              ) : (
                <div className="settings__row-value">
                  {settings[row.field]} min <span className="settings__row-value-arrow">▸</span>
                </div>
              )}
            </div>
          ))}
        </div>

        <div className="settings__section">
          <h4 className="settings__section-title">系统</h4>
          <div className="settings__section-rule" />
          <div className="settings__row" onClick={(e) => e.stopPropagation()}>
            <div className="settings__row-main">
              <div className="settings__row-title">主题</div>
              <div className="settings__row-desc">跟随系统、浅色或深色。</div>
            </div>
            <div className="theme-seg">
              {(["system", "light", "dark"] as const).map((t) => (
                <button
                  key={t}
                  className={"theme-seg__btn" + (settings.theme === t ? " is-active" : "")}
                  onClick={() => update({ theme: t })}
                >
                  {t === "system" ? "系统" : t === "light" ? "浅色" : "深色"}
                </button>
              ))}
            </div>
          </div>
          <div className="settings__row" onClick={(e) => e.stopPropagation()}>
            <div className="settings__row-main">
              <div className="settings__row-title">开机自动启动</div>
            </div>
            <button
              className="switch"
              role="switch"
              aria-checked={settings.autostart}
              onClick={() => update({ autostart: !settings.autostart })}
            />
          </div>
          <div className="settings__row" onClick={(e) => e.stopPropagation()}>
            <div className="settings__row-main">
              <div className="settings__row-title">专注免打扰</div>
              <div className="settings__row-desc">开启后，黑名单内应用在前台时静默提醒，不限全屏。</div>
            </div>
            <button
              className="switch"
              role="switch"
              aria-checked={settings.fullscreen_reminder}
              onClick={() => update({ fullscreen_reminder: !settings.fullscreen_reminder })}
            />
          </div>
          {settings.fullscreen_reminder && (
            <div className="blocklist">
              <div className="blocklist__label">这些应用在前台时不提醒</div>
              {settings.fullscreen_blocklist.length === 0 ? (
                <div className="blocklist__empty">列表为空，照常提醒</div>
              ) : (
                <div className="blocklist__items">
                  {settings.fullscreen_blocklist.map((name) => (
                    <div key={name} className="blocklist__item">
                      <span className="blocklist__name">{name}</span>
                      <button
                        className="blocklist__remove"
                        onClick={() => removeBlock(name)}
                        aria-label={`移除 ${name}`}
                      >
                        ×
                      </button>
                    </div>
                  ))}
                </div>
              )}
              <button className="blocklist__add" onClick={openPicker}>
                + 添加应用
              </button>
              {pickerOpen && (
                <div className="picker">
                  <div className="picker__header">
                    <span>选择要加入黑名单的应用</span>
                    <button className="picker__close" onClick={() => setPickerOpen(false)}>×</button>
                  </div>
                  <div className="picker__list">
                    {windows.length === 0 ? (
                      <div className="picker__empty">没有检测到其他可见窗口</div>
                    ) : (
                      windows.map((w) => {
                        const blocked = settings.fullscreen_blocklist.some(
                          (n) => n.toLowerCase() === w.process.toLowerCase()
                        );
                        return (
                          <button
                            key={w.process}
                            className="picker__item"
                            disabled={blocked}
                            onClick={() => pickWindow(w)}
                          >
                            <span className="picker__item-title">{w.title}</span>
                            <span className="picker__item-process">
                              {blocked ? "已加入" : w.process}
                            </span>
                          </button>
                        );
                      })
                    )}
                  </div>
                </div>
              )}
            </div>
          )}
        </div>
        </div>
      </div>
      <div className="settings__foot">
        <span>v0.1.0</span>
        <span>隐私承诺：不联网 · 不上传</span>
      </div>
    </div>
    {modeDropdownOpen &&
      createPortal(
          <div
            className="omode-dd__panel"
            role="listbox"
            style={{ position: "fixed", top: ddPos.top, left: ddPos.left, width: ddPos.width }}
          >
            {MODE_OPTIONS.map((m) => (
              <div
                key={m.value}
                className={"omode-dd__option" + (settings.overlay_mode === m.value ? " is-selected" : "")}
                role="option"
                aria-selected={settings.overlay_mode === m.value}
                onMouseEnter={() => setHoverMode(m.value)}
                onMouseLeave={() => setHoverMode(null)}
                onClick={() => {
                  update({ overlay_mode: m.value });
                  setModeDropdownOpen(false);
                  setHoverMode(null);
                }}
              >
                <span>{m.label}</span>
                <span className="omode-dd__check">✓</span>

                {/* hover 预览气泡：左侧弹出 */}
                {hoverMode === m.value && (
                  <div className="omode-pop">
                    <div className="omode-pop__title">
                      <span>{m.label} · {m.desc}</span>
                      <span className="omode-pop__tag">{m.tag}</span>
                    </div>
                    <div className="omode-mini">
                      <div className="omode-mini__fake-win" />
                      <div className="omode-mini__bar" />
                      <ModePreview mode={m.value} />
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>,
          document.body
        )}
    </>
  );
}

// ============================================
// ModePreview — 气泡内的迷你动画预览
// 循环播放对应形态的入场动画，让用户悬停时直观看效果。
// ponytail: 纯 CSS keyframes，3s 循环，无状态。
// ============================================
function ModePreview({ mode }: { mode: OverlayMode }) {
  if (mode === "toast") {
    return (
      <div className="omode-mini__toast">
        <div className="omode-mini__toast-row">
          <div className="omode-mini__toast-icon">💧</div>
          <div>
            <div className="omode-mini__toast-title">该喝水了</div>
            <div className="omode-mini__toast-sub">15:10 — 15:55</div>
          </div>
        </div>
        <div className="omode-mini__toast-progress" />
      </div>
    );
  }
  if (mode === "card") {
    return (
      <div className="omode-mini__cardwin">
        <div className="omode-mini__cardwin-head">
          <div className="omode-mini__cardwin-icon">💧</div>
          <div className="omode-mini__cardwin-title">该喝水了</div>
        </div>
        <div className="omode-mini__cardwin-body">去接一杯水</div>
        <div className="omode-mini__cardwin-actions">
          <span className="omode-mini__cardwin-btn ghost">跳过</span>
          <span className="omode-mini__cardwin-btn">我喝了</span>
        </div>
      </div>
    );
  }
  // fullscreen
  return (
    <div className="omode-mini__overlay">
      <div className="omode-mini__overlay-card">
        <div className="omode-mini__overlay-icon">💧🧍</div>
        <div className="omode-mini__overlay-title">去接杯水</div>
        <div className="omode-mini__overlay-bar" />
        <div className="omode-mini__overlay-btn">我喝了</div>
      </div>
    </div>
  );
}
