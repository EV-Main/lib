import * as React from "react";
import { cn } from "../../lib/cn";
import * as copy from "../../generated/copy";
import { SPINNER } from "../../generated/spinner";
import { iconShapes } from "../../icons/icon-shapes";

export function Spinner({ className, ...props }: React.ComponentProps<"svg">) {
  return (
    <svg
      role="status"
      aria-label={copy.LOADING}
      className={cn(SPINNER, className)}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={2}
      {...props}
    >
      {iconShapes("spinner")}
    </svg>
  );
}
