import {
  Badge,
  Button,
  ListRow,
  ListRows,
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
  Table,
  TableBody,
  TableCard,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@evinvest/uikit";
import { Block, PageRoot, Specimen } from "../../gallery/ui";

const KEYS = [
  { id: "sk_live_7f3a", kind: "api", brands: "aquafix, vifnet", created: "2026-09-30 14:02", revoked: false },
  { id: "sk_live_2b91", kind: "form", brands: "aquafix", created: "2026-09-12 09:41", revoked: true },
  { id: "sk_live_c04e", kind: "api", brands: "kitstart", created: "2026-08-03 18:17", revoked: false },
];

const FILLS = [
  { at: "14:02:11", side: "buy", price: "1.0042", size: "1,250.00", fee: "0.63" },
  { at: "14:01:57", side: "sell", price: "1.0038", size: "310.50", fee: "" },
  { at: "13:59:03", side: "buy", price: "1.0040", size: "12,000.00", fee: "6.02" },
];

function RevokeCell({ revoked }: { revoked: boolean }) {
  return revoked ? (
    <Badge variant="neutral">revoked</Badge>
  ) : (
    <Button size="sm" variant="secondary">
      Revoke
    </Button>
  );
}

function CardList() {
  return (
    <TableCard>
      <Table variant="card">
        <TableHeader>
          <TableRow>
            <TableHead>Key</TableHead>
            <TableHead>Kind</TableHead>
            <TableHead>Brands</TableHead>
            <TableHead align="end">Created</TableHead>
            <TableHead>
              <span className="sr-only">State</span>
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {KEYS.map((k) => (
            <TableRow key={k.id} data-state={k.id === "sk_live_7f3a" ? "selected" : undefined}>
              <TableCell className="text-body-s font-mono">{k.id}</TableCell>
              <TableCell>{k.kind}</TableCell>
              <TableCell className="text-ink-mid">{k.brands}</TableCell>
              <TableCell align="end" className="text-ink-soft">
                {k.created}
              </TableCell>
              <TableCell align="end">
                <RevokeCell revoked={k.revoked} />
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableCard>
  );
}

function PhoneList() {
  return (
    <TableCard className="max-w-sm">
      <ListRows variant="card">
        {KEYS.map((k) => (
          <ListRow key={k.id} label={<span className="text-body-s font-mono">{k.id}</span>} description={`${k.kind} · ${k.brands}`}>
            <RevokeCell revoked={k.revoked} />
          </ListRow>
        ))}
      </ListRows>
    </TableCard>
  );
}

function Ledger() {
  return (
    <Table density="compact">
      <TableHeader>
        <TableRow>
          <TableHead>Time</TableHead>
          <TableHead>Side</TableHead>
          <TableHead align="end">Price</TableHead>
          <TableHead align="end">Size</TableHead>
          <TableHead align="end">Fee</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {FILLS.map((f) => (
          <TableRow key={f.at}>
            <TableCell className="text-ink-soft">{f.at}</TableCell>
            <TableCell className={f.side === "sell" ? "text-accent-error-ink font-medium" : "text-positive-ink font-medium"}>{f.side}</TableCell>
            <TableCell align="end">{f.price}</TableCell>
            <TableCell align="end">{f.size}</TableCell>
            <TableCell align="end">{f.fee || "—"}</TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

export default function TablesPage() {
  return (
    <PageRoot
      title="Tables"
      lede="A card list for records, its phone form as rows, and a compact ledger for numbers. Numbers align on the decimal with tabular figures."
    >
      <Block title="Card list" note="variant=card inside a TableCard. The selected row uses the selected tint.">
        <CardList />
      </Block>
      <Block title="Phone form" note="The same records as ListRows.">
        <PhoneList />
      </Block>
      <Block title="Compact ledger">
        <div className="border-border bg-card overflow-x-auto rounded-lg border">
          <Ledger />
        </div>
      </Block>
      <Block title="Pagination">
        <Specimen>
          <Pagination>
            <PaginationContent>
              <PaginationItem>
                <PaginationPrevious href="#/elements/tables" />
              </PaginationItem>
              <PaginationItem>
                <PaginationLink href="#/elements/tables">1</PaginationLink>
              </PaginationItem>
              <PaginationItem>
                <PaginationLink href="#/elements/tables" isActive>
                  2
                </PaginationLink>
              </PaginationItem>
              <PaginationItem>
                <PaginationLink href="#/elements/tables">3</PaginationLink>
              </PaginationItem>
              <PaginationItem>
                <PaginationEllipsis />
              </PaginationItem>
              <PaginationItem>
                <PaginationNext href="#/elements/tables" />
              </PaginationItem>
            </PaginationContent>
          </Pagination>
        </Specimen>
      </Block>
    </PageRoot>
  );
}
