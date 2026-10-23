import { useState } from "react";
import {
  Alert,
  AlertDescription,
  AlertTitle,
  Button,
  ResourceError,
  SystemBanner,
  type AlertVariant,
  type SystemBannerTone,
} from "@evinvest/uikit";
import { Block, PageRoot } from "../../gallery/ui";

const BANNER_TONES: readonly SystemBannerTone[] = ["info", "primary", "warn", "neutral"];
const ALERTS: readonly AlertVariant[] = ["neutral", "info", "positive", "warn", "error"];

function Banners() {
  const [shown, setShown] = useState(true);
  return (
    <div className="border-border flex flex-col overflow-hidden rounded-lg border">
      {BANNER_TONES.map((tone) => (
        <SystemBanner key={tone} tone={tone} action={<Button size="sm" variant="link">Details</Button>}>
          Scheduled maintenance tonight at 22:00 UTC ({tone})
        </SystemBanner>
      ))}
      {shown ? (
        <SystemBanner title="Read-only" onDismiss={() => setShown(false)}>
          Withdrawals are paused until the audit ends.
        </SystemBanner>
      ) : (
        <div className="p-3">
          <Button size="sm" variant="secondary" onClick={() => setShown(true)}>
            Show the dismissible banner again
          </Button>
        </div>
      )}
    </div>
  );
}

export default function BannersPage() {
  return (
    <PageRoot
      title="Banners and alerts"
      lede="A banner is a full-width strip above a page. An alert sits inside the page, next to what it is about."
    >
      <Block title="Banner" note="The fill is always the surface with a bottom hairline. Only the icon carries the tone.">
        <Banners />
      </Block>
      <Block title="Alert">
        <div className="flex max-w-xl flex-col gap-3">
          {ALERTS.map((variant) => (
            <Alert key={variant} variant={variant}>
              <AlertTitle>{variant[0]?.toUpperCase()}{variant.slice(1)}</AlertTitle>
              <AlertDescription>This kit mirrors the Rust crate's semantics.</AlertDescription>
            </Alert>
          ))}
        </div>
      </Block>
      <Block title="Resource error" note="A failed read with a retry.">
        <div className="max-w-xl">
          <ResourceError message="Couldn't load the activity feed." onRetry={() => {}} />
        </div>
      </Block>
    </PageRoot>
  );
}
