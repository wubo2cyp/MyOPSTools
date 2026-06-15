import { useState, type KeyboardEvent } from "react";
import { Send, Square } from "lucide-react";

interface Props {
  disabled: boolean;
  onRunningChange: (running: boolean) => void;
}

/**
 * M1 placeholder composer. M4 will wire it to the SSE stream.
 */
export function Composer({ disabled, onRunningChange }: Props) {
  const [text, setText] = useState("");

  const send = () => {
    if (!text.trim() || disabled) return;
    onRunningChange(true);
    // M1 stub: simulate streaming so the UI is exercisable.
    setTimeout(() => onRunningChange(false), 600);
    setText("");
  };

  const onKey = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  };

  return (
    <div className="flex items-end gap-2 rounded-xl border border-zinc-200 bg-zinc-50 p-2 focus-within:border-accent/60 focus-within:ring-2 focus-within:ring-accent/20">
      <textarea
        value={text}
        onChange={(e) => setText(e.target.value)}
        onKeyDown={onKey}
        rows={1}
        placeholder="向 Agent 发送消息，Enter 发送，Shift+Enter 换行"
        className="flex-1 resize-none bg-transparent px-2 py-1.5 text-sm text-zinc-800 placeholder-zinc-400 focus:outline-none"
      />
      <button
        type="button"
        onClick={send}
        disabled={disabled}
        className="inline-flex h-8 w-8 items-center justify-center rounded-lg bg-zinc-900 text-white transition hover:bg-zinc-700 disabled:opacity-40"
        aria-label={disabled ? "运行中" : "发送"}
      >
        {disabled ? <Square size={14} /> : <Send size={14} />}
      </button>
    </div>
  );
}
