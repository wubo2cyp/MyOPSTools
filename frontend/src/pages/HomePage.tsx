import { useCallback, useEffect, useState } from "react";
import { Sidebar } from "@/components/Sidebar";
import { ChatPanel } from "@/components/ChatPanel";
import { ThemeToggle } from "@/components/ThemeToggle";
import { RequestLogPanel } from "@/components/RequestLogPanel";
import { useChatStore } from "@/store/chatStore";
import { useToastStore } from "@/store/toastStore";
import { listSessions } from "@/api/sessions";
import { Activity } from "lucide-react";

export function HomePage() {
  const [health, setHealth] = useState<string>("checking…");
  const [healthOk, setHealthOk] = useState(true);
  const [logOpen, setLogOpen] = useState(false);
  const sessions = useChatStore((s) => s.sessions);
  const setSessions = useChatStore((s) => s.setSessions);
  const addToast = useToastStore((s) => s.addToast);

  const refreshSessions = useCallback(() => {
    listSessions()
      .then(setSessions)
      .catch((e) => {
        const err = e as Error;
        addToast({ type: "warning", message: `刷新会话列表失败: ${err.message}` });
      });
  }, [setSessions, addToast]);

  useEffect(() => {
    let cancelled = false;
    let timer: ReturnType<typeof setInterval> | null = null;

    const checkHealth = async () => {
      try {
        const res = await fetch("/api/health");
        const data = await res.json();
        if (!cancelled) {
          setHealth(JSON.stringify(data));
          setHealthOk(true);
        }
      } catch (e) {
        if (!cancelled) {
          setHealth(`error: ${(e as Error).message}`);
          setHealthOk(false);
        }
      }
    };

    checkHealth();
    refreshSessions();

    timer = setInterval(checkHealth, 10000);

    return () => {
      cancelled = true;
      if (timer) clearInterval(timer);
    };
  }, [refreshSessions]);

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-zinc-50 dark:bg-zinc-950">
      <Sidebar sessions={sessions} onRefresh={refreshSessions} />
      <main className="flex flex-1 flex-col">
        <header className="flex h-14 items-center justify-between border-b border-zinc-200 bg-white px-6 dark:border-zinc-800 dark:bg-zinc-900">
          <div className="flex items-center gap-2">
            <div className={`h-2 w-2 rounded-full ${healthOk ? "bg-accent" : "bg-red-500"}`} />
            <h1 className="text-base font-semibold tracking-tight text-zinc-900 dark:text-zinc-100">Agent Platform</h1>
            <span className="ml-2 text-xs text-zinc-500 dark:text-zinc-400">轻量级通用 Agent · v0.1.0</span>
          </div>
          <div className="flex items-center gap-3">
            <span className="font-mono text-xs text-zinc-400 dark:text-zinc-500">/api/health → {health}</span>
            <button
              type="button"
              onClick={() => setLogOpen((v) => !v)}
              className="inline-flex h-8 w-8 items-center justify-center rounded-md text-zinc-500 transition hover:bg-zinc-100 hover:text-zinc-900 dark:text-zinc-400 dark:hover:bg-zinc-800 dark:hover:text-zinc-100"
              title="请求日志"
            >
              <Activity size={16} />
            </button>
            <ThemeToggle />
          </div>
        </header>
        <ChatPanel />
      </main>
      <RequestLogPanel open={logOpen} onClose={() => setLogOpen(false)} />
    </div>
  );
}
