import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { CommandPalette, useCommandPaletteShortcut } from "../src/components/command-palette";

const groups = (onNew = vi.fn(), onSearch = vi.fn()) => [
  { id: "s", heading: "Suggestions", items: [{ id: "new", label: "New workspace", shortcut: "⌘N", onSelect: onNew }] },
  { id: "t", heading: "Settings", items: [{ id: "find", label: "Search symbol", keywords: ["ticker"], onSelect: onSearch }] },
];

describe("CommandPalette", () => {
  it("is a named dialog with a search box and the key hints", () => {
    render(<CommandPalette open groups={groups()} />);
    expect(screen.getByRole("dialog", { name: "Command palette" })).toBeInTheDocument();
    expect(screen.getByRole("combobox")).toHaveAttribute("placeholder", "Type a command or search…");
    expect(screen.getByText("navigate")).toBeInTheDocument();
    expect(screen.getByText("esc")).toBeInTheDocument();
  });

  it("runs the chosen action and closes", () => {
    const onNew = vi.fn();
    const onOpenChange = vi.fn();
    render(<CommandPalette open onOpenChange={onOpenChange} groups={groups(onNew)} />);
    fireEvent.click(screen.getByRole("option", { name: /New workspace/ }));
    expect(onNew).toHaveBeenCalledTimes(1);
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("filters by label and by keywords, and says so when nothing matches", () => {
    render(<CommandPalette open groups={groups()} />);
    const box = screen.getByRole("combobox");
    fireEvent.change(box, { target: { value: "ticker" } });
    expect(screen.getByRole("option", { name: /Search symbol/ })).toBeInTheDocument();
    expect(screen.queryByRole("option", { name: /New workspace/ })).toBeNull();
    fireEvent.change(box, { target: { value: "zzzz" } });
    expect(screen.getByText("No results.")).toBeInTheDocument();
  });

  it("runs the highlighted row on Enter", () => {
    const onNew = vi.fn();
    render(<CommandPalette open groups={groups(onNew)} />);
    fireEvent.keyDown(screen.getByRole("combobox"), { key: "Enter" });
    expect(onNew).toHaveBeenCalledTimes(1);
  });
});

function Probe({ onTrigger, enabled }: { onTrigger: () => void; enabled?: boolean }) {
  useCommandPaletteShortcut(onTrigger, enabled);
  return null;
}

describe("useCommandPaletteShortcut", () => {
  it("fires on Ctrl+K and Cmd+K and prevents the browser default", () => {
    const onTrigger = vi.fn();
    render(<Probe onTrigger={onTrigger} />);
    const ctrl = new KeyboardEvent("keydown", { key: "k", ctrlKey: true, cancelable: true });
    document.dispatchEvent(ctrl);
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "K", metaKey: true }));
    expect(onTrigger).toHaveBeenCalledTimes(2);
    expect(ctrl.defaultPrevented).toBe(true);
  });

  it("ignores a bare K and is silent when disabled", () => {
    const onTrigger = vi.fn();
    const { rerender } = render(<Probe onTrigger={onTrigger} />);
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "k" }));
    rerender(<Probe onTrigger={onTrigger} enabled={false} />);
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "k", ctrlKey: true }));
    expect(onTrigger).not.toHaveBeenCalled();
  });
});
