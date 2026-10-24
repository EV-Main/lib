import * as React from "react";
import { cn } from "../lib/cn";
import {
  SMART_HEADER_BAR,
  SMART_HEADER_BASE,
  SMART_HEADER_CONTEXT,
  SMART_HEADER_TOOLBAR,
} from "../generated/smart-header";
import { TopBar } from "./top-bar";

export type SmartHeaderState = "default" | "collapsed" | "banner" | "search";

export interface SmartHeaderProps extends Omit<React.ComponentProps<"div">, "children"> {
  /**
   * `default` — bar, toolbar and context line; `collapsed` — the toolbar folds
   * into the bar as `collapsed`; `banner` — a banner row above one bar;
   * `search` — one bar with the `palette` under it.
   */
  state?: SmartHeaderState;
  /** Brand, workspace tabs: the left side of the bar. */
  start?: React.ReactNode;
  /** The right side of the bar: icon actions, balance, bell, account menu. */
  end?: React.ReactNode;
  /** The folded toolbar, between `start` and `end`; shown in `collapsed` only. */
  collapsed?: React.ReactNode;
  /** A `SystemBanner`; shown in `banner` only. */
  banner?: React.ReactNode;
  /** The second row, 44 high; shown in `default` only. */
  toolbar?: React.ReactNode;
  /** The slim line on the page fill under the toolbar; shown in `default` only. */
  context?: React.ReactNode;
  /** A `CommandPalette` or any panel that drops from the bar; shown in `search` only. */
  palette?: React.ReactNode;
}

/**
 * A page header that adapts to scroll and alerts. The bar is the same in every
 * state, so switching never moves the brand or the account: only the rows
 * around it come and go. Layout only; the caller owns the state and its trigger
 * (a scroll observer, a feed status, the search button).
 */
export function SmartHeader({
  state = "default",
  start,
  end,
  collapsed,
  banner,
  toolbar,
  context,
  palette,
  className,
  ...props
}: SmartHeaderProps) {
  return (
    <div
      data-slot="smart-header"
      data-state={state}
      className={cn(SMART_HEADER_BASE, className)}
      {...props}
    >
      {state === "banner" && banner}
      <TopBar
        className={SMART_HEADER_BAR}
        start={
          <>
            {start}
            {state === "collapsed" && collapsed}
          </>
        }
        end={end}
      />
      {state === "default" && toolbar != null && (
        <div data-slot="smart-header-toolbar" className={SMART_HEADER_TOOLBAR}>
          {toolbar}
        </div>
      )}
      {state === "default" && context != null && (
        <div data-slot="smart-header-context" className={SMART_HEADER_CONTEXT}>
          {context}
        </div>
      )}
      {state === "search" && palette != null && <div data-slot="smart-header-palette">{palette}</div>}
    </div>
  );
}
