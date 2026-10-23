import { Button, ButtonGroup, type ButtonSize, type ButtonVariant } from "@evinvest/uikit";
import { PlusIcon } from "../../gallery/icons";
import { Block, LIVE_STATES, Matrix, PageRoot, Specimen } from "../../gallery/ui";

const VARIANTS: readonly ButtonVariant[] = ["primary", "secondary", "ghost", "solid", "danger", "link"];
const SIZES: readonly ButtonSize[] = ["sm", "md", "touch"];

export default function ButtonPage() {
  return (
    <PageRoot
      title="Button"
      lede="Six styles in three sizes. Primary and danger are outlined in Dark and Light and filled in Oldschool; solid is the one filled button."
    >
      <Block title="Styles and sizes" note={LIVE_STATES}>
        <Matrix
          columns={[...SIZES, "With icon", "Disabled"]}
          rows={VARIANTS.map((variant) => ({
            label: variant,
            cells: [
              ...SIZES.map((size) => (
                <Button key={size} variant={variant} size={size}>
                  Label
                </Button>
              )),
              <Button key="icon" variant={variant}>
                <PlusIcon />
                Add
              </Button>,
              <Button key="off" variant={variant} disabled>
                Label
              </Button>,
            ],
          }))}
        />
      </Block>
      <Block title="As a link" note="With an href the button renders an anchor wearing the same classes.">
        <Specimen>
          <Button href="#/elements/button">Anchor</Button>
          <Button href="#/elements/button" variant="secondary">
            Anchor
          </Button>
          <Button href="#/elements/button" variant="link">
            Link
          </Button>
        </Specimen>
      </Block>
      <Block title="Button group">
        <Specimen>
          <ButtonGroup>
            <Button variant="secondary">Day</Button>
            <Button variant="secondary">Week</Button>
            <Button variant="secondary">Month</Button>
          </ButtonGroup>
        </Specimen>
      </Block>
    </PageRoot>
  );
}
