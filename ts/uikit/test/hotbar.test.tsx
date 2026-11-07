import * as React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { Hotbar, type HotbarAction, type HotbarProps } from "../src/components/shell/hotbar";

const Glyph = ({ className }: { className?: string }) => <svg className={className} />;

function setup(actions: Partial<HotbarAction>[], extra: Partial<HotbarProps> = {}) {
  const full = actions.map(a => ({ id: a.label!, icon: Glyph, onSelect: vi.fn(), ...a }) as HotbarAction);
  const view = render(<Hotbar actions={full} {...extra} />);
  const bar = () => within(view.container.querySelector<HTMLElement>('[data-slot="hotbar"]')!);
  const shown = () => bar().queryAllByRole("button").filter(b => b.dataset["slot"] === "hotbar-action").map(b => b.textContent);
  const customize = () => fireEvent.click(bar().getByRole("button", { name: "Customize" }));
  return { ...view, actions: full, bar, shown, customize };
}

afterEach(() => localStorage.clear());

describe("Hotbar", () => {
  it("runs an action from its shortcut, but not a plain letter typed into a field", () => {
    const { actions } = setup([{ label: "Next lead", shortcut: "N" }]);
    const input = document.body.appendChild(document.createElement("input"));

    fireEvent.keyDown(input, { code: "KeyN", key: "n" });
    expect(actions[0]!.onSelect).not.toHaveBeenCalled();

    fireEvent.keyDown(document.body, { code: "KeyN", key: "т" });
    expect(actions[0]!.onSelect).toHaveBeenCalledTimes(1);
    input.remove();
  });

  it("runs a mod shortcut from inside a field, once per press", () => {
    const { actions } = setup([{ label: "Search", shortcut: "mod+K" }]);
    const input = document.body.appendChild(document.createElement("input"));

    fireEvent.keyDown(input, { code: "KeyK", ctrlKey: true });
    fireEvent.keyDown(input, { code: "KeyK", ctrlKey: true, repeat: true });
    fireEvent.keyDown(input, { code: "KeyK" });
    expect(actions[0]!.onSelect).toHaveBeenCalledTimes(1);
    input.remove();
  });

  it("leaves a key another handler took", () => {
    const { actions } = setup([{ label: "Search", shortcut: "mod+K" }]);
    const takeFirst = (e: KeyboardEvent) => e.preventDefault();
    document.addEventListener("keydown", takeFirst, { capture: true });
    fireEvent.keyDown(document.body, { code: "KeyK", metaKey: true });
    document.removeEventListener("keydown", takeFirst, { capture: true });
    expect(actions[0]!.onSelect).not.toHaveBeenCalled();
  });

  it("hides an unpinned action but keeps its shortcut", () => {
    const { actions, shown } = setup([{ label: "Next lead" }, { label: "Add note", shortcut: "A", pinned: false }]);
    expect(shown()).toEqual(["Next lead"]);
    fireEvent.keyDown(document.body, { code: "KeyA" });
    expect(actions[1]!.onSelect).toHaveBeenCalledTimes(1);
  });

  it("keeps the user's pins across a remount, and Reset brings the defaults back", () => {
    const spec = [{ label: "Next lead" }, { label: "Add note", pinned: false }];
    const first = setup(spec);
    first.customize();
    fireEvent.click(screen.getByRole("checkbox", { name: /Next lead/ }));
    fireEvent.click(screen.getByRole("checkbox", { name: /Add note/ }));
    expect(first.shown()).toEqual(["Add note"]);
    cleanup();

    const second = setup(spec);
    expect(second.shown()).toEqual(["Add note"]);
    second.customize();
    fireEvent.click(screen.getByRole("button", { name: "Reset to default" }));
    expect(second.shown()).toEqual(["Next lead"]);
    expect(localStorage.length).toBe(0);
  });

  it("marks a pressed toggle, and the app flips it", () => {
    const { bar } = setup([{ label: "Places", pressed: true }, { label: "Agent", pressed: false }]);
    expect(bar().getByRole("button", { name: "Places" })).toHaveAttribute("aria-pressed", "true");
    expect(bar().getByRole("button", { name: "Agent" })).toHaveAttribute("aria-pressed", "false");
  });

  it("gives the strip to a takeover, keeping the status and the shortcuts", () => {
    const { actions, bar, shown } = setup([{ label: "Next lead", shortcut: "N" }], {
      takeover: { tone: "error", message: "Call-back overdue" },
      status: <span>Synced</span>,
    });
    expect(bar().getByRole("alert")).toHaveTextContent("Call-back overdue");
    expect(shown()).toEqual([]);
    expect(bar().queryByRole("button", { name: "Customize" })).toBeNull();
    expect(bar().getByText("Synced")).toBeInTheDocument();
    fireEvent.keyDown(document.body, { code: "KeyN" });
    expect(actions[0]!.onSelect).toHaveBeenCalledTimes(1);
  });

  it("refuses two actions on one shortcut", () => {
    vi.spyOn(console, "error").mockImplementation(() => {});
    expect(() => setup([{ label: "Next lead", shortcut: "N" }, { label: "Note", shortcut: "N" }])).toThrow(/bound to N/);
    vi.restoreAllMocks();
  });
});
