/**
 * Wire-format types shared between the Rust backend and the TypeScript frontend.
 *
 * The backend DTOs are documented here. Rust developers must keep the field
 * names + JSON casing in sync. Frontend developers may import directly:
 *
 *   import type { Session, Message } from "@shared/types";
 */

export type Role = "system" | "user" | "assistant" | "tool";

export interface ToolDef {
  name: string;
  description: string;
  input_schema: Record<string, unknown>;
}

export interface Session {
  id: string;
  title: string;
  agent_id: string;
  created_at: string;
  updated_at: string;
}

export interface ToolCall {
  id: string;
  name: string;
  arguments: Record<string, unknown>;
}

export interface Message {
  id: string;
  session_id: string;
  role: Role;
  content: string;
  tool_calls?: ToolCall[];
  tool_call_id?: string;
  created_at: string;
}

export interface CreateSessionReq {
  title?: string;
  agent_id?: string;
}

export interface CreateRunReq {
  user_message: string;
}

/**
 * Events emitted by the backend SSE stream. The Rust runtime sends
 * snake_case JSON; the SSE parser on the frontend promotes the SSE `event:`
 * line to `type`. The shape mirrors the runtime's `RunEvent` enum.
 */
export type RunEvent =
  | { type: "run.started"; run_id: string }
  | { type: "message.delta"; delta: string }
  | { type: "tool.call"; id: string; name: string; arguments: Record<string, unknown> }
  | { type: "tool.result"; call_id: string; output: string }
  | { type: "tool.error"; call_id: string; message: string }
  | { type: "message.final"; message: Message }
  | { type: "run.finished"; run_id: string; status: string }
  | { type: "run.error"; code: string; message: string }
  | { type: "error"; code: string; message: string };

export interface ApiError {
  error: { code: string; message: string };
}
