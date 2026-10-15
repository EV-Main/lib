import type * as React from "react";
import { describe, it, expect, vi } from "vitest";
import { render, fireEvent } from "@testing-library/react";
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "../src/components/dialog";

function tree(props = {}) {
  return (
    <Dialog {...props}>
      <DialogTrigger>open</DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Title</DialogTitle>
          <DialogDescription>Body</DialogDescription>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}

describe("Dialog", () => {
  it("is closed by default and opens the dialog on trigger", () => {
    const { getByText, queryByRole, getByRole } = render(tree());
    expect(queryByRole("dialog")).toBeNull();
    fireEvent.click(getByText("open"));
    const dialog = getByRole("dialog");
    expect(dialog).toHaveAttribute("aria-modal", "true");
    expect(dialog).toHaveAttribute("data-state", "open");
    expect(getByText("Title")).toBeInTheDocument();
    expect(getByText("Body")).toBeInTheDocument();
  });

  it("closes on Escape", () => {
    const { getByText, queryByRole, getByRole } = render(tree());
    fireEvent.click(getByText("open"));
    expect(getByRole("dialog")).toBeInTheDocument();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(queryByRole("dialog")).toBeNull();
  });

  it("closes when the overlay backdrop is clicked", () => {
    const { getByText, queryByRole, container } = render(tree());
    fireEvent.click(getByText("open"));
    const overlay = container.ownerDocument.querySelector('[data-slot="dialog-overlay"]')!;
    fireEvent.pointerDown(overlay);
    expect(queryByRole("dialog")).toBeNull();
  });

  it("should_call_onOpenChange_false_once_when_overlay_is_pressed", () => {
    const onOpenChange = vi.fn();
    const { baseElement } = render(tree({ open: true, onOpenChange }));
    const overlay = baseElement.querySelector('[data-slot="dialog-overlay"]')!;
    fireEvent.pointerDown(overlay);
    fireEvent.click(overlay);
    expect(onOpenChange).toHaveBeenCalledTimes(1);
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("should_name_dialog_by_its_title", () => {
    const { getByRole } = render(tree({ defaultOpen: true }));
    expect(getByRole("dialog", { name: "Title" })).toHaveAccessibleDescription("Body");
  });

  it("should_not_set_aria_labelledby_without_title", () => {
    const { getByRole } = render(
      <Dialog defaultOpen>
        <DialogContent>
          <p>Untitled body</p>
        </DialogContent>
      </Dialog>,
    );
    expect(getByRole("dialog")).not.toHaveAttribute("aria-labelledby");
  });

  it("should_use_title_own_id_when_given", () => {
    const { getByRole } = render(
      <Dialog defaultOpen>
        <DialogContent>
          <DialogTitle id="rename-title">Rename</DialogTitle>
        </DialogContent>
      </Dialog>,
    );
    const dialog = getByRole("dialog", { name: "Rename" });
    expect(dialog).toHaveAttribute("aria-labelledby", "rename-title");
  });

  it("should_render_DialogTitle_outside_Dialog_without_throwing", () => {
    const { getByRole } = render(<DialogTitle>Standalone</DialogTitle>);
    expect(getByRole("heading", { name: "Standalone" })).toBeInTheDocument();
  });

  describe("initial focus", () => {
    // jsdom measures every element 0×0; give the controls a box so they count as Tab stops.
    function opened(content: React.ReactNode) {
      const spy = vi.spyOn(HTMLElement.prototype, "offsetWidth", "get").mockReturnValue(10);
      const utils = render(
        <Dialog>
          <DialogTrigger>open</DialogTrigger>
          {content}
        </Dialog>,
      );
      fireEvent.click(utils.getByText("open"));
      spy.mockRestore();
      return utils;
    }

    it("lands on the first control by default", () => {
      const { getByLabelText } = opened(
        <DialogContent>
          <DialogTitle>Rename</DialogTitle>
          <input aria-label="Name" />
        </DialogContent>,
      );
      expect(document.activeElement).toBe(getByLabelText("Name"));
    });

    it("lands on the panel with initialFocus=container", () => {
      const { getByRole } = opened(
        <DialogContent initialFocus="container">
          <DialogTitle>Rename</DialogTitle>
          <input aria-label="Name" />
        </DialogContent>,
      );
      expect(document.activeElement).toBe(getByRole("dialog"));
    });
  });
});
