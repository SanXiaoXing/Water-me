// Overlay 提醒窗口。按 settings.overlay_mode 渲染三种形态：
// - fullscreen：全屏半透明罩 + 居中植物志卡片（强阻断）
// - card：居中独立卡片（无边框透明窗）
// - toast：右下角通知条（8 秒倒计时后自动跳过）
// See docs/05-UIUX.md §6.1, docs/01-PRD.md FR-033~041。
// 形态原型：docs/design/overlay-mode-picker.html
import { useEffect, useRef, useState } from "react";
import gsap from "gsap";

import { Plant } from "../../Plant";
import {
  getCurrentReminder,
  getSettings,
  onReminderTriggered,
  reminderComplete,
  reminderDefer,
  reminderSkip,
  type OverlayMode,
  type ReminderTriggered,
} from "../../lib/ipc";

const pad = (n: number) => String(n).padStart(2, "0");
const fmtDate = (d: Date) =>
  `Water Me · ${d.getFullYear()}.${pad(d.getMonth() + 1)}.${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
const fmtTime = (d: Date) => `${pad(d.getHours())}:${pad(d.getMinutes())}`;

export function Overlay() {
  const [payload, setPayload] = useState<ReminderTriggered | null>(null);
  const [mode, setMode] = useState<OverlayMode>("fullscreen");
  const root = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // 挂载时拉取当前载荷与设置中的弹窗形态（避免错过挂载前发出的事件）。
    getCurrentReminder().then(setPayload);
    getSettings().then((s) => setMode(s.overlay_mode));
    const unlistenPayload = onReminderTriggered(setPayload);
    return () => {
      unlistenPayload.then((f) => f());
    };
  }, []);

  // 入场动画（各形态不同）。用 useEffect 而非 useLayoutEffect：先让窗口快速渲染静态内容，
  // 再异步跑动画，避免动画计算阻塞首次绘制造成卡顿感。
  useEffect(() => {
    if (!payload || !root.current) return;
    const ctx = gsap.context(() => {
      if (mode === "toast") {
        gsap.fromTo(".toast",
          { opacity: 0, x: 24 },
          { opacity: 1, x: 0, duration: 0.45, ease: "power3.out" });
        gsap.fromTo(".toast > *",
          { opacity: 0, y: 6 },
          { opacity: 1, y: 0, duration: 0.4, ease: "power3.out", stagger: 0.05, delay: 0.1 });
      } else if (mode === "card") {
        gsap.fromTo(".cardwin",
          { opacity: 0, scale: 0.92 },
          { opacity: 1, scale: 1, duration: 0.45, ease: "back.out(1.4)" });
        gsap.fromTo(".cardwin > *",
          { opacity: 0, y: 10 },
          { opacity: 1, y: 0, duration: 0.4, ease: "power3.out", stagger: 0.06, delay: 0.1 });
      } else {
        gsap.fromTo(".overlay-root",
          { opacity: 0 },
          { opacity: 1, duration: 0.3, ease: "power2.out" });
        gsap.fromTo(".overlay-card",
          { opacity: 0, y: 24, scale: 0.96 },
          { opacity: 1, y: 0, scale: 1, duration: 0.5, ease: "power3.out", delay: 0.08 });
        gsap.fromTo(".overlay-card > *",
          { opacity: 0, y: 12 },
          { opacity: 1, y: 0, duration: 0.4, ease: "power3.out", stagger: 0.06, delay: 0.16 });
      }
    }, root);
    return () => ctx.revert();
  }, [payload, mode]);

  // toast 模式：8 秒倒计时后自动 skip（用户没看到 = 跳过这次）。
  useEffect(() => {
    if (mode !== "toast" || !payload) return;
    const ids = payload.activities.map((a) => a.id);
    const timer = setTimeout(() => {
      reminderSkip(ids);
    }, 8000);
    return () => clearTimeout(timer);
  }, [mode, payload]);

  if (!payload) return null;

  const ids = payload.activities.map((a) => a.id);
  const typeLabel = payload.activities.map((a) => `${a.icon} ${a.id}`).join(" · ");
  const endDate = new Date(payload.triggered_at_iso);
  const startDate = new Date(payload.started_at_iso);
  const dateStr = fmtDate(endDate);
  const timeRange = `${fmtTime(startDate)} — ${fmtTime(endDate)}`;
  const actionLabel = payload.activities[0]?.action ?? "完成";

  if (mode === "toast") {
    return (
      <div className="overlay-root overlay-root--toast" ref={root}>
        <div className="toast">
          <div className="toast__row">
            <div className="toast__icon">{payload.activities[0]?.icon ?? "💧"}</div>
            <div className="toast__main">
              <h4 className="toast__title">{payload.title}</h4>
              <p className="toast__sub">{payload.title_en}</p>
              <p className="toast__time">{timeRange} · {payload.duration_min} min</p>
              <div className="toast__actions">
                <button className="btn btn--tertiary" onClick={() => reminderSkip(ids)}>忽略</button>
                <button className="btn btn--primary" onClick={() => reminderComplete(ids)}>
                  {actionLabel}
                </button>
              </div>
            </div>
          </div>
          <div className="toast__progress" />
        </div>
      </div>
    );
  }

  if (mode === "card") {
    return (
      <div className="overlay-root overlay-root--card" ref={root}>
        <div className="cardwin">
          <button className="cardwin__close" onClick={() => reminderSkip(ids)} title="关闭">×</button>
          <div className="cardwin__head">
            <div className="cardwin__icon">{payload.activities[0]?.icon ?? "💧"}</div>
            <div className="cardwin__head-text">
              <span className="cardwin__title">{payload.title}</span>
              <span className="cardwin__time">{timeRange} · {payload.duration_min} min</span>
            </div>
          </div>
          <div className="cardwin__plant">
            <Plant variant="thirsty" size={100} />
          </div>
          <p className="cardwin__body">
            {payload.activities[0]?.message ?? ""}
            <em>{payload.title_en}</em>
          </p>
          <div className="cardwin__actions">
            <button className="btn btn--tertiary" onClick={() => reminderSkip(ids)}>今天跳过</button>
            <button className="btn btn--secondary" onClick={() => reminderDefer(ids)}>稍后</button>
            <button className="btn btn--primary" onClick={() => reminderComplete(ids)}>
              {actionLabel}
            </button>
          </div>
        </div>
      </div>
    );
  }

  // 默认 fullscreen
  return (
    <div className="overlay-root" ref={root}>
      <div className="overlay-card">
        <span className="overlay-card__date">{dateStr}</span>
        <span className="overlay-card__type">{typeLabel}</span>
        <div className="overlay-card__plant">
          <Plant variant="thirsty" size={160} />
        </div>
        <h3 className="overlay-card__title">{payload.title}</h3>
        <p className="overlay-card__sub">{payload.title_en}</p>
        <div className="overlay-card__divider">
          <span className="overlay-card__time-range">
            <span>{fmtTime(startDate)}</span>
            <span className="overlay-card__time-sep">—</span>
            <span>{fmtTime(endDate)}</span>
          </span>
          <span className="overlay-card__duration">{payload.duration_min} min</span>
        </div>
        <div className="overlay-card__actions">
          <button
            className="btn btn--primary btn--pulse"
            onClick={() => reminderComplete(ids)}
          >
            {actionLabel}
          </button>
          <button className="btn btn--secondary" onClick={() => reminderDefer(ids)}>
            10 分钟后
          </button>
          <button className="btn btn--tertiary" onClick={() => reminderSkip(ids)}>
            今天跳过
          </button>
        </div>
      </div>
    </div>
  );
}
