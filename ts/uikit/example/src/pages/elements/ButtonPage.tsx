import { Button, ButtonGroup, type ButtonSize, type ButtonVariant } from "@evinvest/uikit";
import { PlusIcon } from "../../gallery/icons";
import { Block, LIVE_STATES, Matrix, PageRoot, Specimen } from "../../gallery/ui";

const VARIANTS: readonly ButtonVariant[] = ["primary", "secondary", "outline", "ghost", "destructive", "link"];
const SIZES: readonly ButtonSize[] = ["xs", "sm", "md", "lg", "touch"];

export default function ButtonPage() {
  return (
    <PageRoot
      title="Button"
      lede="Six styles in five sizes. Primary is the filled call to action and the default."
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
          <Button href="#/elements/button" variant="outline">
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
            <Button variant="outline">Day</Button>
            <Button variant="outline">Week</Button>
            <Button variant="outline">Month</Button>
          </ButtonGroup>
        </Specimen>
      </Block>
    </PageRoot>
  );
}
