import * as React from "react";
import { cn } from "../../lib/cn";
import {
  ALERT_BASE,
  ALERT_DESCRIPTION,
  ALERT_TITLE,
} from "../../generated/alert";
import { toneClasses, type Tone } from "../../generated/tone";

export interface AlertProps extends React.ComponentProps<"div"> {
  tone?: Tone;
}

export function Alert({ className, tone = "neutral", ...props }: AlertProps) {
  return (
    <div
      data-slot="alert"
      role="alert"
      className={cn(ALERT_BASE, toneClasses[tone], className)}
      {...props}
    />
  );
}

export function AlertTitle({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="alert-title"
      className={cn(ALERT_TITLE, className)}
      {...props}
    />
  );
}

export function AlertDescription({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="alert-description"
      className={cn(ALERT_DESCRIPTION, className)}
      {...props}
    />
  );
}
