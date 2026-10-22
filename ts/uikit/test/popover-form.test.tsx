import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { PopoverForm } from "../src/components/popover-form";

function setup(props: Partial<React.ComponentProps<typeof PopoverForm>> = {}) {
  render(
    <PopoverForm trigger={<button type="button">Filter</button>} title="Status filter" {...props}>
      <label>
        Only favorites
        <input type="checkbox" defaultChecked />
      </label>
    </PopoverForm>,
  );
}

describe("PopoverForm", () => {
  it("stays closed until the trigger is pressed", () => {
    setup();
    expect(screen.queryByRole("dialog")).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Filter" }));
    expect(screen.getByRole("dialog", { name: "Status filter" })).toBeInTheDocument();
  });

  it("submits, reports it, and closes", () => {
    const onSubmit = vi.fn();
    setup({ onSubmit });
    fireEvent.click(screen.getByRole("button", { name: "Filter" }));
    fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    expect(onSubmit).toHaveBeenCalledTimes(1);
    expect(screen.getByRole("button", { name: "Filter" })).toHaveAttribute("aria-expanded", "false");
  });

  it("resets the fields without closing", () => {
    const onReset = vi.fn();
    setup({ onReset, defaultOpen: true });
    const box = screen.getByLabelText("Only favorites") as HTMLInputElement;
    fireEvent.click(box);
    expect(box.checked).toBe(false);
    fireEvent.click(screen.getByRole("button", { name: "Reset" }));
    expect(onReset).toHaveBeenCalledTimes(1);
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });

  it("takes translated button labels", () => {
    setup({ defaultOpen: true, labels: { submit: "Применить", reset: "Сбросить" } });
    expect(screen.getByRole("button", { name: "Применить" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Сбросить" })).toBeInTheDocument();
  });
});
