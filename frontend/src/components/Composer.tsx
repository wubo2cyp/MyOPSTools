import { useState, type KeyboardEvent } from "react";
import { Send, Square } from "lucide-react";

interface Props {
  disabled: boolean;
  isRunning: boolean;
  onSend: (text: string) => void;
  onStop: () => void;
}

export function Composer({ disabled, isRunning, onSend, onStop }: Props) {
  const [text, setText] = useState("");

  const send = () => {
    const trimmed = text.trim();
    if (!trimmed || isRunning) return;
    onSend(trimmed);
    setText("");
  };

  const stop = () => {
    onStop();
  };

  const onKey = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  };

  return (
    <div className="flex items-end gap-2 rounded-xl border border-zinc-200 bg-zinc-50 p-2 focus-within:border-accent/60 focus-within:ring-2 focus-within:ring-accent/20 dark:border-zinc-700 dark:bg-zinc-800">
      <textarea
        value={text}
        onChange={(e) => setText(e.target.value)}
        onKeyDown={onKey}
        rows={1}
        placeholder={disabled ? "请先选择一个会话…" : "向 Agent 发送消息，Enter 发送，Shift+Enter 换行"}
        className="flex-1 resize-none bg-transparent px-2 py-1.5 text-sm text-zinc-800 placeholder-zinc-400 focus:outline-none dark:text-zinc-100 dark:placeholder-zinc-500"
        disabled={disabled}
      />
      {isRunning ? (
        <button
          type="button"
          onClick={stop}
          className="inline-flex h-8 items-center gap-1.5 rounded-lg bg-red-500 px-3 text-xs font-medium text-white transition hover:bg-red-600"
          aria-label="停止"
        >
          <Square size={12} fill="currentColor" />
          停止
        </button>
      ) : (
        <button
          type="button"
          onClick={send}
          disabled={disabled || !text.trim()}
          className="inline-flex h-8 w-8 items-center justify-center rounded-lg bg-zinc-900 text-white transition hover:bg-zinc-700 disabled:opacity-40"
          aria-label="发送"
        >
          <Send size={14} />
        </button>
      )}
    </div>
  );
}
