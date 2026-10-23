"use client";

import * as React from "react";

// `tabindex="-1"` opts an element out of Tab order whatever its tag, so the
// exclusion goes on every branch — otherwise a roving-tabindex grid's parked
// cells would count as tab stops.
const FOCUSABLE = [
  "a[href]",
  "button:not([disabled])",
  "textarea:not([disabled])",
  "input:not([disabled])",
  "select:not([disabled])",
  "[tabindex]",
]
  .map(sel => `${sel}:not([tabindex="-1"])`)
  .join(",");

/**
 * Every enabled, Tab-reachable candidate under `root` in DOM (= Tab) order,
 * before the layout-based visibility filter; `tabindex="-1"` is skipped on any
 * tag. For content that is fully visible while it exists (a popover), this is
 * the tab order itself. Internal to the kit.
 */
export function focusCandidates(root: HTMLElement): HTMLElement[] {
  return Array.from(root.querySelectorAll<HTMLElement>(FOCUSABLE));
}

function focusable(root: HTMLElement): HTMLElement[] {
  return focusCandidates(root).filter(
    (el) => el.offsetWidth > 0 || el.offsetHeight > 0 || el === document.activeElement,
  );
}

/**
 * Where focus lands when the scope opens. `first` is the first Tab stop inside
 * (an input, an alert dialog's least destructive action); `container` is the
 * scope root itself, so a reader starts at the top and the first Tab reaches the
 * first control — right for a panel of content, where the first stop may be an
 * action one keypress away from firing.
 */
export type InitialFocus = "first" | "container";

export interface FocusScopeOptions {
  /** Default `first`; falls back to the container when nothing inside is focusable. */
  initialFocus?: InitialFocus;
  /**
   * Where focus goes back on close when the element focused before opening is
   * no use: `<body>` (Safari does not focus a clicked button), or detached (the
   * opener was re-rendered away). Pass the overlay's trigger.
   */
  returnFocusTo?: React.RefObject<HTMLElement | null>;
}

interface Claim {
  root: HTMLElement;
  // Boxed so an outer scope opened in the same commit can mark it `behind`.
  saved: { el: HTMLElement | null; behind: boolean };
}

// Scopes that took focus in the current task. Cleared in a microtask, so only
// scopes opened in the same commit count, never one that has been open a while.
let claimed: Claim[] = [];

function claim(root: HTMLElement, saved: Claim["saved"]) {
  if (claimed.length === 0) queueMicrotask(() => (claimed = []));
  claimed.push({ root, saved });
}

function usable(el: HTMLElement | null | undefined): el is HTMLElement {
  return !!el && el !== document.body && el.isConnected;
}

// A `<div>` without a tabindex ignores `.focus()`, so a scope with no Tab stops
// used to leave focus on the trigger behind the scrim. `-1` makes the root
// focusable by script (and by a click on its empty area, which keeps focus in
// the scope) without adding it to the Tab order.
function focusRoot(root: HTMLElement) {
  if (!root.hasAttribute("tabindex")) root.tabIndex = -1;
  root.focus();
}

/**
 * Traps Tab focus inside the returned ref's subtree and restores focus to the
 * previously-focused element on unmount — the dep-light core of dialogs/menus,
 * replacing `@radix-ui/react-focus-scope`. On mount, focuses per
 * `initialFocus`; the root gets `tabindex="-1"` whenever it is the target.
 *
 * Rust dialogs rely on the browser's native focus order within a fixed overlay;
 * see the README "Limitations".
 */
export function useFocusScope(
  enabled: boolean,
  { initialFocus = "first", returnFocusTo }: FocusScopeOptions = {},
): React.RefObject<HTMLDivElement | null> {
  const ref = React.useRef<HTMLDivElement | null>(null);
  // Read at close, not captured at open: a trigger re-rendered while the
  // overlay was up is a new node by then.
  const fallbackRef = React.useRef(returnFocusTo);
  fallbackRef.current = returnFocusTo;

  React.useEffect(() => {
    if (!enabled) return;
    const root = ref.current;
    if (!root) return;

    // A scope opened in the same commit (a popover inside this dialog) has
    // already run, since children's effects come first, and taken focus. This
    // one must not pull focus out of it. Both saved the element focused before
    // either opened; the inner scope sits inside this one, so on close it goes
    // back to its own trigger first and to that element only as a fallback.
    const inner = claimed.find((c) => c.root !== root && c.root.contains(document.activeElement));
    const saved: Claim["saved"] = {
      el: inner ? inner.saved.el : (document.activeElement as HTMLElement | null),
      behind: false,
    };
    if (inner) inner.saved.behind = true;
    else {
      const first = initialFocus === "first" ? focusable(root)[0] : undefined;
      if (first) first.focus();
      else focusRoot(root);
    }
    claim(root, saved);

    function onKeyDown(event: KeyboardEvent) {
      if (event.key !== "Tab" || !root) return;
      const items = focusable(root);
      if (items.length === 0) {
        event.preventDefault();
        return;
      }
      const firstEl = items[0]!;
      const lastEl = items[items.length - 1]!;
      const active = document.activeElement;
      // From the root itself Shift+Tab would leave the scope for whatever
      // precedes the portal, so both directions are steered explicitly.
      if (active === root) {
        event.preventDefault();
        (event.shiftKey ? lastEl : firstEl).focus();
      } else if (event.shiftKey && active === firstEl) {
        event.preventDefault();
        lastEl.focus();
      } else if (!event.shiftKey && active === lastEl) {
        event.preventDefault();
        firstEl.focus();
      }
    }

    root.addEventListener("keydown", onKeyDown);
    return () => {
      root.removeEventListener("keydown", onKeyDown);
      claimed = claimed.filter((c) => c.root !== root);
      // Focus that already left the scope (a click outside landed on a field)
      // is the user's; only focus still inside, or dropped to <body>, returns.
      const active = document.activeElement;
      if (active && active !== document.body && !root.contains(active)) return;
      const anchor = fallbackRef.current?.current;
      const target = saved.behind && usable(anchor) ? anchor : usable(saved.el) ? saved.el : anchor;
      if (usable(target)) target.focus({ preventScroll: true });
    };
  }, [enabled, initialFocus]);

  return ref;
}
