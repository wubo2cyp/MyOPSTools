import { describe, it, expect, beforeEach } from "vitest";
import { requestLogger } from "@/lib/requestLogger";

describe("requestLogger", () => {
  // Each test uses a fresh logger instance bound to the singleton
  // to ensure no leakage between tests.
  let log: ReturnType<typeof requestLogger>;

  beforeEach(() => {
    log = requestLogger();
    log.clear();
  });

  it("starts empty after clear", () => {
    expect(log.getLogs()).toHaveLength(0);
  });

  it("pushes entries with id and timestamp", () => {
    log.push({ method: "GET", path: "/x" });
    const list = log.getLogs();
    expect(list).toHaveLength(1);
    expect(list[0].id).toMatch(/^log-/);
    expect(typeof list[0].timestamp).toBe("number");
  });

  it("caps history at 100 entries (FIFO)", () => {
    for (let i = 0; i < 105; i++) {
      log.push({ method: "GET", path: `/p${i}` });
    }
    expect(log.getLogs()).toHaveLength(100);
    expect(log.getLogs()[0].path).toBe("/p5");
  });

  it("computes stats for total / failed / byStatus", () => {
    log.push({ method: "GET", path: "/a", status: 200 });
    log.push({ method: "POST", path: "/b", status: 500 });
    log.push({ method: "GET", path: "/c", error: "boom" });
    const stats = log.stats();
    expect(stats.total).toBe(3);
    expect(stats.failed).toBe(2);
    expect(stats.byStatus[200]).toBe(1);
    expect(stats.byStatus[500]).toBe(1);
  });

  it("notifies subscribers when a log is pushed", () => {
    let count = 0;
    const unsub = log.subscribe(() => count++);
    log.push({ method: "GET", path: "/x" });
    log.push({ method: "GET", path: "/y" });
    expect(count).toBe(2);
    unsub();
    log.push({ method: "GET", path: "/z" });
    expect(count).toBe(2); // unsubscribed, should not change
  });
});
