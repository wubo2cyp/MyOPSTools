import { useToastStore } from "@/store/toastStore";
import { X, AlertCircle, CheckCircle2, Info, AlertTriangle } from "lucide-react";

const ICONS = {
  error: <AlertCircle size={16} className="text-red-500" />,
  success: <CheckCircle2 size={16} className="text-emerald-500" />,
  info: <Info size={16} className="text-blue-500" />,
  warning: <AlertTriangle size={16} className="text-amber-500" />,
};

const BG: Record<string, string> = {
  error: "bg-red-50 border-red-200 text-red-800 dark:bg-red-900/20 dark:border-red-800/40 dark:text-red-200",
  success: "bg-emerald-50 border-emerald-200 text-emerald-800 dark:bg-emerald-900/20 dark:border-emerald-800/40 dark:text-emerald-200",
  info: "bg-blue-50 border-blue-200 text-blue-800 dark:bg-blue-900/20 dark:border-blue-800/40 dark:text-blue-200",
  warning: "bg-amber-50 border-amber-200 text-amber-800 dark:bg-amber-900/20 dark:border-amber-800/40 dark:text-amber-200",
};

export function ToastContainer() {
  const toasts = useToastStore((s) => s.toasts);
  const removeToast = useToastStore((s) => s.removeToast);

  if (toasts.length === 0) return null;

  return (
    <div className="pointer-events-none fixed right-4 top-4 z-50 flex flex-col gap-2">
      {toasts.map((t) => (
        <div
          key={t.id}
          className={`pointer-events-auto flex w-80 items-start gap-2 rounded-lg border px-3 py-2.5 text-sm shadow-lg transition ${BG[t.type]}`}
          role="alert"
        >
          <span className="mt-0.5 shrink-0">{ICONS[t.type]}</span>
          <span className="flex-1 leading-snug">{t.message}</span>
          <button
            type="button"
            onClick={() => removeToast(t.id)}
            className="shrink-0 rounded p-0.5 opacity-60 hover:opacity-100"
            aria-label="关闭"
          >
            <X size={14} />
          </button>
        </div>
      ))}
    </div>
  );
}
