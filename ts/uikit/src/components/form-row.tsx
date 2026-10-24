import * as React from "react";
import { cn } from "../lib/cn";
import {
  FORM_ROW_BASE,
  FORM_ROW_CONTROL,
  FORM_ROW_DESCRIPTION,
  FORM_ROW_DIVIDER,
  FORM_ROW_HINT,
  FORM_ROW_LABEL,
  FORM_ROW_LABEL_COLUMN,
} from "../generated/form-row";

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
        FORM_ROW_BASE,
        divider && FORM_ROW_DIVIDER,
        className,
      )}
      {...props}
    >
      <div data-slot="form-row-label" className={FORM_ROW_LABEL_COLUMN}>
        <Label {...(htmlFor !== undefined ? { htmlFor } : {})} className={FORM_ROW_LABEL}>
          {label}
        </Label>
        {description != null && <span className={FORM_ROW_DESCRIPTION}>{description}</span>}
      </div>
      <div data-slot="form-row-control" className={FORM_ROW_CONTROL}>
        {children}
        {hint != null && <span className={FORM_ROW_HINT}>{hint}</span>}
      </div>
    </div>
  );
}
