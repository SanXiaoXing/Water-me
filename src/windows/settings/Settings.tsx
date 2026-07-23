// Settings 设置窗口。手账布局：提醒间隔（点击行内编辑）+ 系统开关。
// See docs/05-UIUX.md §6.3, docs/01-PRD.md FR-047~056/064。
import { useEffect, useLayoutEffect, useRef, useState } from "react";
import gsap from "gsap";

import {
  getSettings,
  onSettingsChanged,
  updateSettings,
  type Settings,
} from "../../lib/ipc";

const DEFAULTS: Settings = {
  version: 1,
  water_interval_min: 60,
  stand_interval_min: 45,
  idle_threshold_min: 5,
  snooze_interval_min: 10,
  autostart: false,
  fullscreen_reminder: true,
  first_launch: false,
  paused: false,
};

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
  useLayoutEffect(() => {
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

  return (
    <div className="settings" ref={root}>
      <div className="settings__head">
        <h3 className="settings__title">Water Me · 设置</h3>
      </div>
      <div className="settings__body" onClick={() => setEditing(null)}>
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
              <div className="settings__row-title">全屏提醒</div>
              <div className="settings__row-desc">游戏中/演示中默认不打扰。</div>
            </div>
            <button
              className="switch"
              role="switch"
              aria-checked={settings.fullscreen_reminder}
              onClick={() => update({ fullscreen_reminder: !settings.fullscreen_reminder })}
            />
          </div>
        </div>
      </div>
      <div className="settings__foot">
        <span>v0.1.0</span>
        <span>隐私承诺：不联网 · 不上传</span>
      </div>
    </div>
  );
}
