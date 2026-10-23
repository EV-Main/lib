import { useState, type ComponentProps, type MouseEvent } from "react";
import { ShellNav, type NavGroup } from "@evinvest/uikit";

const GROUPS: NavGroup[] = [
  {
    id: "getting-started",
    label: "Getting Started",
    collapsible: true,
    items: [
      { id: "installation", href: "/docs/installation", label: "Installation" },
      { id: "structure", href: "/docs/project-structure", label: "Project Structure" },
      { id: "layouts", href: "/docs/layouts-and-pages", label: "Layouts and Pages" },
      { id: "linking", href: "/docs/linking-and-navigating", label: "Linking and Navigating" },
      { id: "server-client", href: "/docs/server-and-client", label: "Server and Client Components" },
    ],
  },
  {
    id: "guides",
    label: "Guides",
    collapsible: true,
    items: [
      { id: "auth", href: "/guides/authentication", label: "Authentication" },
      { id: "caching", href: "/guides/caching", label: "Caching" },
      { id: "forms", href: "/guides/forms", label: "Forms" },
    ],
  },
  {
    id: "architecture",
    label: "Architecture",
    collapsible: true,
    href: "/architecture",
    items: [
      { id: "accessibility", href: "/architecture/accessibility", label: "Accessibility" },
      { id: "fast-refresh", href: "/architecture/fast-refresh", label: "Fast Refresh" },
    ],
  },
];

/** The pattern with the Next.js docs copy; a local "router" so a click moves the current row. */
export function CollapsibleNavSpecimen() {
  const [pathname, setPathname] = useState("/docs/installation");
  const [Link] = useState(
    () =>
      function DemoLink({ href, onClick, ...props }: ComponentProps<"a">) {
        return (
          <a
            href={href}
            {...props}
            onClick={(e: MouseEvent<HTMLAnchorElement>) => {
              onClick?.(e);
              e.preventDefault();
              if (href) setPathname(href);
            }}
          />
        );
      },
  );
  return (
    <div className="h-[28rem] overflow-hidden rounded-lg border border-border">
      <ShellNav groups={GROUPS} pathname={pathname} linkComponent={Link} labels={{ primary: "Docs" }} />
    </div>
  );
}
