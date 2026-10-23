import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import {
  Card,
  CardHeader,
  CardTitle,
  CardFooter,
} from "../src/components/card";

describe("Card", () => {
  it("renders the base card with its slot", () => {
    const { getByText } = render(<Card>body</Card>);
    const el = getByText("body");
    expect(el).toHaveClass("bg-card");
    expect(el).toHaveAttribute("data-slot", "card");
  });

  it("header carries the landing container canon", () => {
    const { getByText } = render(<CardHeader>h</CardHeader>);
    const el = getByText("h");
    expect(el).toHaveClass("@container/card-header");
    expect(el).toHaveClass("has-data-[slot=card-action]:grid-cols-[1fr_auto]");
  });

  it("title is the h3 style", () => {
    const { getByText } = render(<CardTitle>t</CardTitle>);
    const el = getByText("t");
    expect(el).toHaveClass("text-h3", "font-medium");
    expect(el).toHaveAttribute("data-slot", "card-title");
  });

  it("footer keeps the border-t rule", () => {
    const { getByText } = render(<CardFooter>f</CardFooter>);
    expect(getByText("f")).toHaveClass("[.border-t]:pt-4");
  });
});
