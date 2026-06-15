import { describe, it, expect, beforeEach } from "vitest";
import { act } from "@testing-library/react";
import { useChatStore } from "@/store/chatStore";
import type { Message, ToolCall } from "@shared/types";

const USER_MSG: Message = {
  id: "u1",
  session_id: "s1",
  role: "user",
  content: "hi",
  created_at: "2026-06-15T10:00:00Z",
};

const TOOL_CALL: ToolCall = {
  id: "c1",
  name: "echo",
  arguments: { x: 1 },
};

describe("chatStore", () => {
  beforeEach(() => {
    useChatStore.setState({
      sessions: [],
      activeSessionId: null,
      messages: [],
      toolResults: {},
      streaming: null,
      isRunning: false,
      error: null,
    });
  });

  it("beginStream clears error and initialises buffer", () => {
    useChatStore.setState({ error: { code: "x", message: "y" } });
    act(() => useChatStore.getState().beginStream("r1"));
    const s = useChatStore.getState();
    expect(s.error).toBeNull();
    expect(s.isRunning).toBe(true);
    expect(s.streaming?.id).toBe("r1");
    expect(s.streaming?.text).toBe("");
    expect(s.streaming?.toolCalls).toEqual([]);
  });

  it("appendDelta accumulates streaming text", () => {
    act(() => {
      useChatStore.getState().beginStream("r");
      useChatStore.getState().appendDelta("a");
      useChatStore.getState().appendDelta("b");
    });
    expect(useChatStore.getState().streaming?.text).toBe("ab");
  });

  it("recordToolCall appends; recordToolResult fills in output", () => {
    act(() => {
      useChatStore.getState().beginStream("r");
      useChatStore.getState().recordToolCall(TOOL_CALL);
    });
    expect(useChatStore.getState().streaming?.toolCalls[0].output).toBeUndefined();
    act(() => useChatStore.getState().recordToolResult("c1", "out"));
    const tc = useChatStore.getState().streaming?.toolCalls[0];
    expect(tc?.output).toBe("out");
    expect(useChatStore.getState().toolResults.c1.output).toBe("out");
  });

  it("recordToolError attaches error", () => {
    act(() => {
      useChatStore.getState().beginStream("r");
      useChatStore.getState().recordToolCall(TOOL_CALL);
      useChatStore.getState().recordToolError("c1", "boom");
    });
    expect(useChatStore.getState().streaming?.toolCalls[0].error).toBe("boom");
  });

  it("finalizeStream appends assistant message and clears streaming", () => {
    act(() => {
      useChatStore.getState().setActiveSession("s1");
      useChatStore.getState().beginStream("r");
      useChatStore.getState().appendDelta("hello ");
      useChatStore.getState().appendDelta("world");
      useChatStore.getState().finalizeStream();
    });
    const s = useChatStore.getState();
    expect(s.isRunning).toBe(false);
    expect(s.streaming).toBeNull();
    expect(s.messages).toHaveLength(1);
    expect(s.messages[0].role).toBe("assistant");
    expect(s.messages[0].content).toBe("hello world");
  });

  it("failStream sets error and clears streaming", () => {
    act(() => {
      useChatStore.getState().beginStream("r");
      useChatStore.getState().failStream("net", "down");
    });
    const s = useChatStore.getState();
    expect(s.isRunning).toBe(false);
    expect(s.streaming).toBeNull();
    expect(s.error).toEqual({ code: "net", message: "down" });
  });

  it("setActiveSession resets messages / streaming / error", () => {
    useChatStore.setState({ messages: [USER_MSG], streaming: { id: "x", text: "t", toolCalls: [] }, error: { code: "c", message: "m" } });
    act(() => useChatStore.getState().setActiveSession("s2"));
    const s = useChatStore.getState();
    expect(s.activeSessionId).toBe("s2");
    expect(s.messages).toEqual([]);
    expect(s.streaming).toBeNull();
    expect(s.error).toBeNull();
  });
});
