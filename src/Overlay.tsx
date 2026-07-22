// Overlay 提醒窗口。全屏半透明罩 + 居中 600×420 植物志卡片。
// See docs/05-UIUX.md §6.1, docs/01-PRD.md FR-033~041。
import { useEffect, useState } from "react";

import { Plant } from "./Plant";
import {
  getCurrentReminder,
  onReminderTriggered,
  reminderComplete,
  reminderDefer,
  reminderSkip,
  type ReminderTriggered,
} from "./lib/ipc";

export function Overlay() {
  const [payload, setPayload] = useState<ReminderTriggered | null>(null);

  useEffect(() => {
    // 挂载时拉取当前载荷（避免错过挂载前发出的事件），再监听后续事件。
    getCurrentReminder().then(setPayload);
    const unlistenP = onReminderTriggered(setPayload);
    return () => {
      unlistenP.then((f) => f());
    };
  }, []);

  if (!payload) return null;

  const ids = payload.activities.map((a) => a.id);
  const typeLabel = payload.activities.map((a) => `${a.icon} ${a.id}`).join(" · ");

  // 日期展示：本地时间。
  const now = new Date();
  const pad = (n: number) => String(n).padStart(2, "0");
  const dateStr = `Water Me · ${now.getFullYear()}.${pad(now.getMonth() + 1)}.${pad(now.getDate())} ${pad(now.getHours())}:${pad(now.getMinutes())}`;

  // ponytail: "10 分钟后"文案用字面量；defer 命令在 Rust 侧按实际 snooze_interval_min 计算，
  // 默认值即为 10，与文案一致。用户改了 snooze 后文案不随之变——V1 可接受。
  return (
    <div className="overlay-root">
      <div className="overlay-card stagger">
        <span className="overlay-card__date">{dateStr}</span>
        <span className="overlay-card__type">{typeLabel}</span>
        <div className="overlay-card__plant">
          <Plant variant="thirsty" size={160} />
        </div>
        <h3 className="overlay-card__title">{payload.title}</h3>
        <p className="overlay-card__sub">{payload.title_en}</p>
        <div className="overlay-card__divider">
          <span className="overlay-card__duration">{payload.duration_min} min</span>
        </div>
        <div className="overlay-card__actions">
          <button
            className="btn btn--primary btn--pulse"
            onClick={() => reminderComplete(ids)}
          >
            {payload.activities[0]?.action ?? "完成"}
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
