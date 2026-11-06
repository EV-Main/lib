import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { Button, buttonVariants } from "../src/components/atomic/button";
import { sizeClasses } from "../src/generated/size";

describe("Button", () => {
  it("renders the default variant and size", () => {
    const { getByText } = render(<Button>go</Button>);
    const el = getByText("go");
    expect(el.tagName).toBe("BUTTON");
    expect(el).toHaveClass("bg-primary");
    expect(el).toHaveClass("h-(--control-box)", sizeClasses.md);
    expect(el).toHaveAttribute("data-slot", "button");
  });

  it("squares a small icon button (canon parity with Rust)", () => {
    const { getByText } = render(
      <Button size="sm" icon>
        x
      </Button>,
    );
    expect(getByText("x")).toHaveClass(sizeClasses.sm);
    expect(getByText("x")).toHaveClass("aspect-square");
  });

  it("takes the touch step, as a square when icon", () => {
    const { getByText } = render(
      <>
        <Button size="touch">go</Button>
        <Button size="touch" icon>
          x
        </Button>
      </>,
    );
    expect(getByText("go")).toHaveClass(sizeClasses.touch, "px-6");
    expect(getByText("x")).toHaveClass(sizeClasses.touch, "aspect-square", "px-0");
    expect(getByText("x")).not.toHaveClass("px-6");
  });

  it("recolours at an accent rung, per the variant's face", () => {
    const { getByText } = render(<Button accent="warn">x</Button>);
    expect(getByText("x")).toHaveClass("bg-accent-warn");
    expect(getByText("x")).not.toHaveClass("bg-primary");

    const outlined = buttonVariants({ variant: "outline", accent: "warn" });
    expect(outlined).toContain("border-accent-warn/40");
    expect(outlined).not.toContain("bg-accent-warn ");
  });

  it("renders as child when asChild", () => {
    const { getByRole } = render(
      <Button asChild>
        <a href="#">link</a>
      </Button>,
    );
    const el = getByRole("link");
    expect(el.tagName).toBe("A");
    expect(el).toHaveClass("bg-primary");
  });

  it("buttonVariants helper fuses variant, size and className override", () => {
    const cls = buttonVariants({ variant: "ghost", size: "md", className: "px-10" });
    expect(cls).toContain("hover:bg-hover");
    expect(cls).toContain(sizeClasses.md);
    expect(cls).toContain("px-10");
    // the default size's standalone `px-4` is dropped by the override (the
    // `has-[>svg]:px-3` refinement stays — it's a different utility).
    expect(cls).not.toMatch(/(^|\s)px-4(\s|$)/);
  });
});
