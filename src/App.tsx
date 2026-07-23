// Water Me — 窗口路由。无主窗口架构：按当前 webview label 渲染对应组件。
// See docs/02-Architecture.md §2（窗口模型）。
import { useEffect } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

import { Overlay } from "./windows/overlay/Overlay";
import { Onboard } from "./windows/onboard/Onboard";
import { Settings } from "./windows/settings/Settings";
import { applyTheme, getSettings, onSettingsChanged } from "./lib/ipc";
import "./App.css";

const label = getCurrentWindow().label;

export default function App() {
  // 主题：挂载时读 settings 应用，监听变更同步（多窗口一致）。
  useEffect(() => {
    getSettings().then((s) => applyTheme(s.theme));
    const unlistenP = onSettingsChanged((s) => applyTheme(s.theme));
    return () => { unlistenP.then((f) => f()); };
  }, []);

  if (label === "overlay") return <Overlay />;
  if (label === "onboard") return <Onboard />;
  if (label === "settings") return <Settings />;
  // 兜底：无主窗口，理论上不会进入。返回空容器避免崩溃。
  return <div />;
}
