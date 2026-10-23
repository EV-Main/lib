"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import {
  digitGlyphs,
  mergeItems,
  resolveDirection,
  rollStep,
  splitNumber,
  type NumberItem,
  type RollDirection,
} from "../lib/number-parts";
import { useReducedMotion } from "../primitives/use-reduced-motion";
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

// The JS side has to know when an entering or leaving column is done, so these
// are literals rather than `--ev-dur-*` (400ms is the nearest token).
const EDGE_MS = 350;
const EDGE_EASE = "var(--ev-ease-out, cubic-bezier(0.22, 1, 0.36, 1))";
const SPIN_MS = 750;
// A damped spring (zeta .72, ~4% overshoot) sampled for `linear()`.
const SPRING =
  "linear(0, 0.097, 0.301, 0.521, 0.712, 0.855, 0.95, 1.005, 1.031, 1.038, 1.035, 1.028, 1.019, 1.012, 1.006, 1.002, 1, 0.999, 1, 1, 1)";
const WINDOW_EDGE = "0.25em";
const WINDOW_MASK = `linear-gradient(to bottom, transparent 0, black ${WINDOW_EDGE}, black calc(100% - ${WINDOW_EDGE}), transparent 100%)`;

function spinEasing(): string {
  const springy = typeof CSS !== "undefined" && typeof CSS.supports === "function" && CSS.supports("transition-timing-function", SPRING);
  return springy ? SPRING : EDGE_EASE;
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

interface SlotProps {
  item: NumberItem;
  glyphs: readonly string[];
  direction: RollDirection;
  motion: boolean;
  spin: number;
}

/** One part of the number; collapses to zero width to enter or leave, so its neighbours slide. */
function Slot({ item, glyphs, direction, motion, spin }: SlotProps) {
  const ref = React.useRef<HTMLSpanElement>(null);
  const [open, setOpen] = React.useState(!item.fresh);
  React.useLayoutEffect(() => {
    if (open) return;
    // Reading layout makes the browser style the collapsed state, so opening it is a transition.
    void ref.current?.offsetWidth;
    setOpen(true);
  }, [open]);
  const collapsed = item.exiting || !open;
  const { part } = item;
  return (
    <span
      ref={ref}
      data-slot="animated-number-part"
      data-state={collapsed ? "collapsed" : "open"}
      style={{
        display: "inline-grid",
        gridTemplateColumns: collapsed ? "0fr" : "1fr",
        opacity: collapsed ? 0 : 1,
        transition: motion
          ? `grid-template-columns ${EDGE_MS}ms ${EDGE_EASE}, opacity ${EDGE_MS}ms ${EDGE_EASE}`
          : undefined,
      }}
    >
      <span style={{ minWidth: 0, overflowX: "clip", whiteSpace: "pre" }}>
        {part.kind === "digit" ? (
          <Digit digit={part.digit} glyphs={glyphs} direction={direction} motion={motion} spin={spin} />
        ) : (
          part.text
        )}
      </span>
    </span>
  );
}

interface SpinState {
  digit: number;
  /** Index into the unbounded strip; the glyph at index i is `i mod 10`. */
  at: number;
  lo: number;
  hi: number;
  animate: boolean;
  /** Sign of the last move, for `data-roll`. */
  step: number;
}

const settle = (digit: number): SpinState => ({ digit, at: digit, lo: digit, hi: digit, animate: false, step: 0 });

const glyphIndex = (i: number): number => ((i % 10) + 10) % 10;

interface DigitProps {
  digit: number;
  glyphs: readonly string[];
  direction: RollDirection;
  motion: boolean;
  spin: number;
}

/**
 * A clipped one-line window over a strip of digits. The strip is unbounded,
 * so a forced direction wraps (9 -> 0 going up lands on index 10) and a column
 * never rewinds through the digits in between. Only the cells between where it
 * was and where it is going exist in the DOM; when the transition ends it is
 * rebased to 0..9 without a transition.
 */
function Digit({ digit, glyphs, direction, motion, spin }: DigitProps) {
  const [state, setState] = React.useState<SpinState>(() => settle(digit));
  let s = state;
  if (s.digit !== digit) {
    if (motion) {
      const step = rollStep(s.digit, digit, direction);
      const at = s.at + step;
      s = { digit, at, lo: Math.min(s.lo, at), hi: Math.max(s.hi, at), animate: true, step };
    } else {
      s = settle(digit);
    }
    setState(s);
  }
  const moving = s.lo !== s.hi;
  const cells = Array.from({ length: s.hi - s.lo + 1 }, (_, n) => s.lo + n);
  return (
    <span
      data-slot="animated-number-digit"
      data-roll={moving ? (s.step > 0 ? "up" : "down") : "rest"}
      style={{
        position: "relative",
        display: "inline-block",
        overflow: "clip",
        paddingBlock: WINDOW_EDGE,
        marginBlock: `-${WINDOW_EDGE}`,
        ...(moving ? { maskImage: WINDOW_MASK } : {}),
      }}
    >
      <span style={{ display: "block", visibility: "hidden" }}>{glyphs[0]}</span>
      <span
        onTransitionEnd={(e) => {
          if (e.target === e.currentTarget && e.propertyName === "transform") setState(settle(digit));
        }}
        style={{
          position: "absolute",
          insetInline: 0,
          top: WINDOW_EDGE,
          transform: `translateY(calc(${-s.at} * 1lh))`,
          transition: s.animate ? `transform ${spin}ms ${spinEasing()}` : "none",
        }}
      >
        {cells.map((i) => (
          <span
            key={i}
            style={{ position: "absolute", insetInline: 0, top: `calc(${i} * 1lh)`, height: "1lh", textAlign: "center" }}
          >
            {glyphs[glyphIndex(i)]}
          </span>
        ))}
      </span>
    </span>
  );
}
