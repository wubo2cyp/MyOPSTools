import { useEffect, useState } from "react";
import { listSessions } from "@/api/sessions";
import type { Session } from "@shared/types";

/**
 * Subscribe to the session list. Re-fetches on focus for now (M1).
 * M2 will replace this with SWR / React Query.
 */
export function useSessions() {
  const [sessions, setSessions] = useState<Session[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    listSessions()
      .then(setSessions)
      .catch((e: Error) => setError(e.message));
  }, []);

  return { sessions, error };
}
