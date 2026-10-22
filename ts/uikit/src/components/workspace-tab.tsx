"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { useControllableState } from "../primitives/use-controllable-state";
import { UiIcon } from "./ui-icons";

interface WorkspaceTabsContextValue {
  value: string;
  setValue: (next: string) => void;
}

const WorkspaceTabsContext = React.createContext<WorkspaceTabsContextValue | null>(null);

function useWorkspaceTabs(): WorkspaceTabsContextValue {
  const ctx = React.useContext(WorkspaceTabsContext);
  if (!ctx) throw new Error("WorkspaceTab must be used within <WorkspaceTabs>");
  return ctx;
}

export interface WorkspaceTabsProps extends Omit<React.ComponentProps<"div">, "onChange" | "defaultValue"> {
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
}

/**
 * A row of open workspaces. Arrow keys move focus (Home/End jump); selecting is
 * Enter, Space or a click, so moving through the row never swaps the screen.
 */
export function WorkspaceTabs({
  value,
  defaultValue = "",
  onValueChange,
  className,
  onKeyDown,
  ...props
}: WorkspaceTabsProps) {
  const [active, setValue] = useControllableState<string>({
    ...(value !== undefined ? { value } : {}),
    defaultValue,
    ...(onValueChange ? { onChange: onValueChange } : {}),
  });
  // Tabs come and go as workspaces open and close, so the order is read from the DOM at key time.
  const move = (e: React.KeyboardEvent<HTMLDivElement>) => {
    const tabs = Array.from(e.currentTarget.querySelectorAll<HTMLElement>('[role="tab"]'));
    const at = tabs.indexOf(document.activeElement as HTMLElement);
    if (at < 0) return;
    const target =
      e.key === "ArrowRight" ? (at + 1) % tabs.length
      : e.key === "ArrowLeft" ? (at - 1 + tabs.length) % tabs.length
      : e.key === "Home" ? 0
      : e.key === "End" ? tabs.length - 1
      : -1;
    if (target < 0) return;
    e.preventDefault();
    tabs[target]?.focus();
  };
  return (
    <WorkspaceTabsContext.Provider value={{ value: active, setValue }}>
      <div
        role="tablist"
        aria-orientation="horizontal"
        data-slot="workspace-tabs"
        className={cn("flex items-center gap-1", className)}
        onKeyDown={(e) => {
          onKeyDown?.(e);
          if (!e.defaultPrevented) move(e);
        }}
        {...props}
      />
    </WorkspaceTabsContext.Provider>
  );
}

export interface WorkspaceTabProps extends Omit<React.ComponentProps<"button">, "value"> {
  value: string;
  /** Leading glyph; the layers glyph by default. */
  icon?: React.ReactNode;
  /** Adds a close button; Delete and Backspace on the tab close it too. */
  onClose?: () => void;
  /** Names the close button, e.g. "Close Trading". */
  closeLabel?: string;
}

export function WorkspaceTab({
  value,
  icon,
  onClose,
  closeLabel = "Close tab",
  className,
  children,
  onClick,
  onKeyDown,
  ...props
}: WorkspaceTabProps) {
  const { value: active, setValue } = useWorkspaceTabs();
  const selected = active === value;
  return (
    <div
      data-slot="workspace-tab"
      data-state={selected ? "active" : "inactive"}
      className={cn(
        "group inline-flex h-7 items-center rounded-(--control-radius) border border-transparent text-body-s font-medium text-ink-mid transition-colors",
        "hover:bg-hover hover:text-ink focus-within:ring-2 focus-within:ring-ring",
        selected && "border-border-subtle bg-secondary text-ink hover:bg-secondary",
        className,
      )}
    >
      <button
        type="button"
        role="tab"
        aria-selected={selected}
        tabIndex={selected ? 0 : -1}
        className="inline-flex h-full min-w-0 items-center gap-2 rounded-[inherit] pl-2 outline-none"
        onClick={(e) => {
          onClick?.(e);
          setValue(value);
        }}
        onKeyDown={(e) => {
          onKeyDown?.(e);
          if (onClose && (e.key === "Delete" || e.key === "Backspace")) {
            e.preventDefault();
            onClose();
          }
        }}
        {...props}
      >
        {icon ?? <UiIcon name="layers" />}
        <span className="truncate">{children}</span>
      </button>
      {onClose ? (
        <button
          type="button"
          aria-label={closeLabel}
          tabIndex={-1}
          onClick={onClose}
          className="mx-0.5 inline-flex size-5 items-center justify-center rounded-(--control-radius) text-ink-soft outline-none hover:bg-hover hover:text-ink focus-visible:ring-2 focus-visible:ring-ring"
        >
          <UiIcon name="x" />
        </button>
      ) : (
        <span className="w-2" aria-hidden />
      )}
    </div>
  );
}
