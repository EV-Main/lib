"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { NATIVE_SELECT_BASE, nativeSelectSizeClasses, type NativeSelectSize } from "../generated/select";
import { markLabelable } from "../primitives/labelable";
import { useFieldControlId } from "./field-context";
import { SelectChevron } from "./select-chevron";

export type { NativeSelectSize };

/**
 * A real `<select>` wearing the kit's control: the form posts its `name` with no
 * script at all, and the popup is the platform's own (keyboard, type-ahead, the
 * mobile picker). `Select` is the scripted combobox for everything a native
 * popup cannot draw; this is for a form that has to submit before hydration.
 *
 * A component of its own rather than a `native` switch on `Select`: none of
 * `SelectTrigger` / `SelectValue` / `SelectContent` has a native counterpart, so
 * a flag would leave most of the compound API meaning nothing in one mode.
 *
 * `className` styles the `<select>`; `wrapperClassName` the box that holds it
 * and the arrow — that box is what a layout sizes. It is `w-full` like `Input`,
 * so the two line up in a form column; narrow it with `wrapperClassName`.
 */
export interface NativeSelectProps
  extends Omit<React.ComponentPropsWithoutRef<"select">, "size" | "multiple"> {
  size?: NativeSelectSize;
  /** An empty, unpickable first option shown until a value is chosen. */
  placeholder?: string;
  wrapperClassName?: string;
}

// `appearance-none` drops the OS arrow. The popup is drawn by the platform:
// the option colours reach it where the platform honours them, and `--scheme`
// (a palette's polarity, `light` or `dark`) picks the platform's own dark
// menu where it does not. The placeholder is the empty option being the
// checked one — CSS can see that without script.

export const NativeSelect = React.forwardRef<HTMLSelectElement, NativeSelectProps>(
  function NativeSelect(
    { className, wrapperClassName, size = "md", placeholder, id, children, value, defaultValue, ...props },
    ref,
  ) {
    const controlId = useFieldControlId(id);
    // Without a checked option the browser picks the first enabled one, which
    // would skip a disabled placeholder; start on it unless told otherwise.
    const initial = value === undefined && defaultValue === undefined && placeholder !== undefined ? "" : defaultValue;
    return (
      <span
        data-slot="native-select-wrapper"
        className={cn("relative inline-flex w-full has-[select:disabled]:opacity-50", wrapperClassName)}
      >
        <select
          ref={ref}
          id={controlId}
          data-slot="native-select"
          data-size={size}
          className={cn(NATIVE_SELECT_BASE, nativeSelectSizeClasses[size], className)}
          {...(value !== undefined ? { value } : {})}
          {...(initial !== undefined ? { defaultValue: initial } : {})}
          {...props}
        >
          {placeholder !== undefined && (
            <option value="" disabled hidden>
              {placeholder}
            </option>
          )}
          {children}
        </select>
        <SelectChevron
          className={cn(
            "text-ink-soft pointer-events-none absolute top-1/2 right-3 size-4 -translate-y-1/2",
            // keeps the arrow as far in from the edge as the large inset keeps the text
            size === "touch" && "right-4",
          )}
        />
      </span>
    );
  },
);

markLabelable(NativeSelect);

export function NativeSelectOption(props: React.ComponentProps<"option">) {
  return <option data-slot="native-select-option" {...props} />;
}

export function NativeSelectGroup(props: React.ComponentProps<"optgroup">) {
  return <optgroup data-slot="native-select-group" {...props} />;
}
