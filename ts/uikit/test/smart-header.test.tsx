import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { SmartHeader, type SmartHeaderState } from "../src/components/smart-header";

function view(state: SmartHeaderState) {
  return render(
    <SmartHeader
      state={state}
      start={<span>Brand</span>}
      end={<span>Account</span>}
      collapsed={<span>Folded</span>}
      banner={<div role="status">Feed delayed</div>}
      toolbar={<span>Toolbar</span>}
      context={<span>Context</span>}
      palette={<div role="dialog">Palette</div>}
    />,
  );
}

const shown = (...names: string[]) => names.filter((n) => screen.queryByText(n) !== null);
const ALL = ["Toolbar", "Context", "Folded", "Feed delayed", "Palette"];

describe("SmartHeader", () => {
  it("keeps the bar in every state", () => {
    for (const state of ["default", "collapsed", "banner", "search"] as const) {
      const { unmount } = view(state);
      expect(shown("Brand", "Account")).toEqual(["Brand", "Account"]);
      unmount();
    }
  });

  it("default shows toolbar and context line only", () => {
    view("default");
    expect(shown(...ALL)).toEqual(["Toolbar", "Context"]);
  });

  it("collapsed folds the toolbar into the bar", () => {
    view("collapsed");
    expect(shown(...ALL)).toEqual(["Folded"]);
  });

  it("banner puts the banner above the bar", () => {
    const { container } = view("banner");
    expect(shown(...ALL)).toEqual(["Feed delayed"]);
    const root = container.firstElementChild as HTMLElement;
    expect(root.firstElementChild).toHaveTextContent("Feed delayed");
  });

  it("search drops the palette under the bar", () => {
    const { container } = view("search");
    expect(shown(...ALL)).toEqual(["Palette"]);
    expect(container.firstElementChild).toHaveAttribute("data-state", "search");
  });
});
