import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { RequestLogPanel } from "./RequestLogPanel";
import { requestLogger } from "@/lib/requestLogger";

describe("RequestLogPanel", () => {
  it("renders nothing when closed", () => {
    const { container } = render(<RequestLogPanel open={false} onClose={() => {}} />);
    expect(container.firstChild).toBeNull();
  });

  it("shows empty state when open with no logs", () => {
    requestLogger().clear();
    render(<RequestLogPanel open={true} onClose={() => {}} />);
    expect(screen.getByText("暂无请求记录")).toBeInTheDocument();
  });

  it("renders header buttons and triggers onClose", () => {
    const onClose = vi.fn();
    requestLogger().clear();
    render(<RequestLogPanel open={true} onClose={onClose} />);
    fireEvent.click(screen.getByRole("button", { name: "关闭" }));
    expect(onClose).toHaveBeenCalledOnce();
  });

  it("clears logs when 清空 clicked", () => {
    const log = requestLogger();
    log.clear();
    log.push({ method: "GET", path: "/x", status: 200 });
    render(<RequestLogPanel open={true} onClose={() => {}} />);
    expect(screen.getAllByText(/GET/).length).toBeGreaterThan(0);
    fireEvent.click(screen.getByTitle("清空"));
    expect(log.getLogs()).toHaveLength(0);
  });

  it("shows stats counters", () => {
    const log = requestLogger();
    log.clear();
    log.push({ method: "GET", path: "/a", status: 200 });
    log.push({ method: "GET", path: "/b", status: 500 });
    const { container } = render(<RequestLogPanel open={true} onClose={() => {}} />);
    // The stats div contains three cells; check that the labels render alongside
    // the numeric counters.
    expect(screen.getByText("总请求")).toBeInTheDocument();
    expect(screen.getByText("失败")).toBeInTheDocument();
    expect(screen.getByText("成功率")).toBeInTheDocument();
    // And that the container has at least one stat number rendered (数字节点)
    const digits = container.querySelectorAll("div.text-lg");
    expect(digits.length).toBe(3);
  });
});
