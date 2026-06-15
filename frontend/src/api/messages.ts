import { apiGet, apiPost } from "./client";
import type { Message } from "@shared/types";

export const listMessages = (sessionId: string) =>
  apiGet<Message[]>(`/sessions/${sessionId}/messages`);

export const createMessage = (sessionId: string, role: string, content: string) =>
  apiPost<Message, { role: string; content: string }>(`/sessions/${sessionId}/messages`, { role, content });
