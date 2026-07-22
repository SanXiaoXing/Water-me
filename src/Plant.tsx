// 植物插画。使用 Lottie 动画（src/assets/*.json），替代手写 SVG。
// 三种情绪变体：normal / thirsty / done。
// See docs/05-UIUX.md §6 组件库。
//
// ponytail: 直接用 lottie-web（Lottie 官方实现），单一依赖 + ~15 行胶水，
// 不引入 React 封装库。动画源是 757×1016 竖版，容器按源宽高比撑开避免拉伸。

import { useEffect, useRef } from "react";
import lottie, { type AnimationItem } from "lottie-web";

import plantNormal from "./assets/Plant.json";
import plantThirsty from "./assets/Plant_wilt.json";
import plantDone from "./assets/Plant_watered.json";

type PlantVariant = "normal" | "thirsty" | "done";

const DATA: Record<PlantVariant, unknown> = {
  normal: plantNormal,
  thirsty: plantThirsty,
  done: plantDone,
};

export function Plant({
  variant = "normal",
  size = 160,
}: {
  variant?: PlantVariant;
  size?: number;
}) {
  const ref = useRef<HTMLDivElement>(null);
  const anim = useRef<AnimationItem | null>(null);

  useEffect(() => {
    if (!ref.current) return;
    anim.current = lottie.loadAnimation({
      container: ref.current,
      renderer: "svg",
      loop: true,
      autoplay: true,
      animationData: DATA[variant],
    });
    return () => {
      anim.current?.destroy();
      anim.current = null;
    };
  }, [variant]);

  // 源宽高比 757×1016，按 size 为宽度撑开高度，避免拉伸变形。
  return <div ref={ref} style={{ width: size, height: size * (1016 / 757) }} />;
}
