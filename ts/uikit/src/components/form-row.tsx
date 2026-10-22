import * as React from "react";
import { cn } from "../lib/cn";

export interface FormRowProps extends Omit<React.ComponentProps<"div">, "title"> {
  label: React.ReactNode;
  /** The id of the control in `children`; renders the label as a `<label for>`. */
  htmlFor?: string;
  /** A line under the label. */
  description?: React.ReactNode;
  /** A caption under the control, e.g. a format hint. */
  hint?: React.ReactNode;
  /** Hairline under the row. Default true; the last row of a group turns it off. */
  divider?: boolean;
}

/**
 * Label on the left, the value or its control on the right; they stack on a
 * phone. A plain `div`, not `ListRow`: it is a form field row, not a list item,
 * and its label column has a fixed width so controls line up down the page.
 */
export function FormRow({
  label,
  htmlFor,
  description,
  hint,
  divider = true,
  className,
  children,
  ...props
}: FormRowProps) {
  const Label = htmlFor !== undefined ? "label" : "span";
  return (
    <div
      data-slot="form-row"
      data-divider={divider}
      className={cn(
        "flex flex-col gap-1.5 px-4 py-3 sm:flex-row sm:gap-6",
        divider && "border-b border-border-subtle",
        className,
      )}
      {...props}
    >
      <div data-slot="form-row-label" className="flex min-w-0 flex-col gap-0.5 sm:w-52 sm:shrink-0">
        <Label {...(htmlFor !== undefined ? { htmlFor } : {})} className="text-body text-ink-mid">
          {label}
        </Label>
        {description != null && <span className="text-caption text-ink-soft">{description}</span>}
      </div>
      <div data-slot="form-row-control" className="flex min-w-0 flex-1 flex-col gap-1 text-body text-ink">
        {children}
        {hint != null && <span className="text-caption text-ink-soft">{hint}</span>}
      </div>
    </div>
  );
}
