import { Plus, MessageSquare } from "lucide-react";
import type { Session } from "@shared/types";

interface Props {
  sessions: Session[];
}

export function Sidebar({ sessions }: Props) {
  return (
    <aside className="flex w-72 shrink-0 flex-col border-r border-zinc-200 bg-white">
      <div className="flex h-14 items-center justify-between border-b border-zinc-200 px-4">
        <span className="text-sm font-semibold text-zinc-700">会话</span>
        <button
          type="button"
          className="inline-flex h-7 w-7 items-center justify-center rounded-md text-zinc-500 transition hover:bg-zinc-100 hover:text-zinc-900"
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
                  className="flex w-full items-center justify-between rounded-md px-3 py-2 text-left text-sm text-zinc-700 transition hover:bg-zinc-100"
                >
                  <span className="truncate">{s.title}</span>
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>
      <div className="border-t border-zinc-200 px-4 py-3 text-[10px] uppercase tracking-widest text-zinc-400">
        M1 skeleton
      </div>
    </aside>
  );
}
