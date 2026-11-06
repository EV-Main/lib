"use client";

import * as React from "react";
import { cn } from "../../lib/cn";
import {
  CONFIRM_DIALOG_CONTENT,
  CONFIRM_DIALOG_DESCRIPTION,
  CONFIRM_DIALOG_ERROR,
  CONFIRM_DIALOG_FOOTER,
  CONFIRM_DIALOG_HEADER,
  CONFIRM_DIALOG_ICON,
  CONFIRM_DIALOG_PENDING,
  CONFIRM_DIALOG_SPINNER,
  CONFIRM_DIALOG_TEXT,
  CONFIRM_DIALOG_TITLE,
} from "../../generated/confirm-dialog";
import { toneClasses, type Tone } from "../../generated/tone";
import { useControllableState } from "../../primitives/use-controllable-state";
import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "../atomic/alert-dialog";
import { buttonVariants } from "../atomic/button";
import { Spinner } from "../atomic/spinner";
import { UiIcon } from "../../icons/ui-icons";

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
  tone?: Tone;
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

export function ConfirmDialog({
  open,
  defaultOpen = false,
  onOpenChange,
  trigger,
  title,
  description,
  tone = "neutral",
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
  const danger = tone === "error";
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
      <AlertDialogContent className={cn(CONFIRM_DIALOG_CONTENT, toneClasses[tone], className)}>
        <AlertDialogHeader className={CONFIRM_DIALOG_HEADER}>
          <span className={CONFIRM_DIALOG_ICON}>
            {icon ?? <UiIcon name="alert" />}
          </span>
          <div className={CONFIRM_DIALOG_TEXT}>
            <AlertDialogTitle className={CONFIRM_DIALOG_TITLE}>{title}</AlertDialogTitle>
            {description != null && (
              <AlertDialogDescription className={CONFIRM_DIALOG_DESCRIPTION}>{description}</AlertDialogDescription>
            )}
            {error !== null && (
              <p role="alert" className={CONFIRM_DIALOG_ERROR}>
                {error}
              </p>
            )}
          </div>
        </AlertDialogHeader>
        <AlertDialogFooter className={CONFIRM_DIALOG_FOOTER}>
          {/* aria-disabled, not disabled: a disabled button that holds focus drops it to <body> and Tab leaves the dialog. */}
          <button
            type="button"
            aria-disabled={pending}
            onClick={() => change(false)}
            className={cn(buttonVariants({ variant: "ghost", size: "xs" }), CONFIRM_DIALOG_PENDING)}
          >
            {labels.cancel ?? "Cancel"}
          </button>
          <button
            type="button"
            aria-disabled={pending}
            aria-busy={pending}
            onClick={confirm}
            className={cn(buttonVariants({ variant: danger ? "destructive" : "primary", size: "xs" }), CONFIRM_DIALOG_PENDING)}
          >
            {pending && <Spinner className={CONFIRM_DIALOG_SPINNER} />}
            {confirmLabel}
          </button>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
