import {
  Badge,
  Button,
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
  Item,
  ItemActions,
  ItemContent,
  ItemDescription,
  ItemGroup,
  ItemMedia,
  ItemSeparator,
  ItemTitle,
  KeyValue,
  ListRow,
  ListRows,
} from "@evinvest/uikit";
import { WalletIcon } from "../../gallery/icons";
import { Block, PageRoot } from "../../gallery/ui";

export default function CardsPage() {
  return (
    <PageRoot
      title="Cards and lists"
      lede="A card is a surface step with a hairline. Lists of records use ListRow on a phone and Item for a row with media and actions."
    >
      <Block title="Card">
        <Card className="max-w-md">
          <CardHeader>
            <CardTitle>Open positions</CardTitle>
            <CardDescription>Net exposure is 38% of equity.</CardDescription>
            <CardAction>
              <Badge variant="positive">live</Badge>
            </CardAction>
          </CardHeader>
          <CardContent>
            <span className="font-display text-figure-l text-ink font-light tabular-nums">2,398.73</span>
          </CardContent>
          <CardFooter className="gap-2">
            <Button size="sm">Add funds</Button>
            <Button size="sm" variant="secondary">
              Details
            </Button>
          </CardFooter>
        </Card>
      </Block>
      <Block title="ListRow">
        <div className="max-w-md">
          <ListRows variant="card">
            <ListRow label="Stage" value="new" />
            <ListRow label="Price" value="1,250" />
            <ListRow label="Owner" description="Assigned 3 days ago">
              <Badge variant="info">Ada</Badge>
            </ListRow>
          </ListRows>
        </div>
      </Block>
      <Block title="Item">
        <ItemGroup className="max-w-md">
          <Item>
            <ItemMedia variant="icon">
              <WalletIcon />
            </ItemMedia>
            <ItemContent>
              <ItemTitle>Main wallet</ItemTitle>
              <ItemDescription>USD · updated 13:06</ItemDescription>
            </ItemContent>
            <ItemActions>
              <Button size="sm" variant="secondary">
                Open
              </Button>
            </ItemActions>
          </Item>
          <ItemSeparator />
          <Item>
            <ItemContent>
              <ItemTitle>Savings</ItemTitle>
              <ItemDescription>USD · updated yesterday</ItemDescription>
            </ItemContent>
          </Item>
        </ItemGroup>
      </Block>
      <Block title="KeyValue" note="A pair with a copy button.">
        <div className="max-w-md">
          <KeyValue label="Account ID" value="acc_7f3a91c2" />
        </div>
      </Block>
    </PageRoot>
  );
}
