import { X, AlertTriangle, RotateCcw } from "lucide-react";

interface Props {
  code: string;
  message: string;
  onRetry?: () => void;
  onClose?: () => void;
}

export function ErrorBanner({ code, message, onRetry, onClose }: Props) {
  return (
    <div className="mx-auto mb-4 flex max-w-3xl items-start gap-2 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-800 shadow-sm dark:border-red-900/40 dark:bg-red-900/20 dark:text-red-200">
      <AlertTriangle size={16} className="mt-0.5 shrink-0 text-red-500 dark:text-red-400" />
      <div className="flex-1">
        <div className="font-semibold">{code}</div>
        <div className="mt-0.5 opacity-90">{message}</div>
      </div>
      <div className="flex shrink-0 items-center gap-1">
        {onRetry && (
          <button
            type="button"
            onClick={onRetry}
            className="inline-flex items-center gap-1 rounded-md px-2 py-1 text-xs font-medium text-red-700 transition hover:bg-red-100 dark:text-red-300 dark:hover:bg-red-900/40"
          >
            <RotateCcw size={12} />
            重试
          </button>
        )}
        {onClose && (
          <button
            type="button"
            onClick={onClose}
            className="rounded p-1 text-red-600 opacity-60 hover:opacity-100 dark:text-red-400"
            aria-label="关闭"
          >
            <X size={14} />
          </button>
        )}
      </div>
    </div>
  );
}
