import type { ApiError } from "@shared/types";
import { requestLogger } from "@/lib/requestLogger";

const logger = requestLogger();
const BASE_URL = (import.meta.env.VITE_API_BASE_URL as string | undefined) ?? "/api";

export class ApiClientError extends Error {
  public readonly code: string;
  public readonly status: number;
  constructor(status: number, code: string, message: string) {
    super(message);
    this.status = status;
    this.code = code;
  }
}

async function parseError(res: Response): Promise<ApiClientError> {
  let code = "http_error";
  let message = `HTTP ${res.status}`;
  try {
    const data = (await res.json()) as ApiError;
    if (data?.error?.code) code = data.error.code;
    if (data?.error?.message) message = data.error.message;
  } catch {
    /* swallow */
  }
  return new ApiClientError(res.status, code, message);
}

export async function apiGet<T>(path: string): Promise<T> {
  const start = performance.now();
  try {
    const res = await fetch(`${BASE_URL}${path}`);
    const duration = Math.round(performance.now() - start);
    logger.push({ method: "GET", path, status: res.status, duration });
    if (!res.ok) throw await parseError(res);
    return (await res.json()) as T;
  } catch (e) {
    const duration = Math.round(performance.now() - start);
    const err = e instanceof Error ? e.message : String(e);
    logger.push({ method: "GET", path, error: err, duration });
    throw e;
  }
}

export async function apiPost<T, B = unknown>(path: string, body?: B): Promise<T> {
  const start = performance.now();
  try {
    const res = await fetch(`${BASE_URL}${path}`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: body === undefined ? undefined : JSON.stringify(body),
    });
    const duration = Math.round(performance.now() - start);
    logger.push({ method: "POST", path, status: res.status, duration });
    if (!res.ok) throw await parseError(res);
    return (await res.json()) as T;
  } catch (e) {
    const duration = Math.round(performance.now() - start);
    const err = e instanceof Error ? e.message : String(e);
    logger.push({ method: "POST", path, error: err, duration });
    throw e;
  }
}

export async function apiPut<T, B = unknown>(path: string, body: B): Promise<T> {
  const start = performance.now();
  try {
    const res = await fetch(`${BASE_URL}${path}`, {
      method: "PUT",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(body),
    });
    const duration = Math.round(performance.now() - start);
    logger.push({ method: "PUT", path, status: res.status, duration });
    if (!res.ok) throw await parseError(res);
    return (await res.json()) as T;
  } catch (e) {
    const duration = Math.round(performance.now() - start);
    const err = e instanceof Error ? e.message : String(e);
    logger.push({ method: "PUT", path, error: err, duration });
    throw e;
  }
}

export async function apiDelete<T>(path: string): Promise<T> {
  const start = performance.now();
  try {
    const res = await fetch(`${BASE_URL}${path}`, { method: "DELETE" });
    const duration = Math.round(performance.now() - start);
    logger.push({ method: "DELETE", path, status: res.status, duration });
    if (!res.ok) throw await parseError(res);
    return (await res.json()) as T;
  } catch (e) {
    const duration = Math.round(performance.now() - start);
    const err = e instanceof Error ? e.message : String(e);
    logger.push({ method: "DELETE", path, error: err, duration });
    throw e;
  }
}

export const API_BASE_URL = BASE_URL;
