import { describe, it, expect } from "vitest";
import { render, fireEvent, screen } from "@testing-library/react";
import { IconButton } from "../src/components/icon-button";

describe("IconButton", () => {
  it("is a named button that does not submit forms", () => {
    render(
      <IconButton aria-label="Search">
        <svg />
      </IconButton>,
    );
    const btn = screen.getByRole("button", { name: "Search" });
    expect(btn).toHaveAttribute("type", "button");
    expect(btn).toHaveAttribute("data-slot", "icon-button");
  });

  it("marks the toggled-on state for assistive tech and styling", () => {
    render(
      <IconButton aria-label="Pin" pressed>
        <svg />
      </IconButton>,
    );
    const btn = screen.getByRole("button", { name: "Pin" });
    expect(btn).toHaveAttribute("aria-pressed", "true");
    expect(btn).toHaveClass("bg-active");
  });

  it("leaves aria-pressed off when it is not a toggle", () => {
    render(
      <IconButton aria-label="Pin">
        <svg />
      </IconButton>,
    );
    expect(screen.getByRole("button", { name: "Pin" })).not.toHaveAttribute("aria-pressed");
  });

  it("shows its tooltip on focus", () => {
    render(
      <IconButton aria-label="Copy" tooltip="Copy link">
        <svg />
      </IconButton>,
    );
    expect(screen.queryByRole("tooltip")).toBeNull();
    fireEvent.focus(screen.getByRole("button", { name: "Copy" }));
    expect(screen.getByRole("tooltip")).toHaveTextContent("Copy link");
  });
});
