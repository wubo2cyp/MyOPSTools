import { useState } from "react";
import { Wrench, CheckCircle2, AlertCircle } from "lucide-react";
import type { ToolCall } from "@shared/types";

interface Props {
  call: ToolCall;
  result?: { output: string };
  error?: string;
}

export function ToolCallCard({ call, result, error }: Props) {
  const [open, setOpen] = useState(false);
  return (
    <div className="rounded-xl border border-zinc-200 bg-white text-xs">
      <button
        type="button"
        onClick={() => setOpen((v) => !v)}
        className="flex w-full items-center justify-between gap-2 px-3 py-2"
      >
        <span className="flex items-center gap-2 text-zinc-700">
          <Wrench size={12} className="text-accent" />
          <span className="font-mono">{call.name}</span>
        </span>
        <span className="text-zinc-400">
          {error ? (
            <AlertCircle size={12} className="text-red-500" />
          ) : result ? (
            <CheckCircle2 size={12} className="text-accent" />
          ) : (
            "…"
          )}
        </span>
      </button>
      {open && (
        <div className="border-t border-zinc-100 px-3 py-2 font-mono text-[11px] text-zinc-600">
          <div className="mb-1 text-[10px] uppercase tracking-widest text-zinc-400">arguments</div>
          <pre className="overflow-x-auto">{JSON.stringify(call.arguments, null, 2)}</pre>
          {result && (
            <>
              <div className="mb-1 mt-2 text-[10px] uppercase tracking-widest text-zinc-400">output</div>
              <pre className="overflow-x-auto">{result.output}</pre>
            </>
          )}
          {error && (
            <div className="mt-2 text-red-600">{error}</div>
          )}
        </div>
      )}
    </div>
  );
}
