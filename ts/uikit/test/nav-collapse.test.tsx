import * as React from "react";
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render } from "@testing-library/react";
import { ShellNav } from "../src/components/shell-nav";
import { SectionNav } from "../src/components/section-nav";
import type { NavGroup } from "../src/components/nav-model";

const GROUPS: NavGroup[] = [
  {
    id: "start",
    label: "Getting Started",
    collapsible: true,
    items: [
      { id: "install", href: "/docs/install", label: "Installation" },
      { id: "layouts", href: "/docs/layouts", label: "Layouts" },
    ],
  },
  {
    id: "guides",
    label: "Guides",
    collapsible: true,
    items: [{ id: "auth", href: "/guides/auth", label: "Authentication" }],
  },
  { id: "plain", label: "Plain", items: [{ id: "home", href: "/", label: "Home" }] },
];

const guides = (): NavGroup => {
  const group = GROUPS[1];
  if (!group) throw new Error("fixture");
  return group;
};

// jsdom cannot navigate; a router's link takes the click over in the same way.
const Link = ({ onClick, ...props }: React.ComponentProps<"a">) => (
  <a
    {...props}
    onClick={(e) => {
      onClick?.(e);
      e.preventDefault();
    }}
  />
);

const region = (container: HTMLElement, id: string) =>
  container.querySelector<HTMLElement>(`[id="${id}"]`);

describe("ShellNav collapsible groups", () => {
  it("opens the group holding the current route and closes the others", () => {
    const { getByRole } = render(<ShellNav groups={GROUPS} pathname="/docs/install" />);
    expect(getByRole("button", { name: "Getting Started" })).toHaveAttribute("aria-expanded", "true");
    expect(getByRole("button", { name: "Guides" })).toHaveAttribute("aria-expanded", "false");
  });

  it("toggles on click and takes the closed rows out of the tab order", () => {
    const { getByRole, container } = render(<ShellNav groups={GROUPS} pathname="/docs/install" />);
    const button = getByRole("button", { name: "Guides" });
    const id = button.getAttribute("aria-controls") ?? "";
    expect(region(container, id)).toHaveAttribute("inert");
    fireEvent.click(button);
    expect(button).toHaveAttribute("aria-expanded", "true");
    expect(region(container, id)).not.toHaveAttribute("inert");
    fireEvent.click(button);
    expect(region(container, id)).toHaveAttribute("inert");
  });

  it("opens a closed group when the route moves into it, but not on every render", () => {
    const { getByRole, rerender } = render(<ShellNav groups={GROUPS} pathname="/docs/install" />);
    rerender(<ShellNav groups={GROUPS} pathname="/guides/auth" />);
    expect(getByRole("button", { name: "Guides" })).toHaveAttribute("aria-expanded", "true");
    fireEvent.click(getByRole("button", { name: "Guides" }));
    rerender(<ShellNav groups={GROUPS} pathname="/guides/auth" />);
    expect(getByRole("button", { name: "Guides" })).toHaveAttribute("aria-expanded", "false");
  });

  it("splits a landing page into a link and a named chevron button", () => {
    const groups: NavGroup[] = [{ ...guides(), href: "/guides" }];
    const { getByRole, queryByRole } = render(<ShellNav groups={groups} pathname="/docs" linkComponent={Link} />);
    expect(getByRole("link", { name: "Guides" })).toHaveAttribute("href", "/guides");
    const chevron = getByRole("button", { name: "Expand Guides" });
    expect(chevron).toHaveAttribute("aria-expanded", "false");
    fireEvent.click(getByRole("link", { name: "Guides" }));
    expect(chevron).toHaveAttribute("aria-expanded", "false");
    fireEvent.click(chevron);
    expect(getByRole("button", { name: "Collapse Guides" })).toHaveAttribute("aria-expanded", "true");
    expect(queryByRole("button", { name: "Expand Guides" })).toBeNull();
  });

  it("follows a controlled open prop and reports the request", () => {
    const onOpenChange = vi.fn();
    const closed: NavGroup = { ...guides(), open: false, onOpenChange };
    const { getByRole, rerender } = render(<ShellNav groups={[closed]} pathname="/docs" />);
    fireEvent.click(getByRole("button", { name: "Guides" }));
    expect(onOpenChange).toHaveBeenCalledWith(true);
    expect(getByRole("button", { name: "Guides" })).toHaveAttribute("aria-expanded", "false");
    rerender(<ShellNav groups={[{ ...closed, open: true }]} pathname="/docs" />);
    expect(getByRole("button", { name: "Guides" })).toHaveAttribute("aria-expanded", "true");
  });

  it("honours defaultOpen and leaves non-collapsible groups static", () => {
    const plain = GROUPS[2];
    if (!plain) throw new Error("fixture");
    const { getByRole, queryByRole } = render(
      <ShellNav groups={[{ ...guides(), defaultOpen: true }, plain]} pathname="/" />,
    );
    expect(getByRole("button", { name: "Guides" })).toHaveAttribute("aria-expanded", "true");
    expect(queryByRole("button", { name: "Plain" })).toBeNull();
    expect(getByRole("list", { name: "Plain" })).toBeInTheDocument();
  });

  it("removes every transition under reduced motion", () => {
    const { getByRole, container } = render(<ShellNav groups={GROUPS} pathname="/docs/install" />);
    const id = getByRole("button", { name: "Guides" }).getAttribute("aria-controls") ?? "";
    const outer = region(container, id);
    const animated = [outer, outer?.firstElementChild, container.querySelector("svg")];
    for (const el of animated) expect(el?.getAttribute("class")).toContain("motion-reduce:transition-none");
  });
});

describe("SectionNav collapsible groups", () => {
  it("folds a group and opens the one holding the current section", () => {
    const { getByRole } = render(
      <SectionNav
        value="b"
        groups={[
          { id: "g1", label: "One", collapsible: true, items: [{ id: "a", label: "Alpha" }] },
          { id: "g2", label: "Two", collapsible: true, items: [{ id: "b", label: "Beta" }] },
        ]}
      />,
    );
    expect(getByRole("button", { name: "One" })).toHaveAttribute("aria-expanded", "false");
    expect(getByRole("button", { name: "Two" })).toHaveAttribute("aria-expanded", "true");
  });
});
