import { Sparkles } from "lucide-react";

export function EmptyState() {
  return (
    <div className="flex h-full flex-col items-center justify-center gap-3 text-center text-zinc-500 dark:text-zinc-400">
      <div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-accent/10 text-accent">
        <Sparkles size={20} />
      </div>
      <h2 className="text-base font-semibold text-zinc-700 dark:text-zinc-200">开始与 Agent 对话</h2>
      <p className="max-w-sm text-xs leading-relaxed text-zinc-500 dark:text-zinc-400">
        描述你的任务，Agent 会选择合适的工具来执行。M1 阶段仅展示界面，真实推理将在 M3 接入。
      </p>
    </div>
  );
}
