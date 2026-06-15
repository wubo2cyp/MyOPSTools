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

export type RunEvent =
  | { type: "run.started"; run_id: string }
  | { type: "message.delta"; delta: string }
  | { type: "message.final"; message: Message }
  | { type: "tool.call"; call: ToolCall }
  | { type: "tool.result"; call_id: string; output: string }
  | { type: "error"; code: string; message: string }
  | { type: "run.finished"; run_id: string; status: "ok" | "stopped" | "error" };

export interface ApiError {
  error: { code: string; message: string };
}
