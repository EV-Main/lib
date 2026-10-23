"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { useControllableState } from "../primitives/use-controllable-state";

/** What a nav group takes to become a section that opens and closes. */
export interface NavGroupCollapse {
  /**
   * Draws the group's `label` as a header with a chevron and lets the rows fold
   * away. Needs a `label`. Off by default so an existing rail keeps its static
   * eyebrow.
   */
  collapsible?: boolean;
  /** Open on mount. Default: open exactly when the group holds the current item. */
  defaultOpen?: boolean;
  /** Controlled open state; pair with `onOpenChange`. */
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  /** A landing page: the label navigates to it and only the chevron toggles. */
  href?: string;
}

export interface NavCollapseLabels {
  /** Spoken before the group's name on a closed section's chevron. Default `"Expand"`. */
  expand?: string;
  /** Spoken before the group's name on an open section's chevron. Default `"Collapse"`. */
  collapse?: string;
}

/**
 * Open state of one collapsible group. A route change that lands inside a closed
 * group opens it; a user who closes the group holding the current row is not
 * fought, because only the move INTO the group opens it.
 */
export function useNavGroupOpen(
  group: NavGroupCollapse,
  containsCurrent: boolean,
): readonly [open: boolean, setOpen: (next: boolean) => void] {
  const [open, setOpen] = useControllableState<boolean>({
    value: group.open,
    defaultValue: group.defaultOpen ?? containsCurrent,
    onChange: group.onOpenChange,
  });
  const hadCurrent = React.useRef(containsCurrent);
  React.useEffect(() => {
    if (containsCurrent && !hadCurrent.current && !open) setOpen(true);
    hadCurrent.current = containsCurrent;
  }, [containsCurrent, open, setOpen]);
  return [open, setOpen] as const;
}

const MOTION =
  "duration-(--ev-dur-fast) ease-(--ev-ease-in-out) motion-reduce:transition-none";

/**
 * The folding body. Height runs on a `0fr → 1fr` grid track (no measuring), the
 * content fades, and `visibility` flips at the END of the close so the hidden
 * rows leave the tab order without cutting the fade. `inert` is set at once so
 * a closing region cannot take focus or be read while it fades.
 */
export function NavCollapse({
  open,
  id,
  children,
}: {
  open: boolean;
  id: string;
  children: React.ReactNode;
}) {
  const ref = React.useRef<HTMLDivElement>(null);
  // `inert` as a React prop is a boolean in 19 and unknown in 18; the DOM
  // attribute works in both.
  React.useLayoutEffect(() => {
    ref.current?.toggleAttribute("inert", !open);
  }, [open]);
  return (
    <div
      ref={ref}
      id={id}
      data-slot="nav-collapse"
      data-state={open ? "open" : "closed"}
      className={cn(
        "grid transition-[grid-template-rows]",
        MOTION,
        open ? "grid-rows-[1fr]" : "grid-rows-[0fr]",
      )}
    >
      {/* `overflow-y-clip` keeps the x axis visible so a focus ring on a row is not cut. */}
      <div
        className={cn(
          "min-h-0 overflow-y-clip transition-[opacity,visibility]",
          MOTION,
          open ? "visible opacity-100" : "invisible opacity-0",
        )}
      >
        {children}
      </div>
    </div>
  );
}

function Chevron({ open }: { open: boolean }) {
  return (
    <svg
      aria-hidden
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={2}
      strokeLinecap="round"
      strokeLinejoin="round"
      className={cn("size-4 transition-transform", MOTION, open && "rotate-90")}
    >
      <path d="m9 18 6-6-6-6" />
    </svg>
  );
}

const ROW = "flex min-h-9 items-center rounded-(--control-radius) text-body font-medium text-ink transition-colors motion-reduce:transition-none";
const FOCUS = "outline-none focus-visible:ring-2 focus-visible:ring-ring";

export interface NavGroupHeaderProps {
  label: React.ReactNode;
  labelId: string;
  regionId: string;
  open: boolean;
  onToggle: () => void;
  href?: string | undefined;
  linkComponent?: React.ElementType | undefined;
  /** The landing page is the current page. */
  current?: boolean | undefined;
  labels?: NavCollapseLabels | undefined;
  className?: string | undefined;
}

/**
 * The section's header row. With an `href` the label is a link and the chevron
 * is its own button; without one the whole row is the button and the chevron is
 * a picture of its state.
 */
export function NavGroupHeader({
  label,
  labelId,
  regionId,
  open,
  onToggle,
  href,
  linkComponent,
  current,
  labels,
  className,
}: NavGroupHeaderProps) {
  const chevronId = React.useId();
  const row = cn(ROW, open ? "bg-active" : "hover:bg-hover", className);
  const chip = cn(
    "grid size-7 shrink-0 place-items-center rounded-md text-ink-mid",
    open && "bg-selected",
  );
  if (href === undefined) {
    return (
      <button
        type="button"
        data-slot="nav-group-header"
        aria-expanded={open}
        aria-controls={regionId}
        onClick={onToggle}
        className={cn(row, FOCUS, "w-full gap-2 py-1 pl-3 pr-1.5 text-left")}
      >
        <span id={labelId} className="min-w-0 flex-1 truncate">
          {label}
        </span>
        <span className={chip}>
          <Chevron open={open} />
        </span>
      </button>
    );
  }
  const Link: React.ElementType = linkComponent ?? "a";
  return (
    <div data-slot="nav-group-header" className={cn(row, "gap-1 pr-1.5")}>
      <Link
        id={labelId}
        href={href}
        aria-current={current ? "page" : undefined}
        className={cn("min-w-0 flex-1 truncate rounded-(--control-radius) py-2 pl-3", FOCUS)}
      >
        {label}
      </Link>
      {/* Named "Expand Guides": the button's own aria-label, then the label's text. */}
      <button
        type="button"
        id={chevronId}
        aria-label={open ? (labels?.collapse ?? "Collapse") : (labels?.expand ?? "Expand")}
        aria-labelledby={`${chevronId} ${labelId}`}
        aria-expanded={open}
        aria-controls={regionId}
        onClick={onToggle}
        className={cn(chip, FOCUS, !open && "hover:bg-hover")}
      >
        <Chevron open={open} />
      </button>
    </div>
  );
}
