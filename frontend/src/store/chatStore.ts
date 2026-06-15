import { create } from "zustand";
import type { Message, Session } from "@shared/types";

interface ChatState {
  sessions: Session[];
  messages: Message[];
  setSessions: (s: Session[]) => void;
  appendMessage: (m: Message) => void;
  appendDelta: (delta: string) => void;
  reset: () => void;
}

export const useChatStore = create<ChatState>((set) => ({
  sessions: [],
  messages: [],
  setSessions: (sessions) => set({ sessions }),
  appendMessage: (m) => set((s) => ({ messages: [...s.messages, m] })),
  appendDelta: (delta) =>
    set((s) => {
      const last = s.messages[s.messages.length - 1];
      if (last && last.role === "assistant") {
        const updated: Message = { ...last, content: last.content + delta };
        return { messages: [...s.messages.slice(0, -1), updated] };
      }
      const stub: Message = {
        id: `tmp-${Date.now()}`,
        session_id: "",
        role: "assistant",
        content: delta,
        created_at: new Date().toISOString(),
      };
      return { messages: [...s.messages, stub] };
    }),
  reset: () => set({ messages: [] }),
}));
