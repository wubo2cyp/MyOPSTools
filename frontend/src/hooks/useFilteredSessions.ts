import { useMemo } from "react";
import type { Session } from "@shared/types";

export type SortMode = "newest" | "oldest" | "az" | "za";

export function useFilteredSessions(
  sessions: Session[],
  query: string,
  sort: SortMode,
) {
  return useMemo(() => {
    let list = sessions;

    if (query.trim()) {
      const q = query.toLowerCase().trim();
      list = list.filter((s) => s.title.toLowerCase().includes(q));
    }

    list = [...list].sort((a, b) => {
      switch (sort) {
        case "newest":
          return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
        case "oldest":
          return new Date(a.created_at).getTime() - new Date(b.created_at).getTime();
        case "az":
          return a.title.localeCompare(b.title, "zh-CN");
        case "za":
          return b.title.localeCompare(a.title, "zh-CN");
      }
    });

    return list;
  }, [sessions, query, sort]);
}
