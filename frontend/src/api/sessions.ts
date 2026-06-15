import { apiGet, apiPost, apiPatch, apiDelete } from "./client";
import type { CreateSessionReq, Session } from "@shared/types";

export const listSessions = () => apiGet<Session[]>("/sessions");
export const getSession = (id: string) => apiGet<Session>(`/sessions/${id}`);
export const createSession = (req: CreateSessionReq) => apiPost<Session, CreateSessionReq>("/sessions", req);
export const updateSession = (id: string, title: string) =>
  apiPatch<Session, { title: string }>(`/sessions/${id}`, { title });
export const deleteSession = (id: string) => apiDelete<{ deleted: string }>(`/sessions/${id}`);
