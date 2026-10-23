import * as React from "react";
import { describe, it, expect, beforeEach, afterEach } from "vitest";
import { render, fireEvent, screen, act } from "@testing-library/react";
import { Dialog, DialogContent, DialogTitle } from "../src/components/dialog";
import { AlertDialog, AlertDialogContent, AlertDialogTitle } from "../src/components/alert-dialog";
import { CommandDialog } from "../src/components/command";
import { Popover, PopoverContent, PopoverTrigger } from "../src/components/popover";

// jsdom paints nothing, but it resolves `z-index` from a stylesheet, which is
// all the stacking order needs. The kit's own classes are Tailwind's, absent
// here, so the page's z-50 on every overlay is stated by hand.
let sheet: HTMLStyleElement;
beforeEach(() => {
  sheet = document.createElement("style");
  sheet.textContent =
    '[data-slot$="overlay"], [data-slot="dialog-content"], [data-slot="alert-dialog-content"], [data-slot="popover-content"] { position: fixed; z-index: 50; }';
  document.head.append(sheet);
});
afterEach(() => sheet.remove());

const escape = () => act(() => void fireEvent.keyDown(document.activeElement ?? document.body, { key: "Escape" }));

/** Paint order of the portaled nodes under `body`: later entries paint over earlier ones. */
function paintRank(el: Element): number[] {
  const z = Number.parseInt(getComputedStyle(el).zIndex, 10);
  const index = [...document.body.children].indexOf(el);
  return [z, index];
}

function paintsAbove(upper: Element, lower: Element): boolean {
  const [zu = 0, iu = 0] = paintRank(upper);
  const [zl = 0, il = 0] = paintRank(lower);
  return zu > zl || (zu === zl && iu > il);
}

const overlays = () => [...document.querySelectorAll<HTMLElement>('[data-slot$="overlay"]')];
const opaqueScrims = () => overlays().filter((el) => el.style.backgroundColor !== "transparent");

describe("a Popover opened in the same commit as the Dialog around it", () => {
  function Together() {
    return (
      <Dialog defaultOpen>
        <DialogContent>
          <DialogTitle>Opened together</DialogTitle>
          <Popover defaultOpen>
            <PopoverTrigger>Details</PopoverTrigger>
            <PopoverContent>Opened in the dialog's own commit.</PopoverContent>
          </Popover>
        </DialogContent>
      </Dialog>
    );
  }

  it("paints above the dialog panel", () => {
    render(<Together />);
    const popover = document.querySelector('[data-slot="popover-content"]')!;
    const panel = document.querySelector('[data-slot="dialog-content"]')!;
    expect(paintsAbove(popover, panel)).toBe(true);
  });

  it("keeps focus in the popover, and Escape returns it to the trigger without closing the dialog", () => {
    render(<Together />);
    const popover = document.querySelector<HTMLElement>('[data-slot="popover-content"]')!;
    expect(popover.contains(document.activeElement)).toBe(true);

    escape();

    expect(document.querySelector('[data-slot="popover-content"]')).toBeNull();
    expect(screen.getByRole("dialog")).toBeTruthy();
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Details" }));
  });
});

describe("a Popover opened later inside a Dialog", () => {
  it("leaves the stacking to the DOM order, with no inline z-index", () => {
    render(
      <Dialog defaultOpen>
        <DialogContent>
          <DialogTitle>Edit</DialogTitle>
          <Popover>
            <PopoverTrigger>Details</PopoverTrigger>
            <PopoverContent>Later</PopoverContent>
          </Popover>
        </DialogContent>
      </Dialog>,
    );
    fireEvent.click(screen.getByRole("button", { name: "Details" }));

    const popover = document.querySelector<HTMLElement>('[data-slot="popover-content"]')!;
    expect(popover.style.zIndex).toBe("");
    expect(paintsAbove(popover, document.querySelector('[data-slot="dialog-content"]')!)).toBe(true);
  });
});

describe("modals stacked on one page share one scrim", () => {
  function DialogOverDialog() {
    return (
      <Dialog defaultOpen>
        <DialogContent>
          <DialogTitle>Parent</DialogTitle>
          <AlertDialog defaultOpen>
            <AlertDialogContent>
              <AlertDialogTitle>Child</AlertDialogTitle>
            </AlertDialogContent>
          </AlertDialog>
        </DialogContent>
      </Dialog>
    );
  }

  it("draws one opaque scrim for a Dialog and an AlertDialog opened in the same commit", () => {
    render(<DialogOverDialog />);
    expect(overlays()).toHaveLength(2);
    expect(opaqueScrims()).toHaveLength(1);
  });

  it("keeps the one scrim under both panels, the child panel on top", () => {
    render(<DialogOverDialog />);
    const [scrim] = opaqueScrims();
    const parent = document.querySelector('[data-slot="dialog-content"]')!;
    const child = document.querySelector('[data-slot="alert-dialog-content"]')!;
    expect(paintsAbove(parent, scrim!)).toBe(true);
    expect(paintsAbove(child, parent)).toBe(true);
  });

  it("draws one opaque scrim when a modal opens over one that has been open for a while", () => {
    const { rerender } = render(
      <Dialog open>
        <DialogContent>
          <DialogTitle>Base</DialogTitle>
        </DialogContent>
      </Dialog>,
    );
    expect(opaqueScrims()).toHaveLength(1);

    rerender(
      <>
        <Dialog open>
          <DialogContent>
            <DialogTitle>Base</DialogTitle>
          </DialogContent>
        </Dialog>
        <CommandDialog open onOpenChange={() => {}} />
      </>,
    );

    expect(overlays()).toHaveLength(2);
    expect(opaqueScrims()).toHaveLength(1);
  });

  it("keeps the scrim for the modal below when the top modal closes", () => {
    render(<DialogOverDialog />);
    escape();
    // The AlertDialog is gone (no animation in jsdom); the Dialog still has its scrim.
    expect(document.querySelector('[data-slot="alert-dialog-content"]')).toBeNull();
    expect(screen.getByRole("dialog")).toBeTruthy();
    expect(opaqueScrims()).toHaveLength(1);
  });

  it("changes nothing for a single modal", () => {
    render(
      <Dialog defaultOpen>
        <DialogContent>
          <DialogTitle>Alone</DialogTitle>
        </DialogContent>
      </Dialog>,
    );
    expect(overlays()).toHaveLength(1);
    expect(overlays()[0]!.getAttribute("style")).toBeNull();
  });
});
