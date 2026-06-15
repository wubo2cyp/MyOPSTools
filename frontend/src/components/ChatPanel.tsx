import { useState } from "react";
import { Composer } from "./Composer";
import { MessageBubble } from "./MessageBubble";
import { EmptyState } from "./EmptyState";
import { useChatStore } from "@/store/chatStore";

export function ChatPanel() {
  const messages = useChatStore((s) => s.messages);
  const [isRunning, setIsRunning] = useState(false);

  return (
    <div className="flex flex-1 flex-col overflow-hidden">
      <div className="flex-1 overflow-y-auto px-6 py-6">
        {messages.length === 0 ? (
          <EmptyState />
        ) : (
          <div className="mx-auto flex max-w-3xl flex-col gap-4">
            {messages.map((m) => (
              <MessageBubble key={m.id} message={m} />
            ))}
          </div>
        )}
      </div>
      <div className="border-t border-zinc-200 bg-white px-6 py-4">
        <div className="mx-auto max-w-3xl">
          <Composer disabled={isRunning} onRunningChange={setIsRunning} />
        </div>
      </div>
    </div>
  );
}
