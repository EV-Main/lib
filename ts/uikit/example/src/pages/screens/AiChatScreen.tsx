import { Caption, PageRoot } from "../../gallery/ui";
import { DockPanel, FullChat, ThreadList } from "../parts/chat-parts";

export default function AiChatScreen() {
  return (
    <PageRoot
      title="AI chat"
      lede="A thread list beside a conversation, and the same conversation in a 380 px dock. Send a message to stream an answer; Stop ends it."
    >
      <section className="flex min-w-0 flex-col gap-2">
        <Caption>Chat · thread list, conversation, composer</Caption>
        <div className="border-border bg-card flex h-[820px] min-w-0 flex-col overflow-hidden border md:flex-row">
          <ThreadList className="border-border max-h-64 shrink-0 overflow-y-auto border-b md:max-h-none md:w-[260px] md:border-r md:border-b-0" />
          <FullChat />
        </div>
      </section>
      <section className="flex flex-wrap gap-8">
        <div className="flex flex-col gap-2">
          <Caption>Dock · conversation</Caption>
          <DockPanel />
        </div>
        <div className="flex flex-col gap-2">
          <Caption>Dock · empty</Caption>
          <DockPanel empty />
        </div>
      </section>
    </PageRoot>
  );
}
