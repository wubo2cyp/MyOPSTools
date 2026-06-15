import { create } from "zustand";
import type { Message, Session, ToolCall } from "@shared/types";

export interface ToolCallState {
  call: ToolCall;
  output?: string;
  error?: string;
}

export interface AssistantBuffer {
  id: string;
  toolCalls: ToolCallState[];
  text: string;
}

interface ChatState {
  sessions: Session[];
  activeSessionId: string | null;
  messages: Message[];
  /** Pending tool call results keyed by tool call id (consumed when assistant finalises). */
  toolResults: Record<string, { output?: string; error?: string }>;
  /** Transient streaming buffer for the active assistant turn. */
  streaming: AssistantBuffer | null;
  isRunning: boolean;

  setSessions: (s: Session[]) => void;
  setActiveSession: (id: string | null) => void;
  setMessages: (msgs: Message[]) => void;
  appendMessage: (m: Message) => void;

  // Streaming controls
  beginStream: (runId: string) => void;
  appendDelta: (delta: string) => void;
  recordToolCall: (call: ToolCall) => void;
  recordToolResult: (callId: string, output: string) => void;
  recordToolError: (callId: string, error: string) => void;
  finalizeStream: (final?: Message) => void;
  failStream: (code: string, message: string) => void;
  setRunning: (running: boolean) => void;

  reset: () => void;
}

let _tmpCounter = 0;
const newId = () => `tmp-${Date.now()}-${_tmpCounter++}`;

export const useChatStore = create<ChatState>((set) => ({
  sessions: [],
  activeSessionId: null,
  messages: [],
  toolResults: {},
  streaming: null,
  isRunning: false,

  setSessions: (sessions) => set({ sessions }),
  setActiveSession: (id) => set({ activeSessionId: id, messages: [], streaming: null, toolResults: {} }),
  setMessages: (messages) => set({ messages }),
  appendMessage: (m) => set((s) => ({ messages: [...s.messages, m] })),

  beginStream: (runId) =>
    set({
      streaming: { id: runId, toolCalls: [], text: "" },
      isRunning: true,
    }),

  appendDelta: (delta) =>
    set((s) => {
      if (!s.streaming) return s;
      return {
        streaming: {
          ...s.streaming,
          text: s.streaming.text + delta,
        },
      };
    }),

  recordToolCall: (call) =>
    set((s) => {
      if (!s.streaming) return s;
      return {
        streaming: {
          ...s.streaming,
          toolCalls: [...s.streaming.toolCalls, { call }],
        },
      };
    }),

  recordToolResult: (callId, output) =>
    set((s) => {
      const toolResults = { ...s.toolResults, [callId]: { output } };
      let streaming = s.streaming;
      if (streaming) {
        streaming = {
          ...streaming,
          toolCalls: streaming.toolCalls.map((tc) =>
            tc.call.id === callId ? { ...tc, output } : tc,
          ),
        };
      }
      return { toolResults, streaming };
    }),

  recordToolError: (callId, error) =>
    set((s) => {
      const toolResults = { ...s.toolResults, [callId]: { error } };
      let streaming = s.streaming;
      if (streaming) {
        streaming = {
          ...streaming,
          toolCalls: streaming.toolCalls.map((tc) =>
            tc.call.id === callId ? { ...tc, error } : tc,
          ),
        };
      }
      return { toolResults, streaming };
    }),

  finalizeStream: (final) =>
    set((s) => {
      const streaming = s.streaming;
      if (!streaming) {
        return { isRunning: false };
      }
      // If server didn't send a final message, synthesise one from the buffer.
      const assistantMsg: Message =
        final ??
        {
          id: streaming.id,
          session_id: s.activeSessionId ?? "",
          role: "assistant",
          content: streaming.text,
          tool_calls: streaming.toolCalls.length
            ? streaming.toolCalls.map((t) => t.call)
            : undefined,
          created_at: new Date().toISOString(),
        };
      return {
        messages: [...s.messages, assistantMsg],
        streaming: null,
        isRunning: false,
      };
    }),

  failStream: (code, message) =>
    set((s) => ({
      streaming: null,
      isRunning: false,
      messages: [
        ...s.messages,
        {
          id: newId(),
          session_id: s.activeSessionId ?? "",
          role: "system",
          content: `[${code}] ${message}`,
          created_at: new Date().toISOString(),
        },
      ],
    })),

  setRunning: (running) => set({ isRunning: running }),

  reset: () => set({ messages: [], streaming: null, toolResults: {} }),
}));
