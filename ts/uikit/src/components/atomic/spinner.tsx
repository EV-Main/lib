import * as React from "react";
import { cn } from "../../lib/cn";
import * as copy from "../../generated/copy";
import { SPINNER } from "../../generated/spinner";

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
      <path d="M21 12a9 9 0 1 1-6.219-8.56" />
    </svg>
  );
}
