import { describe, it, expect } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ToastContainer } from "./Toast";
import { useToastStore } from "@/store/toastStore";

describe("ToastContainer", () => {
  beforeEach(() => useToastStore.setState({ toasts: [] }));
  const setup = () => render(<ToastContainer />);

  it("renders nothing when there are no toasts", () => {
    const { container } = setup();
    expect(container.firstChild).toBeNull();
  });

  it("renders a toast and dismisses on click", () => {
    useToastStore.getState().addToast({ type: "error", message: "boom", duration: 0 });
    setup();
    expect(screen.getByText("boom")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "关闭" }));
    expect(useToastStore.getState().toasts).toHaveLength(0);
  });
});
