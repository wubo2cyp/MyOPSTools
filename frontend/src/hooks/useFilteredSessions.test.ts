import { describe, it, expect } from "vitest";
import { renderHook } from "@testing-library/react";
import { useFilteredSessions, type SortMode } from "@/hooks/useFilteredSessions";
import type { Session } from "@shared/types";

const SESSIONS: Session[] = [
  { id: "a", title: "B 会议", agent_id: "default", created_at: "2026-06-10T10:00:00Z", updated_at: "" },
  { id: "b", title: "A 项目", agent_id: "default", created_at: "2026-06-12T10:00:00Z", updated_at: "" },
  { id: "c", title: "C 旅行", agent_id: "default", created_at: "2026-06-11T10:00:00Z", updated_at: "" },
];

function filter(q: string, s: SortMode) {
  return renderHook(() => useFilteredSessions(SESSIONS, q, s)).result.current;
}

describe("useFilteredSessions", () => {
  it("returns all sessions when query is empty", () => {
    const result = filter("", "newest");
    expect(result).toHaveLength(3);
  });

  it("filters by title case-insensitively", () => {
    // Session "B 会议" lowercased is "b 会议"; a query of "会议" (no A) should match.
    const result = filter("会议", "newest");
    expect(result.map((s) => s.id)).toEqual(["a"]);
  });

  it("filters by English substring (case-insensitive)", () => {
    // Use a session that contains an English word to test case folding.
    const mixed: Session[] = [
      { id: "x", title: "React Project", agent_id: "default", created_at: "2026-06-10T10:00:00Z", updated_at: "" },
    ];
    const { result } = renderHook(() => useFilteredSessions(mixed, "react", "newest"));
    expect(result.current).toHaveLength(1);
    const { result: r2 } = renderHook(() => useFilteredSessions(mixed, "REACT", "newest"));
    expect(r2.current).toHaveLength(1);
  });

  it("returns no match for non-existent query", () => {
    const result = filter("zzz", "newest");
    expect(result).toHaveLength(0);
  });

  it("sorts by newest first", () => {
    const result = filter("", "newest");
    expect(result.map((s) => s.id)).toEqual(["b", "c", "a"]);
  });

  it("sorts by oldest first", () => {
    const result = filter("", "oldest");
    expect(result.map((s) => s.id)).toEqual(["a", "c", "b"]);
  });

  it("sorts A-Z by title", () => {
    const result = filter("", "az");
    expect(result.map((s) => s.title)).toEqual(["A 项目", "B 会议", "C 旅行"]);
  });

  it("sorts Z-A by title", () => {
    const result = filter("", "za");
    expect(result.map((s) => s.title)).toEqual(["C 旅行", "B 会议", "A 项目"]);
  });

  it("combines filter and sort", () => {
    // all three contain no common char; let's filter "会" (a) or "旅" (c)
    const result = filter("会", "az");
    expect(result.map((s) => s.id)).toEqual(["a"]);
  });
});
