import { ToggleGroup, ToggleGroupItem } from "@evinvest/uikit";
import { THEMES, THEME_LABEL, isTheme, setTheme, type Theme } from "./theme";

/** `compact` shows the initial on a phone, where the app bar has little room. */
export function ThemeSwitch({ theme, compact = false }: { theme: Theme; compact?: boolean }) {
  return (
    <ToggleGroup
      type="single"
      variant="outline"
      size="sm"
      value={theme}
      aria-label="Theme"
      onValueChange={(v) => {
        if (typeof v === "string" && isTheme(v)) setTheme(v);
      }}
    >
      {THEMES.map((t) => (
        <ToggleGroupItem key={t} value={t} aria-label={THEME_LABEL[t]}>
          {compact ? THEME_LABEL[t].slice(0, 1) : THEME_LABEL[t]}
        </ToggleGroupItem>
      ))}
    </ToggleGroup>
  );
}
