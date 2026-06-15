import { create } from "zustand";

export interface ToastItem {
  id: string;
  type: "error" | "success" | "info" | "warning";
  message: string;
  duration?: number;
}

interface ToastState {
  toasts: ToastItem[];
  addToast: (t: Omit<ToastItem, "id">) => void;
  removeToast: (id: string) => void;
}

let _toastCounter = 0;

export const useToastStore = create<ToastState>((set) => ({
  toasts: [],
  addToast: (item) => {
    const id = `toast-${Date.now()}-${_toastCounter++}`;
    set((s) => {
      const next = [...s.toasts, { ...item, id }];
      // Keep max 5 toasts, evict oldest
      if (next.length > 5) next.shift();
      return { toasts: next };
    });
    // Auto-dismiss after duration (default 4s)
    const duration = item.duration ?? 4000;
    if (duration > 0) {
      setTimeout(() => {
        set((s) => ({ toasts: s.toasts.filter((t) => t.id !== id) }));
      }, duration);
    }
  },
  removeToast: (id) =>
    set((s) => ({ toasts: s.toasts.filter((t) => t.id !== id) })),
}));
