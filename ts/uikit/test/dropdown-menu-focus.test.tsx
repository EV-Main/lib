import { describe, it, expect, vi } from "vitest";
import { act, fireEvent, render, screen } from "@testing-library/react";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "../src/components/dropdown-menu";

function menu(extra?: React.ReactNode) {
  render(
    <DropdownMenu>
      <DropdownMenuTrigger>Open</DropdownMenuTrigger>
      <DropdownMenuContent>
        <DropdownMenuItem>One</DropdownMenuItem>
        <DropdownMenuSub>
          <DropdownMenuSubTrigger>More</DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem>Deep</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>
        {extra}
      </DropdownMenuContent>
    </DropdownMenu>,
  );
  fireEvent.click(screen.getByRole("button", { name: "Open" }));
}

describe("DropdownMenu focus return", () => {
  it("returns focus to the trigger on Escape", () => {
    menu();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(screen.getByRole("button", { name: "Open" })).toHaveFocus();
  });

  it("returns focus to the trigger after an item is chosen", () => {
    menu();
    fireEvent.click(screen.getByRole("menuitem", { name: "One" }));
    expect(screen.getByRole("button", { name: "Open" })).toHaveFocus();
  });

  it("leaves focus where it went when an outside field took it", () => {
    render(
      <>
        <input aria-label="other" />
        <DropdownMenu>
          <DropdownMenuTrigger>Open</DropdownMenuTrigger>
          <DropdownMenuContent>
            <DropdownMenuItem>One</DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </>,
    );
    fireEvent.click(screen.getByRole("button", { name: "Open" }));
    screen.getByRole("textbox").focus();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(screen.getByRole("textbox")).toHaveFocus();
  });
});

describe("DropdownMenuCheckboxItem", () => {
  it("toggles without closing, so several can be switched in one visit", () => {
    const onClick = vi.fn();
    menu(
      <DropdownMenuCheckboxItem checked={false} onClick={onClick}>
        Grid
      </DropdownMenuCheckboxItem>,
    );
    fireEvent.click(screen.getByRole("menuitemcheckbox", { name: "Grid" }));
    expect(onClick).toHaveBeenCalledTimes(1);
    expect(screen.getByRole("menu")).toBeInTheDocument();
  });

  it("toggles from the keyboard", () => {
    const onClick = vi.fn();
    menu(
      <DropdownMenuCheckboxItem checked={false} onClick={onClick}>
        Grid
      </DropdownMenuCheckboxItem>,
    );
    fireEvent.keyDown(screen.getByRole("menuitemcheckbox", { name: "Grid" }), { key: " " });
    expect(onClick).toHaveBeenCalledTimes(1);
  });
});

describe("DropdownMenuSub", () => {
  it("opens on ArrowRight and focuses its first item", () => {
    menu();
    const trigger = screen.getByRole("menuitem", { name: "More" });
    trigger.focus();
    fireEvent.keyDown(trigger, { key: "ArrowRight" });
    expect(screen.getByRole("menuitem", { name: "Deep" })).toHaveFocus();
  });

  it("opens on Enter and Space too", () => {
    menu();
    const trigger = screen.getByRole("menuitem", { name: "More" });
    fireEvent.keyDown(trigger, { key: "Enter" });
    expect(screen.getByRole("menuitem", { name: "Deep" })).toBeInTheDocument();
  });

  it("closes on ArrowLeft and refocuses the sub-trigger", () => {
    menu();
    const trigger = screen.getByRole("menuitem", { name: "More" });
    fireEvent.keyDown(trigger, { key: "ArrowRight" });
    fireEvent.keyDown(screen.getByRole("menuitem", { name: "Deep" }), { key: "ArrowLeft" });
    expect(screen.queryByRole("menuitem", { name: "Deep" })).toBeNull();
    expect(trigger).toHaveFocus();
  });

  it("closes only the flyout on Escape, not the whole menu", () => {
    menu();
    const trigger = screen.getByRole("menuitem", { name: "More" });
    fireEvent.keyDown(trigger, { key: "ArrowRight" });
    fireEvent.keyDown(screen.getByRole("menuitem", { name: "Deep" }), { key: "Escape" });
    expect(screen.queryByRole("menuitem", { name: "Deep" })).toBeNull();
    expect(screen.getByRole("menu")).toBeInTheDocument();
    expect(trigger).toHaveFocus();
  });

  it("opens on mouse hover after a short dwell", async () => {
    // jsdom has no PointerEvent, so React would read no `pointerType` at all.
    class MousePointerEvent extends MouseEvent {
      pointerType = "mouse";
    }
    vi.stubGlobal("PointerEvent", MousePointerEvent);
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    try {
      menu();
      const trigger = screen.getByRole("menuitem", { name: "More" });
      fireEvent.pointerEnter(trigger.parentElement as HTMLElement, {});
      expect(screen.queryByRole("menuitem", { name: "Deep" })).toBeNull();
      await act(async () => {
        await vi.advanceTimersByTimeAsync(150);
      });
      expect(screen.getByRole("menuitem", { name: "Deep" })).toBeInTheDocument();
    } finally {
      vi.useRealTimers();
      vi.unstubAllGlobals();
    }
  });
});
