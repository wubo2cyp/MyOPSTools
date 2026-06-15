import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ErrorBanner } from "./ErrorBanner";

describe("ErrorBanner", () => {
  it("renders code and message", () => {
    render(<ErrorBanner code="404" message="not found" />);
    expect(screen.getByText("404")).toBeInTheDocument();
    expect(screen.getByText("not found")).toBeInTheDocument();
  });

  it("triggers onClose when X clicked", () => {
    const onClose = vi.fn();
    render(<ErrorBanner code="e" message="m" onClose={onClose} />);
    fireEvent.click(screen.getByRole("button", { name: "关闭" }));
    expect(onClose).toHaveBeenCalledOnce();
  });

  it("triggers onRetry when 重试 clicked", () => {
    const onRetry = vi.fn();
    render(<ErrorBanner code="e" message="m" onRetry={onRetry} />);
    fireEvent.click(screen.getByText("重试"));
    expect(onRetry).toHaveBeenCalledOnce();
  });

  it("hides action buttons when handlers not provided", () => {
    render(<ErrorBanner code="e" message="m" />);
    expect(screen.queryByRole("button", { name: "关闭" })).toBeNull();
    expect(screen.queryByText("重试")).toBeNull();
  });
});
