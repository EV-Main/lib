"use client";

import * as React from "react";
import { useReducedMotion } from "../primitives/use-reduced-motion";

export interface CountTweenOptions {
  /** Milliseconds from the old value to the new one. Default `800`. */
  duration?: number;
  /** Maps progress 0..1 to eased progress 0..1. Default ease-out cubic. */
  easing?: (progress: number) => number;
  /** Count up from here on the first render. Default `0`; only read with `animateOnMount`. */
  from?: number;
  /** Tween on the first render too. Off by default so the server markup is the final value. */
  animateOnMount?: boolean;
}

const easeOutCubic = (p: number): number => 1 - (1 - p) ** 3;

/**
 * The number a count-up is showing right now, for custom renderers (charts,
 * labels). It returns `value` itself on the first render and whenever nothing
 * is in flight, so a server render prints the final figure. A new `value`
 * mid-flight restarts from what is on screen. Reduced motion returns `value`
 * with no frames at all.
 */
export function useCountTween(value: number, options: CountTweenOptions = {}): number {
  const { duration = 800, easing = easeOutCubic, from = 0, animateOnMount = false } = options;
  const reduced = useReducedMotion();
  const [shown, setShown] = React.useState(value);
  // The frame loop reads the on-screen value without being a dependency of itself.
  const onScreen = React.useRef(value);
  const mounted = React.useRef(false);
  const easingRef = React.useRef(easing);
  easingRef.current = easing;

  React.useLayoutEffect(() => {
    const first = !mounted.current;
    mounted.current = true;
    const start = first ? (animateOnMount ? from : value) : onScreen.current;
    const paint = (v: number) => {
      onScreen.current = v;
      setShown(v);
    };
    if (reduced || duration <= 0 || start === value || !Number.isFinite(start) || !Number.isFinite(value)) {
      paint(value);
      return;
    }
    paint(start);
    let frame = 0;
    let t0: number | null = null;
    const tick = (now: number) => {
      t0 ??= now;
      const p = Math.min(1, (now - t0) / duration);
      paint(p >= 1 ? value : start + (value - start) * easingRef.current(p));
      if (p < 1) frame = requestAnimationFrame(tick);
    };
    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
  }, [value, duration, reduced, animateOnMount, from]);

  return reduced ? value : shown;
}
