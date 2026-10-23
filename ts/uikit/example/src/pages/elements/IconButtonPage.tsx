import { useState } from "react";
import { IconButton, type IconButtonSize, type IconButtonVariant } from "@evinvest/uikit";
import { BellIcon, SearchIcon } from "../../gallery/icons";
import { Block, LIVE_STATES, Matrix, PageRoot, Specimen } from "../../gallery/ui";

const VARIANTS: readonly IconButtonVariant[] = ["secondary", "ghost"];
const SIZES: readonly IconButtonSize[] = ["sm", "md", "touch"];

function Toggled() {
  const [on, setOn] = useState(true);
  return (
    <IconButton aria-label="Notifications" pressed={on} onClick={() => setOn((v) => !v)} tooltip="Toggle notifications">
      <BellIcon />
    </IconButton>
  );
}

export default function IconButtonPage() {
  return (
    <PageRoot title="IconButton" lede="A square button holding one 16 px glyph. It always has an accessible name, and an optional tooltip.">
      <Block title="Styles and sizes" note={LIVE_STATES}>
        <Matrix
          columns={[...SIZES, "Pressed", "Disabled"]}
          rows={VARIANTS.map((variant) => ({
            label: variant,
            cells: [
              ...SIZES.map((size) => (
                <IconButton key={size} aria-label="Search" variant={variant} size={size}>
                  <SearchIcon />
                </IconButton>
              )),
              <IconButton key="on" aria-label="Search" variant={variant} pressed>
                <SearchIcon />
              </IconButton>,
              <IconButton key="off" aria-label="Search" variant={variant} disabled>
                <SearchIcon />
              </IconButton>,
            ],
          }))}
        />
      </Block>
      <Block title="Toggle with a tooltip" note="pressed sets the active fill and aria-pressed.">
        <Specimen>
          <Toggled />
        </Specimen>
      </Block>
    </PageRoot>
  );
}
