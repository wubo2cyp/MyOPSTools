import { useCallback, useRef, useState } from "react";
import { openRunStream } from "@/api/runs";
import type { RunEvent } from "@shared/types";

interface State {
  isRunning: boolean;
  lastError: string | null;
}

export interface RunStreamHandle {
  start: (sessionId: string, userMessage: string) => Promise<void>;
  stop: () => void;
  state: State;
}

/**
 * Hook that drives an SSE run. Forwards every event to the supplied consumer.
 *
 * Usage (M4):
 *   const run = useRunStream({ onEvent: (e) => store.apply(e) });
 *   run.start(sessionId, "hello");
 */
export function useRunStream({ onEvent }: { onEvent: (e: RunEvent) => void }): RunStreamHandle {
  const [state, setState] = useState<State>({ isRunning: false, lastError: null });
  const closeRef = useRef<(() => void) | null>(null);

  const stop = useCallback(() => {
    closeRef.current?.();
    closeRef.current = null;
    setState((s) => ({ ...s, isRunning: false }));
  }, []);

  const start = useCallback(
    async (sessionId: string, userMessage: string) => {
      stop();
      setState({ isRunning: true, lastError: null });
      try {
        const { events, close } = await openRunStream(sessionId, { user_message: userMessage });
        closeRef.current = close;
        for await (const ev of events) {
          onEvent(ev);
          if (ev.type === "run.finished" || ev.type === "error") break;
        }
      } catch (e) {
        setState({ isRunning: false, lastError: (e as Error).message });
      } finally {
        closeRef.current = null;
        setState((s) => ({ ...s, isRunning: false }));
      }
    },
    [onEvent, stop],
  );

  return { start, stop, state };
}
