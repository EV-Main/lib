import * as React from "react";
import { cn } from "../lib/cn";
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
      className={cn("flex flex-col border border-border bg-card", className)}
      {...props}
    >
      {state === "banner" && banner}
      <TopBar
        className="h-10 border-border-subtle bg-card px-3"
        start={
          <>
            {start}
            {state === "collapsed" && collapsed}
          </>
        }
        end={end}
      />
      {state === "default" && toolbar != null && (
        <div
          data-slot="smart-header-toolbar"
          className="flex h-11 items-center gap-3 border-t border-border py-0 pl-2 pr-3"
        >
          {toolbar}
        </div>
      )}
      {state === "default" && context != null && (
        <div
          data-slot="smart-header-context"
          className="flex h-6.5 items-center gap-4 border-t border-border bg-background px-3 text-caption text-ink-soft"
        >
          {context}
        </div>
      )}
      {state === "search" && palette != null && <div data-slot="smart-header-palette">{palette}</div>}
    </div>
  );
}
