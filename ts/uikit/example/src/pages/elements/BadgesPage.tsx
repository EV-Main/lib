import { Avatar, AvatarFallback, Badge, Kbd, KbdGroup, StatusItem, type BadgeVariant, type StatusTone } from "@evinvest/uikit";
import { Block, Matrix, PageRoot, Specimen } from "../../gallery/ui";

const BADGES: readonly BadgeVariant[] = ["neutral", "primary", "positive", "info", "warn", "error"];
const TONES: readonly StatusTone[] = ["positive", "neutral", "warn", "error", "muted"];
const SHAPES = ["circle", "square"] as const;
const SIZES = ["sm", "md", "lg"] as const;

export default function BadgesPage() {
  return (
    <PageRoot
      title="Badges and status"
      lede="Small marks that carry a state. Badges are chips: outlined in Dark and Light, tinted and pill-shaped in Oldschool."
    >
      <Block title="Badge" note="Six tones. The tone sets the ink, the edge and the chip fill.">
        <Specimen>
          {BADGES.map((v) => (
            <Badge key={v} variant={v}>
              {v}
            </Badge>
          ))}
        </Specimen>
      </Block>
      <Block title="StatusItem" note="A dot and a short label. The dot is decorative, so the label must say the state.">
        <Specimen>
          {TONES.map((tone) => (
            <StatusItem key={tone} tone={tone}>
              {tone}
            </StatusItem>
          ))}
          <StatusItem tone="positive" value="49ms">
            API
          </StatusItem>
        </Specimen>
      </Block>
      <Block title="Kbd">
        <Specimen>
          <Kbd>K</Kbd>
          <KbdGroup>
            <Kbd>⌘</Kbd>
            <Kbd>K</Kbd>
          </KbdGroup>
          <span className="text-body text-ink-mid">
            Press <Kbd>Esc</Kbd> to close
          </span>
        </Specimen>
      </Block>
      <Block title="Avatar" note="A circle is always round; a square follows the chip radius.">
        <Matrix
          columns={SIZES}
          rows={SHAPES.map((shape) => ({
            label: shape,
            cells: SIZES.map((size) => (
              <Avatar key={size} shape={shape} size={size}>
                <AvatarFallback>J</AvatarFallback>
              </Avatar>
            )),
          }))}
        />
      </Block>
    </PageRoot>
  );
}
