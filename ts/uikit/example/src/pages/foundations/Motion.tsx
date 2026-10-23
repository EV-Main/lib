import { Block, PageRoot, Specimen } from "../../gallery/ui";
import { DrawerLauncher } from "../elements/overlay-parts";
import {
  EaseDemo,
  MotionTokens,
  PresenceDemo,
  ReducedMotionNote,
  RiseDemo,
  SettledDemo,
  SkeletonDemo,
  StaggerDemo,
  ToastBurst,
} from "./motion-parts";
import { CountOnReveal, DigitsDemo, NumberReducedNote, TickerDemo, TrendDemo } from "./number-parts";

export default function Motion() {
  return (
    <PageRoot
      title="Motion"
      lede="Quiet on purpose: short, small travel. Motion says what changed and never decorates. Every entrance is an attribute, so it needs no animation library."
    >
      <Block title="Tokens" note="One place retunes every animation the kit ships.">
        <MotionTokens />
      </Block>
      <Block title="Rise" note='data-enter="rise": the element fades in and rises, once, on insertion.'>
        <RiseDemo />
      </Block>
      <Block title="Stagger" note='data-enter="stagger": each direct child rises one step after the previous. Rows use 35 ms; the sections of a page use 50 ms, which is what every page in this gallery does.'>
        <StaggerDemo />
      </Block>
      <Block title="Settled" note="The skeleton to content handover. The content rises into place; with no skeleton shown first, it cuts straight in.">
        <SettledDemo />
      </Block>
      <Block title="Skeleton" note="A skeleton fades in before it starts to pulse.">
        <SkeletonDemo />
      </Block>
      <Block title="Easing">
        <EaseDemo />
      </Block>
      <Block title="Numbers" note="AnimatedNumber rolls each digit column to the new value, or counts up with mode=&quot;count&quot;. This ticker steps every 1.5 s, up and down.">
        <TickerDemo />
      </Block>
      <Block title="Count up on reveal" note="animateOnMount starts from 0 (or from). The server markup is the final value.">
        <CountOnReveal />
      </Block>
      <Block title="Trend" note="By default columns spin the way the value moved. trend forces up (1) or down (-1); 0 sends each column the short way.">
        <TrendDemo />
      </Block>
      <Block title="Digits entering and leaving" note="Columns are keyed by place value, so the digits that stay keep their node and only the new places slide in.">
        <DigitsDemo />
      </Block>
      <Block title="Numbers and reduced motion">
        <NumberReducedNote />
      </Block>
      <Block title="Toasts" note="Toasts pile up collapsed and spread on hover. Drag one sideways to flick it away.">
        <ToastBurst />
      </Block>
      <Block title="Drawer" note="The panel slides from its edge on a Vaul-style curve. Drag towards the edge to dismiss.">
        <Specimen>
          <DrawerLauncher />
        </Specimen>
      </Block>
      <Block title="Presence" note="A menu and a dialog animate out as well as in: the node stays until the exit ends.">
        <PresenceDemo />
      </Block>
      <Block title="Reduced motion">
        <ReducedMotionNote />
      </Block>
    </PageRoot>
  );
}
