import { apiGet } from "./client";
import type { Message } from "@shared/types";

export const listMessages = (sessionId: string) =>
  apiGet<Message[]>(`/sessions/${sessionId}/messages`);
