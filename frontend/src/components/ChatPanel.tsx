import { Composer } from "./Composer";
import { MessageBubble } from "./MessageBubble";
import { ToolCallCard } from "./ToolCallCard";
import { EmptyState } from "./EmptyState";
import { ErrorBanner } from "./ErrorBanner";
import { useChatStore } from "@/store/chatStore";
import { useRunStream } from "@/hooks/useRunStream";
import { Loader2 } from "lucide-react";

export function ChatPanel() {
  const messages = useChatStore((s) => s.messages);
  const streaming = useChatStore((s) => s.streaming);
  const isRunning = useChatStore((s) => s.isRunning);
  const activeSessionId = useChatStore((s) => s.activeSessionId);
  const error = useChatStore((s) => s.error);
  const clearError = useChatStore((s) => s.clearError);
  const { start, stop } = useRunStream();

  const handleSend = async (text: string) => {
    if (!activeSessionId) return;
    await start(activeSessionId, text);
  };

  return (
    <div className="flex flex-1 flex-col overflow-hidden">
      <div className="flex-1 overflow-y-auto px-6 py-6">
        {error && (
          <ErrorBanner
            code={error.code}
            message={error.message}
            onClose={clearError}
          />
        )}
        {messages.length === 0 && !streaming ? (
          <EmptyState />
        ) : (
          <div className="mx-auto flex max-w-3xl flex-col gap-4">
            {messages.map((m) => (
              <MessageBubble key={m.id} message={m} />
            ))}

            {/* Live streaming assistant turn */}
            {streaming && (
              <div className="flex gap-3">
                <div
                  className="flex h-7 w-7 shrink-0 items-center justify-center rounded-full bg-accent/10 text-accent"
                  aria-hidden
                >
                  {isRunning ? <Loader2 size={14} className="animate-spin" /> : "🤖"}
                </div>
                <div className="max-w-[80%] rounded-2xl bg-white px-4 py-2.5 text-sm leading-relaxed shadow-sm ring-1 ring-zinc-200 dark:bg-zinc-800 dark:ring-zinc-700">
                  <div className="mb-1 text-[10px] font-medium uppercase tracking-widest opacity-60 dark:text-zinc-400">
                    Agent {isRunning && <span className="ml-1 text-accent">streaming…</span>}
                  </div>
                  {streaming.text && (
                    <div className="whitespace-pre-wrap text-zinc-800 dark:text-zinc-100">
                      {streaming.text}
                      {isRunning && <span className="ml-0.5 inline-block h-3 w-1.5 animate-pulse bg-accent align-middle" />}
                    </div>
                  )}
                  {streaming.toolCalls.length > 0 && (
                    <div className="mt-2 flex flex-col gap-2">
                      {streaming.toolCalls.map((tc) => (
                        <ToolCallCard
                          key={tc.call.id}
                          call={tc.call}
                          result={tc.output ? { output: tc.output } : undefined}
                          error={tc.error}
                        />
                      ))}
                    </div>
                  )}
                </div>
              </div>
            )}
          </div>
        )}
      </div>
      <div className="border-t border-zinc-200 bg-white px-6 py-4 dark:border-zinc-800 dark:bg-zinc-900">
        <div className="mx-auto max-w-3xl">
          <Composer
            disabled={isRunning || !activeSessionId}
            isRunning={isRunning}
            onSend={handleSend}
            onStop={stop}
          />
        </div>
      </div>
    </div>
  );
}
