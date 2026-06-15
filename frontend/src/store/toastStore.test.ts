import { describe, it, expect, beforeEach } from "vitest";
import { useToastStore } from "@/store/toastStore";

describe("toastStore", () => {
  beforeEach(() => {
    useToastStore.setState({ toasts: [] });
  });

  it("adds a toast with generated id", () => {
    const { addToast } = useToastStore.getState();
    addToast({ type: "info", message: "hello" });
    const list = useToastStore.getState().toasts;
    expect(list).toHaveLength(1);
    expect(list[0].message).toBe("hello");
    expect(list[0].type).toBe("info");
    expect(list[0].id).toMatch(/^toast-/);
  });

  it("removes a toast by id", () => {
    const { addToast, removeToast } = useToastStore.getState();
    addToast({ type: "info", message: "x" });
    const id = useToastStore.getState().toasts[0].id;
    removeToast(id);
    expect(useToastStore.getState().toasts).toHaveLength(0);
  });

  it("caps the queue at 5 toasts (FIFO)", () => {
    const { addToast } = useToastStore.getState();
    for (let i = 0; i < 8; i++) {
      addToast({ type: "info", message: `m${i}`, duration: 0 });
    }
    const list = useToastStore.getState().toasts;
    expect(list).toHaveLength(5);
    // 8 pushed with cap 5 → m0, m1, m2 evicted, list = [m3, m4, m5, m6, m7]
    expect(list[0].message).toBe("m3");
    expect(list[4].message).toBe("m7");
  });
});
