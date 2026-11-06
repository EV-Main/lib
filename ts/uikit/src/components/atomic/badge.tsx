import * as React from "react";
import { cn } from "../../lib/cn";
import { Slot } from "../../primitives/slot";
import { BADGE_BASE } from "../../generated/badge";
import { toneClasses, type Tone } from "../../generated/tone";

export interface BadgeProps extends React.ComponentProps<"span"> {
  tone?: Tone;
  asChild?: boolean;
}

export function Badge({
  className,
  tone = "neutral",
  asChild = false,
  ...props
}: BadgeProps) {
  const Comp = asChild ? Slot : "span";
  return (
    <Comp
      data-slot="badge"
      className={cn(BADGE_BASE, toneClasses[tone], className)}
      {...(props as Record<string, unknown>)}
    />
  );
}
