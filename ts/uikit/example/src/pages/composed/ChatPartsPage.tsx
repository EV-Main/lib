import { useEffect, useState } from "react";
import {
  Button,
  ChatAttachment,
  ChatBubble,
  ChatMarker,
  ChatMessage,
  ChatStreamText,
  ChatTyping,
} from "@evinvest/uikit";
import { Block, Caption, PageRoot, Specimen } from "../../gallery/ui";
import { AssistantComposer, DockPanel, useAssistant } from "../parts/chat-parts";

const SAMPLE = "Messages carry plain text, inline code and links. Bubbles hug their content up to a max width.";

function Stream() {
  const a = useAssistant();
  return (
    <div className="flex w-full max-w-xl flex-col gap-3">
      <div className="flex gap-2">
        <Button size="sm" onClick={() => a.send("Rebalance ABCUSD")} disabled={a.streaming}>
          Simulate an answer
        </Button>
        <Button size="sm" variant="secondary" onClick={a.stop} disabled={!a.streaming}>
          Stop
        </Button>
      </div>
      <ChatMessage role="assistant" name="Assistant" time="13:08">
        {a.streaming ? <ChatMarker type="status" state="running" /> : null}
        {a.shown ? (
          <ChatBubble variant="assistant" streaming={a.streaming}>{a.shown}</ChatBubble>
        ) : a.streaming ? (
          <ChatTyping />
        ) : (
          <Caption>Nothing yet.</Caption>
        )}
      </ChatMessage>
      <Caption>Words arrive one at a time; with reduced motion on they arrive in chunks of eight.</Caption>
    </div>
  );
}

const GROW = "Each new word fades in from a soft blur and settles, while the words already shown stay exactly where they are.";

function GrowingText() {
  const [n, setN] = useState(0);
  const words = GROW.split(" ");
  useEffect(() => {
    if (n >= words.length) return;
    const id = setTimeout(() => setN(n + 1), 140);
    return () => clearTimeout(id);
  }, [n, words.length]);
  return (
    <div className="flex max-w-xl flex-col gap-3">
      <p className="text-body text-ink min-h-12">
        <ChatStreamText text={words.slice(0, n).join(" ")} streaming={n < words.length} />
      </p>
      <Button size="sm" variant="secondary" onClick={() => setN(0)}>Replay</Button>
    </div>
  );
}

function ComposerStates() {
  const noop = () => {};
  return (
    <div className="flex w-full max-w-2xl flex-col gap-4">
      <AssistantComposer streaming={false} onSend={noop} onStop={noop} hint="Enter to send · Shift+Enter for a new line" />
      <AssistantComposer streaming onSend={noop} onStop={noop} hint="Enter to queue · Stop to end" />
    </div>
  );
}

export default function ChatPartsPage() {
  return (
    <PageRoot
      title="Chat parts"
      lede="Bubbles, turns, markers, attachments, a composer and a panel. Messages rise as they arrive, and a streaming answer fills in word by word."
    >
      <Block title="Bubble" note="User and muted hug their text; the assistant has no box.">
        <Specimen className="items-start">
          {(["user", "assistant", "muted"] as const).map((variant) => (
            <div key={variant} className="flex max-w-xs flex-col gap-1">
              <Caption>{variant}</Caption>
              <ChatBubble variant={variant}>{SAMPLE}</ChatBubble>
            </div>
          ))}
        </Specimen>
      </Block>
      <Block title="Message">
        <Specimen className="items-start">
          <div className="flex w-full max-w-xl flex-col gap-4">
            <ChatMessage role="assistant" name="Assistant" time="13:04">
              <ChatBubble variant="assistant">{SAMPLE}</ChatBubble>
            </ChatMessage>
            <div className="flex justify-end">
              <ChatMessage role="user" name="You" time="13:04">
                <ChatBubble variant="user">Summarize my current exposure.</ChatBubble>
              </ChatMessage>
            </div>
          </div>
        </Specimen>
      </Block>
      <Block title="Marker" note="Status, three tool states, and a separator.">
        <Specimen className="items-start">
          <div className="flex w-full max-w-xl flex-col gap-2">
            <ChatMarker type="status" state="running" />
            <ChatMarker type="status" state="done" />
            <ChatMarker type="tool" state="running" title="get_positions" detail="account=main · instrument=ABCUSD" meta="Running…" />
            <ChatMarker type="tool" state="done" title="get_positions" detail="account=main · instrument=ABCUSD" meta="0.4s" />
            <ChatMarker type="tool" state="error" title="get_positions" detail="account=main · instrument=ABCUSD" meta="Failed · 1.2s" />
            <ChatMarker type="separator" />
          </div>
        </Specimen>
      </Block>
      <Block title="Attachment">
        <Specimen>
          <ChatAttachment type="file" name="q3-statement.pdf" meta="PDF · 1.2 MB" onRemove={() => {}} />
          <ChatAttachment type="image" name="exposure-chart.png" meta="PNG · 340 KB" onRemove={() => {}} />
          <ChatAttachment type="image" state="uploading" name="exposure-chart.png" progress={62} onRemove={() => {}} />
        </Specimen>
      </Block>
      <Block title="Composer" note="Default, and streaming: the send button becomes Stop, and Enter still queues a follow-up.">
        <Specimen>
          <ComposerStates />
        </Specimen>
      </Block>
      <Block title="Typing" note="The wait before the first token. Reduced motion shows a still ellipsis.">
        <Specimen className="items-start">
          <ChatTyping />
        </Specimen>
      </Block>
      <Block title="Stream text" note="ChatStreamText on its own: pass the growing string and only the new words animate. A ChatBubble with `streaming` does this for string children.">
        <Specimen className="items-start">
          <GrowingText />
        </Specimen>
      </Block>
      <Block title="Token stream">
        <Specimen className="items-start">
          <Stream />
        </Specimen>
      </Block>
      <Block title="Dock" note="The conversation and the empty state. Send a message in either to stream an answer.">
        <div className="flex flex-wrap gap-6">
          <DockPanel />
          <DockPanel empty />
        </div>
      </Block>
    </PageRoot>
  );
}
