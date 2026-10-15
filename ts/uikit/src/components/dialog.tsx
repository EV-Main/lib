"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { useControllableState } from "../primitives/use-controllable-state";
import { DismissableLayerScope, useDismissableLayer } from "../primitives/dismissable-layer";
import { type InitialFocus, useFocusScope } from "../primitives/focus-scope";
import { usePresence } from "../primitives/presence";
import { mergeRefs } from "../primitives/merge-refs";
import { Portal } from "../primitives/portal";
import { Slot } from "../primitives/slot";
import {
  DIALOG_CLOSE,
  DIALOG_CONTENT,
  DIALOG_DESCRIPTION,
  DIALOG_FOOTER,
  DIALOG_HEADER,
  DIALOG_OVERLAY,
  DIALOG_TITLE,
} from "../generated/dialog";

interface DialogContextValue {
  open: boolean;
  setOpen: (next: boolean) => void;
  triggerRef: React.RefObject<HTMLElement | null>;
  /** Ids of the mounted DialogTitle / DialogDescription; unset while none is. */
  titleId: string | undefined;
  descriptionId: string | undefined;
  setTitleId: React.Dispatch<React.SetStateAction<string | undefined>>;
  setDescriptionId: React.Dispatch<React.SetStateAction<string | undefined>>;
}

const DialogContext = React.createContext<DialogContextValue | null>(null);

function useDialog(): DialogContextValue {
  const ctx = React.useContext(DialogContext);
  if (!ctx) throw new Error("Dialog parts must be used within <Dialog>");
  return ctx;
}

export interface DialogProps {
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  children?: React.ReactNode;
}

export function Dialog({ open, defaultOpen = false, onOpenChange, children }: DialogProps) {
  const [isOpen, setOpen] = useControllableState<boolean>({
    ...(open !== undefined ? { value: open } : {}),
    defaultValue: defaultOpen,
    ...(onOpenChange ? { onChange: onOpenChange } : {}),
  });
  const triggerRef = React.useRef<HTMLElement | null>(null);
  const [titleId, setTitleId] = React.useState<string | undefined>(undefined);
  const [descriptionId, setDescriptionId] = React.useState<string | undefined>(undefined);
  return (
    <DialogContext.Provider
      value={{ open: isOpen, setOpen, triggerRef, titleId, descriptionId, setTitleId, setDescriptionId }}
    >
      {children}
    </DialogContext.Provider>
  );
}

export interface DialogTriggerProps extends React.ComponentProps<"button"> {
  asChild?: boolean;
}

export function DialogTrigger({ asChild = false, onClick, ...props }: DialogTriggerProps) {
  const { open, setOpen, triggerRef } = useDialog();
  const Comp = asChild ? Slot : "button";
  return (
    <Comp
      data-slot="dialog-trigger"
      aria-expanded={open}
      ref={triggerRef as React.Ref<HTMLButtonElement>}
      onClick={(e: React.MouseEvent<HTMLButtonElement>) => {
        onClick?.(e);
        setOpen(true);
      }}
      {...(props as Record<string, unknown>)}
    />
  );
}

export type DialogPortalProps = { children?: React.ReactNode };

export function DialogPortal({ children }: DialogPortalProps) {
  return <>{children}</>;
}

export interface DialogOverlayProps extends React.ComponentProps<"div"> {}

export function DialogOverlay({ className, ...props }: DialogOverlayProps) {
  const { open } = useDialog();
  // No onClick: the content's dismissable layer already closes on the
  // pointer-down that lands here, and a second path called onOpenChange(false)
  // twice for one press whenever the overlay was still hit by the click
  // (a host that closes later, CSS without the closed-state pointer-events).
  return (
    <div
      data-slot="dialog-overlay"
      data-state={open ? "open" : "closed"}
      className={cn(DIALOG_OVERLAY, className)}
      {...(props as Record<string, unknown>)}
    />
  );
}

const XIcon = (props: React.SVGProps<SVGSVGElement>) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    {...props}
  >
    <path d="M18 6 6 18M6 6l12 12" />
  </svg>
);

