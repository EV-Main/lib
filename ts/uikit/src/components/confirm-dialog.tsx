"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { useControllableState } from "../primitives/use-controllable-state";
import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "./alert-dialog";
import { buttonVariants } from "./button";
import { Spinner } from "./spinner";
import { UiIcon } from "./ui-icons";

export type ConfirmDialogTone = "default" | "danger";

export interface ConfirmDialogLabels {
  cancel?: string;
  /** Shown when `onConfirm` rejects with something that has no message. */
  error?: string;
}

export interface ConfirmDialogProps {
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  /** Opens the dialog; wrapped with `asChild`, so pass one button. */
  trigger?: React.ReactNode;
  title: React.ReactNode;
  description?: React.ReactNode;
  tone?: ConfirmDialogTone;
  confirmLabel: React.ReactNode;
  /**
   * Resolve to close the dialog. While a returned promise is pending both
   * buttons are disabled and Escape and outside clicks do nothing; a rejection
   * keeps the dialog open and shows its message under the text.
   */
  onConfirm: () => void | Promise<void>;
  /** Replaces the tone's glyph. */
  icon?: React.ReactNode;
  labels?: ConfirmDialogLabels;
  className?: string;
}

const PENDING = "aria-disabled:cursor-not-allowed aria-disabled:opacity-40";

export function ConfirmDialog({
  open,
  defaultOpen = false,
  onOpenChange,
  trigger,
  title,
  description,
  tone = "default",
  confirmLabel,
  onConfirm,
  icon,
  labels = {},
  className,
}: ConfirmDialogProps) {
  const [isOpen, setOpen] = useControllableState<boolean>({
    ...(open !== undefined ? { value: open } : {}),
    defaultValue: defaultOpen,
    ...(onOpenChange ? { onChange: onOpenChange } : {}),
  });
  const [pending, setPending] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const danger = tone === "danger";
  // A controlled parent can reopen the dialog without going through `change`.
  React.useEffect(() => {
    if (isOpen) setError(null);
  }, [isOpen]);

  const change = (next: boolean) => {
    if (pending) return;
    if (next) setError(null);
    setOpen(next);
  };

  const confirm = async () => {
    if (pending) return;
    setError(null);
    setPending(true);
    try {
      await onConfirm();
      setPending(false);
      setOpen(false);
    } catch (e) {
      setPending(false);
      setError(e instanceof Error && e.message ? e.message : (labels.error ?? "Something went wrong"));
    }
  };

  return (
    <AlertDialog open={isOpen} onOpenChange={change}>
      {trigger != null && <AlertDialogTrigger asChild>{trigger}</AlertDialogTrigger>}
      <AlertDialogContent className={cn("max-w-105 gap-4 bg-popover p-5", className)}>
        <AlertDialogHeader className="flex-row gap-3 text-left">
          <span
            className={cn(
              "flex size-8 shrink-0 items-center justify-center rounded-(--control-radius)",
              danger ? "bg-chip-error-bg text-accent-error-ink" : "bg-muted text-ink-mid",
            )}
          >
            {icon ?? <UiIcon name="alert" />}
          </span>
          <div className="flex min-w-0 flex-col gap-1">
            <AlertDialogTitle className="text-h3 font-medium text-ink">{title}</AlertDialogTitle>
            {description != null && (
              <AlertDialogDescription className="text-body text-ink-mid">{description}</AlertDialogDescription>
            )}
            {error !== null && (
              <p role="alert" className="text-body text-accent-error-ink">
                {error}
              </p>
            )}
          </div>
        </AlertDialogHeader>
        <AlertDialogFooter className="flex-row justify-end gap-2">
          {/* aria-disabled, not disabled: a disabled button that holds focus drops it to <body> and Tab leaves the dialog. */}
          <button
            type="button"
            aria-disabled={pending}
            onClick={() => change(false)}
            className={cn(buttonVariants({ variant: "ghost" }), PENDING)}
          >
            {labels.cancel ?? "Cancel"}
          </button>
          <button
            type="button"
            aria-disabled={pending}
            aria-busy={pending}
            onClick={confirm}
            className={cn(buttonVariants({ variant: danger ? "danger" : "solid" }), PENDING)}
          >
            {pending && <Spinner className="size-3.5" />}
            {confirmLabel}
          </button>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
