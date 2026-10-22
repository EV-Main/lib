import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { WorkspaceTabs, WorkspaceTab } from "../src/components/workspace-tab";

function setup(onClose = vi.fn()) {
  render(
    <WorkspaceTabs defaultValue="a">
      <WorkspaceTab value="a">Alpha</WorkspaceTab>
      <WorkspaceTab value="b" onClose={onClose} closeLabel="Close Beta">
        Beta
      </WorkspaceTab>
      <WorkspaceTab value="c">Gamma</WorkspaceTab>
    </WorkspaceTabs>,
  );
  return onClose;
}

describe("WorkspaceTabs", () => {
  it("exposes a tablist whose selected tab is the only tab stop", () => {
    setup();
    expect(screen.getByRole("tablist")).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: "Alpha" })).toHaveAttribute("aria-selected", "true");
    expect(screen.getByRole("tab", { name: "Alpha" })).toHaveAttribute("tabindex", "0");
    expect(screen.getByRole("tab", { name: "Beta" })).toHaveAttribute("tabindex", "-1");
  });

  it("selects a tab on click", () => {
    setup();
    fireEvent.click(screen.getByRole("tab", { name: "Gamma" }));
    expect(screen.getByRole("tab", { name: "Gamma" })).toHaveAttribute("aria-selected", "true");
  });

  it("moves focus with arrows and wraps, without selecting", () => {
    setup();
    screen.getByRole("tab", { name: "Alpha" }).focus();
    fireEvent.keyDown(screen.getByRole("tab", { name: "Alpha" }), { key: "ArrowLeft" });
    expect(screen.getByRole("tab", { name: "Gamma" })).toHaveFocus();
    expect(screen.getByRole("tab", { name: "Alpha" })).toHaveAttribute("aria-selected", "true");
    fireEvent.keyDown(screen.getByRole("tab", { name: "Gamma" }), { key: "Home" });
    expect(screen.getByRole("tab", { name: "Alpha" })).toHaveFocus();
  });

  it("closes through the labelled button", () => {
    const onClose = setup();
    fireEvent.click(screen.getByRole("button", { name: "Close Beta" }));
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("closes a closable tab on Delete and Backspace, and ignores them elsewhere", () => {
    const onClose = setup();
    fireEvent.keyDown(screen.getByRole("tab", { name: "Beta" }), { key: "Delete" });
    fireEvent.keyDown(screen.getByRole("tab", { name: "Beta" }), { key: "Backspace" });
    expect(onClose).toHaveBeenCalledTimes(2);
    fireEvent.keyDown(screen.getByRole("tab", { name: "Alpha" }), { key: "Delete" });
    expect(onClose).toHaveBeenCalledTimes(2);
  });
});