export interface DialogContentProps extends React.ComponentProps<"div"> {
  showCloseButton?: boolean;
  /** Where focus lands on open. Default `first` — a dialog is usually a short form. */
  initialFocus?: InitialFocus;
}

export function DialogContent({
  className,
  children,
  showCloseButton = true,
  initialFocus = "first",
  ...props
}: DialogContentProps) {
  const { open, setOpen, triggerRef, titleId, descriptionId } = useDialog();
  const { isPresent, ref: presRef } = usePresence(open);
  const focusRef = useFocusScope(open, { initialFocus, returnFocusTo: triggerRef });
  const dismissRef = useDismissableLayer({
    enabled: open,
    onDismiss: () => setOpen(false),
    exclude: [triggerRef],
    modal: true,
  });
  React.useEffect(() => {
    if (!open) return;
    const prev = document.body.style.overflow;
    document.body.style.overflow = "hidden";
    return () => {
      document.body.style.overflow = prev;
    };
  }, [open]);
  if (!isPresent) return null;
  return (
    <Portal>
      <DialogOverlay />
      <div
        data-slot="dialog-content"
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
        aria-describedby={descriptionId}
        data-state={open ? "open" : "closed"}
        ref={mergeRefs(focusRef, dismissRef, presRef as React.Ref<HTMLDivElement>)}
        className={cn(DIALOG_CONTENT, className)}
        {...(props as Record<string, unknown>)}
      >
        <DismissableLayerScope layer={dismissRef}>{children}</DismissableLayerScope>
        {showCloseButton && (
          <button
            type="button"
            data-slot="dialog-close"
            onClick={() => setOpen(false)}
            className={DIALOG_CLOSE}
          >
            <XIcon />
            <span className="sr-only">Close</span>
          </button>
        )}
      </div>
    </Portal>
  );
}

export interface DialogCloseProps extends React.ComponentProps<"button"> {
  asChild?: boolean;
}

export function DialogClose({ asChild = false, onClick, ...props }: DialogCloseProps) {
  const { setOpen } = useDialog();
  const Comp = asChild ? Slot : "button";
  return (
    <Comp
      data-slot="dialog-close"
      onClick={(e: React.MouseEvent<HTMLButtonElement>) => {
        onClick?.(e);
        setOpen(false);
      }}
      {...(props as Record<string, unknown>)}
    />
  );
}

export function DialogHeader({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="dialog-header"
      className={cn(DIALOG_HEADER, className)}
      {...props}
    />
  );
}

export function DialogFooter({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="dialog-footer"
      className={cn(DIALOG_FOOTER, className)}
      {...props}
    />
  );
}

/**
 * Announces `id` as the dialog's title or description while the part is
 * mounted. A layout effect, so the reference is in place before the first
 * paint of an opening dialog and gone with the part — DialogContent never
 * points at an id that is not in the document. Outside a Dialog there is
 * nothing to register with: the part renders as before rather than throwing,
 * since it never needed the context until now.
 */
function useRegisteredId(
  own: string | undefined,
  register: React.Dispatch<React.SetStateAction<string | undefined>> | undefined,
): string {
  const generated = React.useId();
  const id = own ?? generated;
  React.useLayoutEffect(() => {
    if (!register) return;
    register(id);
    // Clear only our own id: a title mounted after this one has already taken the slot.
    return () => register((current) => (current === id ? undefined : current));
  }, [id, register]);
  return id;
}

export function DialogTitle({ className, id, ...props }: React.ComponentProps<"h2">) {
  const titleId = useRegisteredId(id, React.useContext(DialogContext)?.setTitleId);
  return (
    <h2
      id={titleId}
      data-slot="dialog-title"
      className={cn(DIALOG_TITLE, className)}
      {...props}
    />
  );
}

export function DialogDescription({ className, id, ...props }: React.ComponentProps<"p">) {
  const descriptionId = useRegisteredId(id, React.useContext(DialogContext)?.setDescriptionId);
  return (
    <p
      id={descriptionId}
      data-slot="dialog-description"
      className={cn(DIALOG_DESCRIPTION, className)}
      {...props}
    />
  );
}
