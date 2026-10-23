import {
  Avatar,
  AvatarFallback,
  Button,
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
  IconButton,
  InfoTip,
  InfoTipContent,
  InfoTipTrigger,
  Input,
  Label,
  Popover,
  PopoverContent,
  PopoverTrigger,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@evinvest/uikit";
import { SearchIcon } from "../../gallery/icons";
import { Block, PageRoot, Specimen } from "../../gallery/ui";

export default function TipsPage() {
  return (
    <PageRoot
      title="Tips and popovers"
      lede="Four ways to show more without leaving the page. A tooltip names, a hover card previews, an info tip explains, a popover holds controls."
    >
      <Block title="Tooltip" note="Opens on hover and on focus. It names a control; it never holds anything interactive.">
        <Specimen>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button variant="secondary">Hover me</Button>
            </TooltipTrigger>
            <TooltipContent>Tooltip text</TooltipContent>
          </Tooltip>
          <IconButton aria-label="Search" tooltip="Search markets">
            <SearchIcon />
          </IconButton>
        </Specimen>
      </Block>
      <Block title="HoverCard" note="A preview that opens after a short intent delay.">
        <Specimen>
          <HoverCard>
            <HoverCardTrigger asChild>
              <Button variant="link">@ada</Button>
            </HoverCardTrigger>
            <HoverCardContent>
              <div className="flex items-center gap-3">
                <Avatar shape="circle" size="md">
                  <AvatarFallback>A</AvatarFallback>
                </Avatar>
                <div className="flex flex-col">
                  <span className="text-body text-ink font-medium">Ada Lovelace</span>
                  <span className="text-body-s text-ink-soft">Joined September 2026</span>
                </div>
              </div>
            </HoverCardContent>
          </HoverCard>
        </Specimen>
      </Block>
      <Block title="InfoTip" note="A toggletip for help text beside a label. It works on tap, and the bubble is a live region.">
        <Specimen>
          <span className="text-body text-ink flex items-center gap-1.5">
            Funding rate
            <InfoTip>
              <InfoTipTrigger label="About the funding rate" />
              <InfoTipContent>Paid between longs and shorts every eight hours.</InfoTipContent>
            </InfoTip>
          </span>
        </Specimen>
      </Block>
      <Block title="Popover" note="Moves focus in, traps Tab, and returns focus to the trigger on close.">
        <Specimen>
          <Popover>
            <PopoverTrigger asChild>
              <Button variant="secondary">Set a limit</Button>
            </PopoverTrigger>
            <PopoverContent className="flex w-64 flex-col gap-3">
              <Label htmlFor="limit">Limit price</Label>
              <Input id="limit" defaultValue="2,402.25" />
              <Button size="sm">Apply</Button>
            </PopoverContent>
          </Popover>
        </Specimen>
      </Block>
    </PageRoot>
  );
}
