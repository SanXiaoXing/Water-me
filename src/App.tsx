// Water Me — 窗口路由。无主窗口架构：按当前 webview label 渲染对应组件。
// See docs/02-Architecture.md §2（窗口模型）。
import { getCurrentWindow } from "@tauri-apps/api/window";

import { Overlay } from "./Overlay";
import { Onboard } from "./Onboard";
import { Settings } from "./Settings";
import "./App.css";

const label = getCurrentWindow().label;

export default function App() {
  if (label === "overlay") return <Overlay />;
  if (label === "onboard") return <Onboard />;
  if (label === "settings") return <Settings />;
  // 兜底：无主窗口，理论上不会进入。返回空容器避免崩溃。
  return <div />;
}
