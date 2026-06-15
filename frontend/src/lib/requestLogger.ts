interface LogEntry {
  id: string;
  timestamp: number;
  method: string;
  path: string;
  status?: number;
  duration?: number;
  error?: string;
}

const MAX = 100;
let _logs: LogEntry[] = [];
let _listeners: (() => void)[] = [];
let _counter = 0;

function notify() {
  _listeners.forEach((cb) => cb());
}

export function requestLogger() {
  return {
    push(entry: Omit<LogEntry, "id" | "timestamp">) {
      const id = `log-${Date.now()}-${_counter++}`;
      _logs.push({ ...entry, id, timestamp: Date.now() });
      if (_logs.length > MAX) _logs.shift();
      notify();
    },
    clear() {
      _logs = [];
      notify();
    },
    getLogs() {
      return [..._logs];
    },
    stats() {
      const total = _logs.length;
      const failed = _logs.filter((l) => l.error || (l.status && l.status >= 400)).length;
      const byStatus: Record<number, number> = {};
      _logs.forEach((l) => {
        if (l.status) {
          byStatus[l.status] = (byStatus[l.status] || 0) + 1;
        }
      });
      return { total, failed, byStatus };
    },
    subscribe(cb: () => void) {
      _listeners.push(cb);
      return () => {
        _listeners = _listeners.filter((l) => l !== cb);
      };
    },
  };
}

export type { LogEntry };
