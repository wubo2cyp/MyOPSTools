import { API_BASE_URL } from "./client";
import type { CreateRunReq, RunEvent } from "@shared/types";

/**
 * Open an SSE connection to start a run. Returns:
 *   - an async iterator of `RunEvent`
 *   - a `close()` function to cancel the underlying request.
 *
 * Implementation note: the browser `fetch` API supports streaming response bodies
 * (ReadableStream) starting from Chrome 105+ / Safari 16.4+. We deliberately do
 * NOT use `EventSource` because it is GET-only and does not accept a JSON body.
 */
export async function openRunStream(
  sessionId: string,
  req: CreateRunReq,
): Promise<{ events: AsyncIterableIterator<RunEvent>; close: () => void }> {
  const ctrl = new AbortController();
  const res = await fetch(`${API_BASE_URL}/sessions/${sessionId}/runs`, {
    method: "POST",
    headers: { "content-type": "application/json", accept: "text/event-stream" },
    body: JSON.stringify(req),
    signal: ctrl.signal,
  });
  if (!res.ok || !res.body) {
    ctrl.abort();
    throw new Error(`failed to open SSE: HTTP ${res.status}`);
  }
  const events = parseSse(res.body);
  return { events, close: () => ctrl.abort() };
}

async function* parseSse(body: ReadableStream<Uint8Array>): AsyncIterableIterator<RunEvent> {
  const reader = body.getReader();
  const decoder = new TextDecoder("utf-8");
  let buffer = "";
  while (true) {
    const { value, done } = await reader.read();
    if (done) break;
    buffer += decoder.decode(value, { stream: true });
    let idx: number;
    // SSE frames are separated by a blank line.
    while ((idx = buffer.indexOf("\n\n")) >= 0) {
      const frame = buffer.slice(0, idx);
      buffer = buffer.slice(idx + 2);
      const event = parseFrame(frame);
      if (event) yield event;
    }
  }
}

function parseFrame(frame: string): RunEvent | null {
  let eventName = "message";
  const dataLines: string[] = [];
  for (const raw of frame.split("\n")) {
    const line = raw.trimEnd();
    if (!line || line.startsWith(":")) continue;
    if (line.startsWith("event:")) {
      eventName = line.slice(6).trim();
    } else if (line.startsWith("data:")) {
      dataLines.push(line.slice(5).trimStart());
    }
  }
  if (dataLines.length === 0) return null;
  const data = dataLines.join("\n");
  try {
    const parsed = JSON.parse(data) as Record<string, unknown>;
    // Backend tool.call events use flat fields: { id, name, arguments }.
    // Promote to the { call: { id, name, arguments } } shape used by the
    // shared RunEvent type so consumers have a single shape to handle.
    if (eventName === "tool.call" && parsed && "id" in parsed && "name" in parsed) {
      const call = { id: String(parsed.id), name: String(parsed.name), arguments: (parsed.arguments as Record<string, unknown>) ?? {} };
      return { type: "tool.call", ...call } as RunEvent;
    }
    return { type: eventName, ...(parsed as object) } as RunEvent;
  } catch {
    // If a frame is not JSON, surface it as a generic message.delta.
    return { type: "message.delta", delta: data };
  }
}
