import { useCallback, useRef } from "react";
import { openRunStream } from "@/api/runs";
import { useChatStore } from "@/store/chatStore";
import { useToastStore } from "@/store/toastStore";

/**
 * Hook that drives an SSE run end-to-end.
 *
 * - opens a POST /sessions/:id/runs SSE stream
 * - dispatches every event into the chat store (Zustand)
 * - exposes `start()` and `stop()` (cancels via AbortController)
 */
export function useRunStream() {
  const closeRef = useRef<(() => void) | null>(null);

  const stop = useCallback(() => {
    closeRef.current?.();
    closeRef.current = null;
  }, []);

  const start = useCallback(async (sessionId: string, userMessage: string) => {
    // Cancel any prior run first
    stop();

    const store = useChatStore.getState();
    store.beginStream(`run-${Date.now()}`);

    // Optimistically append user message
    store.appendMessage({
      id: `usr-${Date.now()}`,
      session_id: sessionId,
      role: "user",
      content: userMessage,
      created_at: new Date().toISOString(),
    });

    try {
      const { events, close } = await openRunStream(sessionId, { user_message: userMessage });
      closeRef.current = close;
      for await (const ev of events) {
        // ev.type is set by the SSE parser
        switch (ev.type) {
          case "run.started":
            store.beginStream(ev.run_id);
            break;
          case "message.delta":
            store.appendDelta(ev.delta);
            break;
          case "tool.call":
            store.recordToolCall({ id: ev.id, name: ev.name, arguments: ev.arguments });
            break;
          case "tool.result":
            store.recordToolResult(ev.call_id, ev.output);
            break;
          case "tool.error":
            store.recordToolError(ev.call_id, ev.message);
            break;
          case "message.final":
            store.finalizeStream(ev.message);
            break;
          case "run.finished":
            store.finalizeStream();
            break;
          case "run.error":
            store.failStream(ev.code, ev.message);
            break;
          case "error":
            store.failStream(ev.code, ev.message);
            break;
        }
        if (ev.type === "run.finished" || ev.type === "error") {
          break;
        }
      }
    } catch (e) {
      const err = e as Error;
      if (err.name === "AbortError") {
        // User cancelled; mark as finished without appending error
        useChatStore.getState().finalizeStream();
      } else {
        useChatStore.getState().failStream("network", err.message);
        useToastStore.getState().addToast({
          type: "error",
          message: `运行失败: ${err.message}`,
        });
      }
    } finally {
      closeRef.current = null;
    }
  }, [stop]);

  return { start, stop };
}
