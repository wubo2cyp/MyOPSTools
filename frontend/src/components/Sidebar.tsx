import { Plus, MessageSquare, Trash2, Loader2 } from "lucide-react";
import { useState } from "react";
import type { Session } from "@shared/types";
import { useChatStore } from "@/store/chatStore";
import { useToastStore } from "@/store/toastStore";
import { createSession, deleteSession } from "@/api/sessions";
import { listMessages } from "@/api/messages";
import { SessionSearchBar } from "./SessionSearchBar";
import { useFilteredSessions } from "@/hooks/useFilteredSessions";

interface Props {
  sessions: Session[];
  onRefresh: () => void;
}

export function Sidebar({ sessions, onRefresh }: Props) {
  const activeId = useChatStore((s) => s.activeSessionId);
  const setActive = useChatStore((s) => s.setActiveSession);
  const setMessages = useChatStore((s) => s.setMessages);
  const [creating, setCreating] = useState(false);
  const addToast = useToastStore((s) => s.addToast);

  const [query, setQuery] = useState("");
  const [sort, setSort] = useState<import("@/hooks/useFilteredSessions").SortMode>("newest");

  const filtered = useFilteredSessions(sessions, query, sort);

  const handleNew = async () => {
    if (creating) return;
    setCreating(true);
    try {
      const created = await createSession({});
      onRefresh();
      await selectSession(created.id);
    } catch (e) {
      const err = e as Error;
      addToast({ type: "error", message: `创建会话失败: ${err.message}` });
      console.error("failed to create session", e);
    } finally {
      setCreating(false);
    }
  };

  const selectSession = async (id: string) => {
    setActive(id);
    try {
      const msgs = await listMessages(id);
      setMessages(msgs);
    } catch (e) {
      const err = e as Error;
      addToast({ type: "error", message: `加载消息失败: ${err.message}` });
      console.error("failed to load messages", e);
      setMessages([]);
    }
  };

  const handleDelete = async (id: string, e: React.MouseEvent) => {
    e.stopPropagation();
    if (!confirm("确定删除此会话？")) return;
    try {
      await deleteSession(id);
      if (activeId === id) setActive(null);
      onRefresh();
    } catch (err) {
      const e = err as Error;
      addToast({ type: "error", message: `删除会话失败: ${e.message}` });
      console.error("failed to delete session", err);
    }
  };

  return (
    <aside className="flex w-72 shrink-0 flex-col border-r border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900">
      <div className="flex h-14 items-center justify-between border-b border-zinc-200 px-4 dark:border-zinc-800">
        <span className="text-sm font-semibold text-zinc-700 dark:text-zinc-200">会话</span>
        <button
          type="button"
          onClick={handleNew}
          disabled={creating}
          className="inline-flex h-7 w-7 items-center justify-center rounded-md text-zinc-500 transition hover:bg-zinc-100 hover:text-zinc-900 disabled:opacity-40 dark:text-zinc-400 dark:hover:bg-zinc-800 dark:hover:text-zinc-100"
          aria-label="新建会话"
        >
          {creating ? <Loader2 size={16} className="animate-spin" /> : <Plus size={16} />}
        </button>
      </div>
      <SessionSearchBar
        query={query}
        onQueryChange={setQuery}
        sort={sort}
        onSortChange={setSort}
      />
      <div className="flex-1 overflow-y-auto p-2">
        {filtered.length === 0 ? (
          <div className="flex h-full flex-col items-center justify-center gap-2 px-4 py-12 text-center text-zinc-400 dark:text-zinc-500">
            <MessageSquare size={20} />
            <p className="text-xs">
              {query ? "未找到匹配会话" : "还没有会话，点击 + 新建一个吧"}
            </p>
          </div>
        ) : (
          <ul className="space-y-1">
            {filtered.map((s) => (
              <li key={s.id}>
                <button
                  type="button"
                  onClick={() => selectSession(s.id)}
                  className={`group flex w-full items-center justify-between rounded-md px-3 py-2 text-left text-sm transition ${
                    activeId === s.id
                      ? "bg-accent/10 text-accent"
                      : "text-zinc-700 hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800"
                  }`}
                >
                  <span className="truncate">{s.title || "新会话"}</span>
                  <span
                    role="button"
                    tabIndex={0}
                    onClick={(e) => handleDelete(s.id, e)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter") handleDelete(s.id, e as unknown as React.MouseEvent);
                    }}
                    className="ml-1 hidden h-5 w-5 shrink-0 items-center justify-center rounded text-zinc-400 hover:bg-red-100 hover:text-red-500 group-hover:flex dark:text-zinc-500 dark:hover:bg-red-900/30 dark:hover:text-red-400"
                    aria-label="删除会话"
                  >
                    <Trash2 size={12} />
                  </span>
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>
      <div className="border-t border-zinc-200 px-4 py-3 text-[10px] uppercase tracking-widest text-zinc-400 dark:border-zinc-800 dark:text-zinc-500">
        v0.1.0 · SSE streaming
      </div>
    </aside>
  );
}
