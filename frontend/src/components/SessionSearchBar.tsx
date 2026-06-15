import { Search, ArrowDownAZ, ArrowUpAZ, Clock } from "lucide-react";
export type SortMode = "newest" | "oldest" | "az" | "za";

interface Props {
  query: string;
  onQueryChange: (q: string) => void;
  sort: SortMode;
  onSortChange: (s: SortMode) => void;
}

const SORT_OPTIONS: { value: SortMode; label: string; icon: React.ReactNode }[] = [
  { value: "newest", label: "最新", icon: <Clock size={12} /> },
  { value: "oldest", label: "最旧", icon: <Clock size={12} /> },
  { value: "az", label: "A-Z", icon: <ArrowDownAZ size={12} /> },
  { value: "za", label: "Z-A", icon: <ArrowUpAZ size={12} /> },
];

export function SessionSearchBar({ query, onQueryChange, sort, onSortChange }: Props) {
  return (
    <div className="flex items-center gap-2 border-b border-zinc-200 px-3 py-2 dark:border-zinc-800">
      <div className="flex flex-1 items-center gap-2 rounded-md border border-zinc-200 bg-zinc-50 px-2 py-1.5 dark:border-zinc-700 dark:bg-zinc-800">
        <Search size={14} className="text-zinc-400 dark:text-zinc-500" />
        <input
          type="text"
          value={query}
          onChange={(e) => onQueryChange(e.target.value)}
          placeholder="搜索会话…"
          className="flex-1 bg-transparent text-xs text-zinc-800 placeholder-zinc-400 outline-none dark:text-zinc-100 dark:placeholder-zinc-500"
        />
        {query && (
          <button
            type="button"
            onClick={() => onQueryChange("")}
            className="rounded p-0.5 text-zinc-400 hover:text-zinc-600 dark:text-zinc-500 dark:hover:text-zinc-300"
          >
            ×
          </button>
        )}
      </div>
      <div className="relative">
        <select
          value={sort}
          onChange={(e) => onSortChange(e.target.value as SortMode)}
          className="appearance-none rounded-md border border-zinc-200 bg-white px-2 py-1.5 pr-6 text-xs text-zinc-700 outline-none dark:border-zinc-700 dark:bg-zinc-800 dark:text-zinc-300"
        >
          {SORT_OPTIONS.map((opt) => (
            <option key={opt.value} value={opt.value}>
              {opt.label}
            </option>
          ))}
        </select>
        <span className="pointer-events-none absolute right-1.5 top-1/2 -translate-y-1/2 text-zinc-400 dark:text-zinc-500">
          {SORT_OPTIONS.find((o) => o.value === sort)?.icon}
        </span>
      </div>
    </div>
  );
}
