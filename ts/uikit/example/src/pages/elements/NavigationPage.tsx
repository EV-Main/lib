import {
  Breadcrumb,
  BreadcrumbEllipsis,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuTrigger,
} from "@evinvest/uikit";
import { Block, PageRoot, Specimen } from "../../gallery/ui";
import { CollapsibleNavSpecimen } from "./collapsible-nav-specimen";
import { ShellFrame } from "./shell-frame";

export default function NavigationPage() {
  return (
    <PageRoot
      title="Navigation"
      lede="Breadcrumbs say where you are, a navigation menu goes elsewhere, and the app shell holds both on a rail, a tab bar and a top bar."
    >
      <Block title="Breadcrumb">
        <Specimen>
          <Breadcrumb>
            <BreadcrumbList>
              <BreadcrumbItem>
                <BreadcrumbLink href="#/elements/navigation">Home</BreadcrumbLink>
              </BreadcrumbItem>
              <BreadcrumbSeparator />
              <BreadcrumbItem>
                <BreadcrumbEllipsis />
              </BreadcrumbItem>
              <BreadcrumbSeparator />
              <BreadcrumbItem>
                <BreadcrumbLink href="#/elements/navigation">Funds</BreadcrumbLink>
              </BreadcrumbItem>
              <BreadcrumbSeparator />
              <BreadcrumbItem>
                <BreadcrumbPage>Allocation</BreadcrumbPage>
              </BreadcrumbItem>
            </BreadcrumbList>
          </Breadcrumb>
        </Specimen>
      </Block>
      <Block title="NavigationMenu">
        <Specimen className="min-h-48 items-start">
          <NavigationMenu>
            <NavigationMenuList>
              <NavigationMenuItem>
                <NavigationMenuTrigger>Products</NavigationMenuTrigger>
                <NavigationMenuContent>
                  <ul className="flex w-56 flex-col gap-1 p-1">
                    <li>
                      <NavigationMenuLink href="#/elements/navigation">Invest</NavigationMenuLink>
                    </li>
                    <li>
                      <NavigationMenuLink href="#/elements/navigation">Wallet</NavigationMenuLink>
                    </li>
                  </ul>
                </NavigationMenuContent>
              </NavigationMenuItem>
              <NavigationMenuItem>
                <NavigationMenuLink href="#/elements/navigation">Docs</NavigationMenuLink>
              </NavigationMenuItem>
            </NavigationMenuList>
          </NavigationMenu>
        </Specimen>
      </Block>
      <Block
        title="Collapsible sections"
        note="Set collapsible on a ShellNav group. The group holding the current page opens itself; the chevron turns and the rows fold. Architecture has a landing page, so its label is a link and only the chevron toggles."
      >
        <Specimen className="items-start">
          <CollapsibleNavSpecimen />
        </Specimen>
      </Block>
      <Block title="App shell" note="Rail from lg, tab bar below. Clicks mark at once; the fake router lands 250 ms later.">
        <ShellFrame />
      </Block>
    </PageRoot>
  );
}
