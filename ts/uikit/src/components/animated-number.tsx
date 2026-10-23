"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import {
  digitGlyphs,
  mergeItems,
  resolveDirection,
  splitNumber,
  type NumberItem,
  type RollDirection,
} from "../lib/number-parts";
import { useReducedMotion } from "../primitives/use-reduced-motion";
import { EDGE_MS, SPIN_MS, Slot } from "./animated-number-columns";
import { useCountTween } from "./use-count-tween";

export interface AnimatedNumberProps
  extends Omit<React.HTMLAttributes<HTMLSpanElement>, "children" | "prefix"> {
  value: number;
  /** Passed to `Intl.NumberFormat`: currency, percent, compact, `signDisplay`... */
  format?: Intl.NumberFormatOptions;
  locales?: Intl.LocalesArgument;
  /** `roll`: digit columns spin. `count`: the number is tweened and re-formatted each frame. Default `roll`. */
  mode?: "roll" | "count";
  /**
   * Roll only. `1` every column spins up, `-1` down, `0` each column takes its
   * shorter way. Default: up when the value rose, down when it fell.
   */
  trend?: RollDirection;
  /** Milliseconds one change takes: the spin in `roll` (default 750), the whole tween in `count` (default 800). */
  duration?: number;
  prefix?: React.ReactNode;
  suffix?: React.ReactNode;
  /** Announce each new value politely. Off by default: a ticker would talk over the page. */
  live?: boolean;
  /** Animate the first render from `from`. The server markup still prints `value`. */
  animateOnMount?: boolean;
  /** The first-render start with `animateOnMount`. Default `0`. */
  from?: number;
}

/**
 * A number that animates when `value` changes. `roll` (default) spins each
 * digit column like an odometer; `count` tweens the number itself. Both format
 * with `Intl.NumberFormat`, inherit font and colour, and swap instantly under
 * `prefers-reduced-motion`. The final formatted value is in the DOM once, as
 * text for assistive tech and for copying; the moving glyphs are `aria-hidden`.
 */
export function AnimatedNumber({
  value,
  format,
  locales,
  mode = "roll",
  trend,
  duration,
  prefix,
  suffix,
  live = false,
  animateOnMount = false,
  from = 0,
  className,
  style,
  ...props
}: AnimatedNumberProps) {
  // Callers pass `format` inline; keying on its content keeps one formatter per distinct options.
  const formatKey = JSON.stringify(format ?? null);
  const localeKey = Array.isArray(locales) ? locales.join(",") : String(locales ?? "");
  const formatter = React.useMemo(
    () => new Intl.NumberFormat(locales, format),
    [localeKey, formatKey],
  );
  const text = formatter.format(value);
  return (
    <span
      data-slot="animated-number"
      data-mode={mode}
      className={cn("relative inline-flex items-baseline tabular-nums", className)}
      style={style}
      {...props}
    >
      {prefix}
      <span
        data-slot="animated-number-text"
        className="sr-only"
        aria-live={live ? "polite" : undefined}
        aria-atomic={live ? true : undefined}
      >
        {text}
      </span>
      <span aria-hidden className="inline-flex select-none items-baseline">
        {mode === "count" ? (
          <CountView
            value={value}
            formatter={formatter}
            from={from}
            animateOnMount={animateOnMount}
            {...(duration === undefined ? {} : { duration })}
          />
        ) : (
          <RollView
            value={value}
            formatter={formatter}
            from={from}
            animateOnMount={animateOnMount}
            spin={duration ?? SPIN_MS}
            {...(trend === undefined ? {} : { trend })}
          />
        )}
      </span>
      {suffix}
    </span>
  );
}

interface CountViewProps {
  value: number;
  formatter: Intl.NumberFormat;
  from: number;
  animateOnMount: boolean;
  duration?: number;
}

function CountView({ value, formatter, from, animateOnMount, duration }: CountViewProps) {
  const shown = useCountTween(value, {
    from,
    animateOnMount,
    ...(duration === undefined ? {} : { duration }),
  });
  return <span>{formatter.format(shown)}</span>;
}

interface RollState {
  formatter: Intl.NumberFormat;
  value: number;
  direction: RollDirection;
  items: NumberItem[];
}

interface RollViewProps {
  value: number;
  formatter: Intl.NumberFormat;
  trend?: RollDirection;
  spin: number;
  from: number;
  animateOnMount: boolean;
}

// With `animateOnMount` the first client commit shows `from` and the value
// arrives two frames later: one frame is not enough for the browser to have
// painted `from` and so have a start for the transition.
function useMountLead(enabled: boolean): boolean {
  const [lead, setLead] = React.useState(false);
  const started = React.useRef(false);
  React.useLayoutEffect(() => {
    if (!enabled || started.current) return;
    started.current = true;
    setLead(true);
    let inner = 0;
    const outer = requestAnimationFrame(() => {
      inner = requestAnimationFrame(() => setLead(false));
    });
    return () => {
      cancelAnimationFrame(outer);
      cancelAnimationFrame(inner);
      started.current = false;
      setLead(false);
    };
  }, [enabled]);
  return lead;
}

function RollView({ value, formatter, trend, spin, from, animateOnMount }: RollViewProps) {
  const motion = !useReducedMotion();
  const lead = useMountLead(animateOnMount && motion);
  const target = lead ? from : value;
  const glyphs = React.useMemo(() => digitGlyphs(formatter), [formatter]);
  const [state, setState] = React.useState<RollState>(() => ({
    formatter,
    value: target,
    direction: 0,
    items: splitNumber(formatter, target, glyphs).map((part) => ({ part, exiting: false, fresh: false })),
  }));

  let current = state;
  if (state.formatter !== formatter || state.value !== target) {
    const sameFormat = state.formatter === formatter;
    // `lead` is a jump to `from`, not a change the reader should watch.
    const animate = motion && sameFormat && !lead;
    current = {
      formatter,
      value: target,
      direction: sameFormat ? resolveDirection(trend, state.value, target) : 0,
      items: mergeItems(state.items, splitNumber(formatter, target, glyphs), animate),
    };
    setState(current);
  }

  const leaving = current.items.some((item) => item.exiting);
  React.useEffect(() => {
    if (!leaving) return;
    const id = setTimeout(
      () => setState((s) => ({ ...s, items: s.items.filter((item) => !item.exiting) })),
      EDGE_MS + 50,
    );
    return () => clearTimeout(id);
  }, [leaving, current.items]);

  return (
    <>
      {current.items.map((item) => (
        <Slot
          key={item.part.key}
          item={item}
          glyphs={glyphs}
          direction={current.direction}
          motion={motion}
          spin={spin}
        />
      ))}
    </>
  );
}
