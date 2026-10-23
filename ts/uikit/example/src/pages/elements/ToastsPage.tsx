import { Button, toast, type ToastVariant } from "@evinvest/uikit";
import { TOAST_POSITIONS, setToastPosition, useToastPosition } from "../../gallery/toast-position";
import { Block, PageRoot, Specimen } from "../../gallery/ui";

const TONES: readonly ToastVariant[] = ["neutral", "positive", "info", "warn", "error"];

function fire(variant: ToastVariant): void {
  const title = `${variant[0]?.toUpperCase() ?? ""}${variant.slice(1)} toast`;
  // single line, to match the Dioxus port (Rust toasts have no description)
  if (variant === "neutral") toast(title);
  else toast[variant](title);
}

export default function ToastsPage() {
  const position = useToastPosition();
  return (
    <PageRoot
      title="Toasts"
      lede="Toasts pile up collapsed and spread on hover. Drag one sideways to flick it away. The stack is one Toaster in the shell."
    >
      <Block title="Tones">
        <Specimen>
          {TONES.map((tone) => (
            <Button key={tone} size="sm" variant={tone === "error" ? "danger" : "secondary"} onClick={() => fire(tone)}>
              {tone}
            </Button>
          ))}
          <Button
            size="sm"
            onClick={() => {
              fire("positive");
              fire("info");
              fire("warn");
            }}
          >
            Stack x3
          </Button>
          <Button size="sm" variant="ghost" onClick={() => toast("Persistent: it will not close by itself", { duration: Infinity })}>
            Persistent
          </Button>
        </Specimen>
      </Block>
      <Block title="Position">
        <Specimen>
          {TOAST_POSITIONS.map((p) => (
            <Button key={p} size="sm" variant={p === position ? "solid" : "secondary"} onClick={() => setToastPosition(p)}>
              {p}
            </Button>
          ))}
        </Specimen>
      </Block>
    </PageRoot>
  );
}
