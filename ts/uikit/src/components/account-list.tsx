"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { useControllableState } from "../primitives/use-controllable-state";
import { Avatar, AvatarFallback } from "./avatar";
import { UiIcon } from "./ui-icons";

interface AccountListContextValue {
  value: string;
  setValue: (next: string) => void;
}

const AccountListContext = React.createContext<AccountListContextValue | null>(null);

export interface AccountListProps extends Omit<React.ComponentProps<"div">, "onChange" | "defaultValue"> {
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
  /** Accessible name of the group, e.g. "Accounts". */
  "aria-label": string;
}

/**
 * Pick the account you act as: one is current, arrows move and select
 * (radio-group semantics), so the current account is the only tab stop.
 */
export function AccountList({
  value,
  defaultValue = "",
  onValueChange,
  className,
  onKeyDown,
  ...props
}: AccountListProps) {
  const [current, setValue] = useControllableState<string>({
    ...(value !== undefined ? { value } : {}),
    defaultValue,
    ...(onValueChange ? { onChange: onValueChange } : {}),
  });

  const move = (e: React.KeyboardEvent<HTMLDivElement>) => {
    const forward = e.key === "ArrowDown" || e.key === "ArrowRight";
    const back = e.key === "ArrowUp" || e.key === "ArrowLeft";
    if (!forward && !back) return;
    const radios = Array.from(
      e.currentTarget.querySelectorAll<HTMLButtonElement>('[role="radio"]:not(:disabled)'),
    );
    const at = radios.indexOf(document.activeElement as HTMLButtonElement);
    if (at < 0) return;
    e.preventDefault();
    const next = radios[(at + (forward ? 1 : -1) + radios.length) % radios.length];
    next?.focus();
    next?.click();
  };

  return (
    <AccountListContext.Provider value={{ value: current, setValue }}>
      <div
        role="radiogroup"
        data-slot="account-list"
        className={cn("flex flex-col gap-2", className)}
        onKeyDown={(e) => {
          onKeyDown?.(e);
          if (!e.defaultPrevented) move(e);
        }}
        {...props}
      />
    </AccountListContext.Provider>
  );
}

export interface AccountListItemProps extends Omit<React.ComponentProps<"div">, "value" | "title"> {
  value: string;
  name: React.ReactNode;
  /** A line under the name: handle, email. */
  handle?: React.ReactNode;
  /** Replaces the initial-in-a-square default. */
  avatar?: React.ReactNode;
  /** Role badges after the name block. */
  badges?: React.ReactNode;
  /** A figure at the end, set in the display face: a balance. */
  figure?: React.ReactNode;
  /** Controls beside the row (a menu button); they cannot live inside the radio. */
  actions?: React.ReactNode;
  disabled?: boolean;
}

export function AccountListItem({
  value,
  name,
  handle,
  avatar,
  badges,
  figure,
  actions,
  disabled,
  className,
  ...props
}: AccountListItemProps) {
  const ctx = React.useContext(AccountListContext);
  if (!ctx) throw new Error("AccountListItem must be used within <AccountList>");
  const checked = ctx.value === value;
  const initial = typeof name === "string" ? name.trim().charAt(0).toUpperCase() : "";
  return (
    <div
      data-slot="account-list-item"
      data-state={checked ? "checked" : "unchecked"}
      className={cn(
        "flex items-center gap-3 rounded-(--control-radius) border p-2.5 pr-3 transition-colors",
        checked ? "border-primary-border bg-selected" : "border-border-subtle bg-muted hover:bg-hover",
        disabled && "opacity-40",
        className,
      )}
      {...props}
    >
      <button
        type="button"
        role="radio"
        aria-checked={checked}
        disabled={disabled}
        // Until one is chosen every radio is a tab stop, or the group could not be entered.
        tabIndex={checked || ctx.value === "" ? 0 : -1}
        onClick={() => ctx.setValue(value)}
        className="flex min-w-0 flex-1 items-center gap-3 rounded-(--control-radius) text-left outline-none focus-visible:ring-2 focus-visible:ring-ring"
      >
        {avatar ?? (
          <Avatar className="size-14 shrink-0 rounded-(--control-radius)">
            <AvatarFallback className="font-display text-h2 font-light text-ink">{initial}</AvatarFallback>
          </Avatar>
        )}
        <span className="flex min-w-0 flex-1 flex-col gap-0.5">
          <span className="truncate text-body-l text-ink">{name}</span>
          {handle != null && <span className="truncate text-body text-ink-mid">{handle}</span>}
        </span>
        {badges != null && <span className="flex shrink-0 items-center gap-1.5">{badges}</span>}
        {figure != null && <span className="shrink-0 font-display text-figure-m font-light tabular-nums text-ink">{figure}</span>}
        <span className={cn("flex size-4 shrink-0 text-primary-ink", !checked && "invisible")} aria-hidden>
          <UiIcon name="check" />
        </span>
      </button>
      {actions != null && <div className="flex shrink-0 items-center gap-1">{actions}</div>}
    </div>
  );
}
