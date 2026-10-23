import * as React from "react";
import { describe, it, expect, beforeEach, afterEach } from "vitest";
import { render, fireEvent, screen, act } from "@testing-library/react";
import { Dialog, DialogContent, DialogTitle } from "../src/components/dialog";
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
