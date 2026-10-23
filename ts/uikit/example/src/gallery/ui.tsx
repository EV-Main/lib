import type { ComponentType, ReactNode } from "react";
import { Card, CardContent, Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@evinvest/uikit";

/**
 * Root of every gallery page. The shell keys it by route, so the staggered
 * entrance (`.page-root` sets the section step) replays on each navigation.
 */
export function PageRoot({ title, lede, children }: { title: string; lede?: ReactNode; children: ReactNode }) {
  return (
    <div data-enter="stagger" className="page-root flex flex-col gap-10 p-4 lg:p-8">
      <header className="flex max-w-3xl flex-col gap-2">
        <h1 className="font-display text-h1 text-ink font-light">{title}</h1>
        {lede ? <p className="text-body-l text-ink-mid">{lede}</p> : null}
      </header>
      {children}
    </div>
  );
}

export function Block({
  title,
  note,
  actions,
  children,
}: {
  title: string;
  note?: ReactNode;
  actions?: ReactNode;
  children: ReactNode;
}) {
  return (
    <section className="flex min-w-0 flex-col gap-3">
      <div className="flex flex-wrap items-end justify-between gap-x-4 gap-y-2">
        <div className="flex flex-col gap-0.5">
          <h2 className="text-h3 text-ink font-medium">{title}</h2>
          {note ? <p className="text-body-s text-ink-soft max-w-prose">{note}</p> : null}
        </div>
        {actions}
      </div>
      {children}
    </section>
  );
}

/** A bordered plane that specimens sit on; `bg-card` is the first surface step. */
export function Specimen({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <Card className={`gap-0 py-4 shadow-card ${className ?? ""}`}>
      <CardContent className="flex flex-wrap items-center gap-3 px-4">{children}</CardContent>
    </Card>
  );
}

export interface MatrixRow {
  label: string;
  cells: readonly ReactNode[];
}

/** States (or variants) down the side, one column per size or kind. */
export function Matrix({ columns, rows }: { columns: readonly string[]; rows: readonly MatrixRow[] }) {
  return (
    <div className="border-border bg-card shadow-card overflow-x-auto rounded-lg border">
      <Table density="compact">
        <TableHeader>
          <TableRow>
            <TableHead className="text-ink-soft w-28">
              <span className="sr-only">Row</span>
            </TableHead>
            {columns.map((c) => (
              <TableHead key={c} className="text-ink-soft">
                {c}
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {rows.map((r) => (
            <TableRow key={r.label} className="hover:bg-transparent">
              <TableCell className="text-body-s text-ink-soft">{r.label}</TableCell>
              {r.cells.map((cell, i) => (
                <TableCell key={columns[i]} className="py-2">
                  {cell}
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}

/** Hover, focus and press cannot be pinned from CSS, so the matrices say they are live. */
export const LIVE_STATES = "Hover, focus (Tab) and press are live on every specimen. Disabled and error are pinned.";

export function Caption({ children }: { children: ReactNode }) {
  return <span className="text-caption text-ink-soft">{children}</span>;
}

/** Stands in for an element family that executor B has not exported yet. */
export function waitingFor(name: string): ComponentType {
  return function Waiting() {
    return (
      <PageRoot title={name} lede={`${name} is built by the component track and is not exported from the kit yet. This page fills in when it lands.`}>
        <Block title="Status">
          <Specimen>
            <span className="text-body text-ink-mid">Waiting for the export.</span>
          </Specimen>
        </Block>
      </PageRoot>
    );
  };
}
