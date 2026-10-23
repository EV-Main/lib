"use client";

import * as React from "react";
import type { NumberItem, RollDirection } from "../lib/number-parts";
import { rollStep } from "../lib/number-parts";

// The JS side has to know when an entering or leaving column is done, so these
// are literals rather than `--ev-dur-*` (400ms is the nearest token).
export const EDGE_MS = 350;
const EDGE_EASE = "var(--ev-ease-out, cubic-bezier(0.22, 1, 0.36, 1))";
export const SPIN_MS = 750;
// A damped spring (zeta .72, ~4% overshoot) sampled for `linear()`.
const SPRING =
  "linear(0, 0.097, 0.301, 0.521, 0.712, 0.855, 0.95, 1.005, 1.031, 1.038, 1.035, 1.028, 1.019, 1.012, 1.006, 1.002, 1, 0.999, 1, 1, 1)";
const WINDOW_EDGE = "0.25em";
const WINDOW_MASK = `linear-gradient(to bottom, transparent 0, black ${WINDOW_EDGE}, black calc(100% - ${WINDOW_EDGE}), transparent 100%)`;

function spinEasing(): string {
  const springy = typeof CSS !== "undefined" && typeof CSS.supports === "function" && CSS.supports("transition-timing-function", SPRING);
  return springy ? SPRING : EDGE_EASE;
}

interface SlotProps {
  item: NumberItem;
  glyphs: readonly string[];
  direction: RollDirection;
  motion: boolean;
  spin: number;
}

/** One part of the number; collapses to zero width to enter or leave, so its neighbours slide. */
export function Slot({ item, glyphs, direction, motion, spin }: SlotProps) {
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
