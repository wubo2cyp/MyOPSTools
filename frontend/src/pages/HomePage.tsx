import { useEffect, useState } from "react";
import { Sidebar } from "@/components/Sidebar";
import { ChatPanel } from "@/components/ChatPanel";
import { useChatStore } from "@/store/chatStore";
import { listSessions } from "@/api/sessions";

export function HomePage() {
  const [health, setHealth] = useState<string>("checking…");
  const sessions = useChatStore((s) => s.sessions);
  const setSessions = useChatStore((s) => s.setSessions);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const res = await fetch("/api/health");
        const data = await res.json();
        if (!cancelled) setHealth(JSON.stringify(data));
      } catch (e) {
        if (!cancelled) setHealth(`error: ${(e as Error).message}`);
      }
    })();
    listSessions().then(setSessions).catch(() => {
      /* M1: empty list is fine */
    });
    return () => {
      cancelled = true;
    };
  }, [setSessions]);

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-zinc-50">
      <Sidebar sessions={sessions} />
      <main className="flex flex-1 flex-col">
        <header className="flex h-14 items-center justify-between border-b border-zinc-200 bg-white px-6">
          <div className="flex items-center gap-2">
            <div className="h-2 w-2 rounded-full bg-accent" />
            <h1 className="text-base font-semibold tracking-tight">Agent Platform</h1>
            <span className="ml-2 text-xs text-zinc-500">轻量级通用 Agent · v0.1.0</span>
          </div>
          <span className="font-mono text-xs text-zinc-400">/api/health → {health}</span>
        </header>
        <ChatPanel />
      </main>
    </div>
  );
}
