import { Plus, MessageSquare, Trash2 } from "lucide-react";
import { useState } from "react";
import type { Session } from "@shared/types";
import { useChatStore } from "@/store/chatStore";
import { createSession, deleteSession } from "@/api/sessions";
import { listMessages } from "@/api/messages";

interface Props {
  sessions: Session[];
  onRefresh: () => void;
}

export function Sidebar({ sessions, onRefresh }: Props) {
  const activeId = useChatStore((s) => s.activeSessionId);
  const setActive = useChatStore((s) => s.setActiveSession);
  const setMessages = useChatStore((s) => s.setMessages);
  const [creating, setCreating] = useState(false);

  const handleNew = async () => {
    if (creating) return;
    setCreating(true);
    try {
      const created = await createSession({});
      onRefresh();
      await selectSession(created.id);
    } catch (e) {
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
      console.error("failed to delete session", err);
    }
  };

  return (
    <aside className="flex w-72 shrink-0 flex-col border-r border-zinc-200 bg-white">
      <div className="flex h-14 items-center justify-between border-b border-zinc-200 px-4">
        <span className="text-sm font-semibold text-zinc-700">会话</span>
        <button
          type="button"
          onClick={handleNew}
          disabled={creating}
          className="inline-flex h-7 w-7 items-center justify-center rounded-md text-zinc-500 transition hover:bg-zinc-100 hover:text-zinc-900 disabled:opacity-40"
          aria-label="新建会话"
        >
          <Plus size={16} />
        </button>
      </div>
      <div className="flex-1 overflow-y-auto p-2">
        {sessions.length === 0 ? (
          <div className="flex h-full flex-col items-center justify-center gap-2 px-4 py-12 text-center text-zinc-400">
            <MessageSquare size={20} />
            <p className="text-xs">还没有会话，点击 + 新建一个吧</p>
          </div>
        ) : (
          <ul className="space-y-1">
            {sessions.map((s) => (
              <li key={s.id}>
                <button
                  type="button"
                  onClick={() => selectSession(s.id)}
                  className={`group flex w-full items-center justify-between rounded-md px-3 py-2 text-left text-sm transition ${
                    activeId === s.id
                      ? "bg-accent/10 text-accent"
                      : "text-zinc-700 hover:bg-zinc-100"
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
                    className="ml-1 hidden h-5 w-5 shrink-0 items-center justify-center rounded text-zinc-400 hover:bg-red-100 hover:text-red-500 group-hover:flex"
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
      <div className="border-t border-zinc-200 px-4 py-3 text-[10px] uppercase tracking-widest text-zinc-400">
        v0.1.0 · SSE streaming
      </div>
    </aside>
  );
}
