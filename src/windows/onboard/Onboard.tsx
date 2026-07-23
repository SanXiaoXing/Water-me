// Onboard 首次启动引导。两步：欢迎+隐私承诺 / 参数配置+自启开关。
// See docs/05-UIUX.md §6.2, docs/01-PRD.md FR-074~078。
import { useEffect, useRef, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import gsap from "gsap";

import { Plant } from "../../Plant";
import { completeOnboard, type Settings } from "../../lib/ipc";

// 各间隔的取值范围（与 Settings 一致）。默认值即 state 初值。
const INTERVALS = {
  water: { min: 15, max: 180, step: 5, def: 60 },
  stand: { min: 10, max: 120, step: 5, def: 45 },
  idle: { min: 1, max: 30, step: 1, def: 5 },
};

/** 内联步进器。Onboard 与 Settings 各自维护一份，避免过早抽象。 */
function Stepper({
  value,
  onChange,
  min,
  max,
  step,
}: {
  value: number;
  onChange: (v: number) => void;
  min: number;
  max: number;
  step: number;
}) {
  return (
    <div className="stepper">
      <button
        className="stepper__btn"
        disabled={value <= min}
        onClick={() => onChange(Math.max(min, value - step))}
      >
        −
      </button>
      <span className="stepper__value">{value} min</span>
      <button
        className="stepper__btn"
        disabled={value >= max}
        onClick={() => onChange(Math.min(max, value + step))}
      >
        +
      </button>
    </div>
  );
}

export function Onboard() {
  const [step, setStep] = useState<0 | 1>(0);
  const [autostart, setAutostart] = useState(false);
  const [water, setWater] = useState(INTERVALS.water.def);
  const [stand, setStand] = useState(INTERVALS.stand.def);
  const [idle, setIdle] = useState(INTERVALS.idle.def);
  const root = useRef<HTMLDivElement>(null);

  // 步骤切换/首挂载入场：main 整体上移淡入，内部子元素错落，footer 稍后淡入。
  // 用 fromTo 避免 from 的首帧闪烁；ctx.revert() 在卸载/重跑时还原 inline 样式。
  // 用 useEffect 而非 useLayoutEffect：先让窗口快速渲染静态内容，再异步跑动画，
  // 避免动画计算阻塞首次绘制造成卡顿感。
  useEffect(() => {
    const ctx = gsap.context(() => {
      gsap.fromTo(".onboard__main",
        { opacity: 0, y: 18 },
        { opacity: 1, y: 0, duration: 0.42, ease: "power3.out" });
      gsap.fromTo(".onboard__main > *",
        { opacity: 0, y: 14 },
        { opacity: 1, y: 0, duration: 0.38, ease: "power3.out", stagger: 0.06, delay: 0.05 });
      gsap.fromTo(".onboard__footer",
        { opacity: 0 },
        { opacity: 1, duration: 0.42, ease: "power2.out", delay: 0.12 });
    }, root);
    return () => ctx.revert();
  }, [step]);

  const finish = () => {
    // 把用户选定的间隔一并写入；first_launch 由后端置 false。
    completeOnboard({
      autostart,
      water_interval_min: water,
      stand_interval_min: stand,
      idle_threshold_min: idle,
    } as Partial<Settings>).then(() => {
      getCurrentWindow().close();
    });
  };

  return (
    <div className="onboard" ref={root}>
      {step === 0 ? (
        <>
          <div className="onboard__main" key="step-0">
            <div className="onboard__head">
              <div className="onboard__plant">
                <Plant variant="normal" size={120} />
              </div>
              <h2 className="onboard__title">Water Me</h2>
              <p className="onboard__sub">Humans need watering, too.</p>
            </div>
            <div className="onboard__divider" />
            <p className="onboard__body">我会在你工作太久的时候，轻轻提醒你照顾自己。</p>
            <div className="onboard__privacy">
              <h4>隐私承诺</h4>
              <ul>
                <li>不联网</li>
                <li>不上传任何数据</li>
                <li>不记录键盘内容</li>
                <li>不记录鼠标坐标</li>
                <li>只监测"有没有在用电脑"</li>
              </ul>
            </div>
          </div>
          <div className="onboard__footer">
            <div className="onboard__dots">
              <span className="onboard__dot onboard__dot--active" />
              <span className="onboard__dot" />
            </div>
            <button className="btn btn--primary" onClick={() => setStep(1)}>
              下一步 →
            </button>
          </div>
        </>
      ) : (
        <>
          <div className="onboard__main" key="step-1">
            <div className="onboard__head">
              <h2 className="onboard__title">
                已经准备好陪你。
              </h2>
              <p className="onboard__sub">调整成你觉得舒服的节奏</p>
            </div>
            <div className="onboard__divider" />
            <div className="onboard__setting">
              <div>
                <span className="onboard__setting-label">喝水间隔</span>
                <span className="onboard__setting-desc">每工作多久提醒一次。</span>
              </div>
              <Stepper
                value={water}
                onChange={setWater}
                min={INTERVALS.water.min}
                max={INTERVALS.water.max}
                step={INTERVALS.water.step}
              />
            </div>
            <div className="onboard__setting">
              <div>
                <span className="onboard__setting-label">站立间隔</span>
                <span className="onboard__setting-desc">每工作多久提醒一次。</span>
              </div>
              <Stepper
                value={stand}
                onChange={setStand}
                min={INTERVALS.stand.min}
                max={INTERVALS.stand.max}
                step={INTERVALS.stand.step}
              />
            </div>
            <div className="onboard__setting">
              <div>
                <span className="onboard__setting-label">闲置暂停</span>
                <span className="onboard__setting-desc">多久没动就暂停计时。</span>
              </div>
              <Stepper
                value={idle}
                onChange={setIdle}
                min={INTERVALS.idle.min}
                max={INTERVALS.idle.max}
                step={INTERVALS.idle.step}
              />
            </div>
            <div className="onboard__setting">
              <div>
                <span className="onboard__setting-label">开机自动启动</span>
              </div>
              <button
                className="switch"
                role="switch"
                aria-checked={autostart}
                onClick={() => setAutostart((v) => !v)}
              />
            </div>
          </div>
          <div className="onboard__footer">
            <div className="onboard__dots">
              <span className="onboard__dot" />
              <span className="onboard__dot onboard__dot--active" />
            </div>
            <button className="btn btn--tertiary" onClick={() => setStep(0)}>
              ← 上一步
            </button>
            <button className="btn btn--primary" onClick={finish}>
              开始使用
            </button>
          </div>
        </>
      )}
    </div>
  );
}
