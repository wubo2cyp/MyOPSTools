import { useEffect, useState, useCallback } from "react";
import { requestLogger, type LogEntry } from "@/lib/requestLogger";
import { X, Activity, Trash2, Copy, Clock } from "lucide-react";

const logger = requestLogger();

interface Props {
  open: boolean;
  onClose: () => void;
}

export function RequestLogPanel({ open, onClose }: Props) {
  const [logs, setLogs] = useState<LogEntry[]>(logger.getLogs());
  const [stats, setStats] = useState(logger.stats());

  useEffect(() => {
    const unsub = logger.subscribe(() => {
      setLogs(logger.getLogs());
      setStats(logger.stats());
    });
    return unsub;
  }, []);

  const handleClear = useCallback(() => {
    logger.clear();
  }, []);

  const handleCopy = useCallback(() => {
    const json = JSON.stringify(logs, null, 2);
    navigator.clipboard.writeText(json).catch(() => {});
  }, [logs]);

  if (!open) return null;

  return (
    <div className="fixed inset-0 z-40 flex justify-end">
      <div className="absolute inset-0 bg-black/20 dark:bg-black/40" onClick={onClose} />
      <div className="relative z-10 flex h-full w-96 flex-col border-l border-zinc-200 bg-white shadow-xl dark:border-zinc-800 dark:bg-zinc-900">
        <div className="flex h-14 items-center justify-between border-b border-zinc-200 px-4 dark:border-zinc-800">
          <div className="flex items-center gap-2 text-sm font-semibold text-zinc-800 dark:text-zinc-100">
            <Activity size={16} className="text-accent" />
            请求日志
          </div>
          <div className="flex items-center gap-1">
            <button
              type="button"
              onClick={handleCopy}
              className="rounded p-1.5 text-zinc-500 hover:bg-zinc-100 dark:text-zinc-400 dark:hover:bg-zinc-800"
              title="复制 JSON"
            >
              <Copy size={14} />
            </button>
            <button
              type="button"
              onClick={handleClear}
              className="rounded p-1.5 text-zinc-500 hover:bg-zinc-100 dark:text-zinc-400 dark:hover:bg-zinc-800"
              title="清空"
            >
              <Trash2 size={14} />
            </button>
            <button
              type="button"
              onClick={onClose}
              className="rounded p-1.5 text-zinc-500 hover:bg-zinc-100 dark:text-zinc-400 dark:hover:bg-zinc-800"
              aria-label="关闭"
            >
              <X size={14} />
            </button>
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-3 border-b border-zinc-200 px-4 py-3 dark:border-zinc-800">
          <div className="text-center">
            <div className="text-lg font-semibold text-zinc-800 dark:text-zinc-100">{stats.total}</div>
            <div className="text-[10px] uppercase tracking-widest text-zinc-400 dark:text-zinc-500">总请求</div>
          </div>
          <div className="text-center">
            <div className="text-lg font-semibold text-red-600 dark:text-red-400">{stats.failed}</div>
            <div className="text-[10px] uppercase tracking-widest text-zinc-400 dark:text-zinc-500">失败</div>
          </div>
          <div className="text-center">
            <div className="text-lg font-semibold text-zinc-800 dark:text-zinc-100">
              {stats.total > 0 ? Math.round(((stats.total - stats.failed) / stats.total) * 100) : 0}%
            </div>
            <div className="text-[10px] uppercase tracking-widest text-zinc-400 dark:text-zinc-500">成功率</div>
          </div>
        </div>

        {/* Logs */}
        <div className="flex-1 overflow-y-auto p-2">
          {logs.length === 0 ? (
            <div className="flex h-full flex-col items-center justify-center gap-2 text-zinc-400 dark:text-zinc-500">
              <Clock size={20} />
              <p className="text-xs">暂无请求记录</p>
            </div>
          ) : (
            <div className="space-y-1">
              {logs.slice().reverse().map((log) => (
                <div
                  key={log.id}
                  className="rounded-md border border-zinc-100 bg-zinc-50 px-2.5 py-2 text-xs dark:border-zinc-800 dark:bg-zinc-800/50"
                >
                  <div className="flex items-center justify-between gap-2">
                    <span className="font-mono font-medium text-zinc-700 dark:text-zinc-300">
                      {log.method} {log.path}
                    </span>
                    <span
                      className={`shrink-0 rounded px-1 py-0.5 text-[10px] font-medium ${
                        log.error || (log.status && log.status >= 400)
                          ? "bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-300"
                          : log.status && log.status >= 200 && log.status < 300
                          ? "bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-300"
                          : "bg-zinc-100 text-zinc-600 dark:bg-zinc-800 dark:text-zinc-400"
                      }`}
                    >
                      {log.status ?? "—"}
                    </span>
                  </div>
                  <div className="mt-1 flex items-center gap-3 text-[10px] text-zinc-400 dark:text-zinc-500">
                    <span>{new Date(log.timestamp).toLocaleTimeString()}</span>
                    {log.duration !== undefined && <span>{log.duration}ms</span>}
                    {log.error && <span className="text-red-500 dark:text-red-400">{log.error}</span>}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
