import * as React from "react";
import { cn } from "../lib/cn";
import { buttonVariants } from "./button";
import { Tooltip, TooltipContent, TooltipTrigger } from "./tooltip";

export type IconButtonVariant = "secondary" | "ghost";
export type IconButtonSize = "sm" | "md" | "touch";

export interface IconButtonProps extends Omit<React.ComponentPropsWithoutRef<"button">, "aria-label" | "children"> {
  /** Required: a glyph alone names nothing. */
  "aria-label": string;
  /** The glyph, 16 px. */
  children: React.ReactNode;
  variant?: IconButtonVariant;
  size?: IconButtonSize;
  /** Toggled on: the active fill, and `aria-pressed` for assistive tech. */
  pressed?: boolean;
  /** Shown in a Tooltip on hover and focus. */
  tooltip?: React.ReactNode;
}

/**
 * A square button holding one glyph. It forwards its ref (also on React 18,
 * where function components drop `ref`) so it can be the child of any
 * `asChild` trigger; with a `tooltip` the same element anchors the tooltip.
 */
export const IconButton = React.forwardRef<HTMLButtonElement, IconButtonProps>(function IconButton(
  { variant = "ghost", size = "md", pressed, tooltip, className, type = "button", ...props },
  ref,
) {
  const button = (
    <button
      ref={ref}
      type={type}
      data-slot="icon-button"
      data-variant={variant}
      aria-pressed={pressed}
      className={cn(
        buttonVariants({ variant, size, icon: true }),
        "text-ink-mid hover:text-ink",
        pressed && "bg-active text-ink hover:bg-active",
        className,
      )}
      {...props}
    />
  );
  if (tooltip === undefined) return button;
  return (
    <Tooltip>
      <TooltipTrigger asChild>{button}</TooltipTrigger>
      <TooltipContent>{tooltip}</TooltipContent>
    </Tooltip>
  );
});
