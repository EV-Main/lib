"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { useControllableState } from "../primitives/use-controllable-state";
import { Button } from "./button";
import { Popover, PopoverContent, PopoverTrigger } from "./popover";

export interface PopoverFormLabels {
  submit?: string;
  reset?: string;
}

export interface PopoverFormProps {
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  /** The button that opens the form; wrapped with `asChild`, so pass one element. */
  trigger: React.ReactNode;
  title: React.ReactNode;
  /** The fields: `Field`s with an `Input`, `Select` or `Checkbox`. */
  children: React.ReactNode;
  /** Called on submit; the popover closes afterwards. */
  onSubmit?: (event: React.FormEvent<HTMLFormElement>) => void;
  /** Called when Reset is pressed; native fields are reset by the browser first. */
  onReset?: () => void;
  labels?: PopoverFormLabels;
  className?: string;
}

/** A filter-style form in a popover: a title, fields, Reset and Apply. */
export function PopoverForm({
  open,
  defaultOpen = false,
  onOpenChange,
  trigger,
  title,
  children,
  onSubmit,
  onReset,
  labels = {},
  className,
}: PopoverFormProps) {
  const [isOpen, setOpen] = useControllableState<boolean>({
    ...(open !== undefined ? { value: open } : {}),
    defaultValue: defaultOpen,
    ...(onOpenChange ? { onChange: onOpenChange } : {}),
  });
  const titleId = React.useId();
  return (
    <Popover open={isOpen} onOpenChange={setOpen}>
      <PopoverTrigger asChild>{trigger}</PopoverTrigger>
      <PopoverContent align="start" className={cn("w-70 p-2", className)} aria-labelledby={titleId} role="dialog">
        <form
          className="flex flex-col gap-2.5 px-0 py-1.5"
          onSubmit={(e) => {
            e.preventDefault();
            onSubmit?.(e);
            setOpen(false);
          }}
          onReset={() => onReset?.()}
        >
          <h3 id={titleId} className="px-0 text-h3 font-medium text-ink">
            {title}
          </h3>
          <div className="flex flex-col gap-2.5">{children}</div>
          <div className="flex justify-end gap-1.5">
            <Button type="reset" variant="ghost" size="sm">
              {labels.reset ?? "Reset"}
            </Button>
            <Button type="submit" variant="primary" size="sm">
              {labels.submit ?? "Apply"}
            </Button>
          </div>
        </form>
      </PopoverContent>
    </Popover>
  );
}
