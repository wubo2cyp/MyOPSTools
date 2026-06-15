import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { EmptyState } from "./EmptyState";

describe("EmptyState", () => {
  it("renders the welcome heading", () => {
    render(<EmptyState />);
    expect(screen.getByText(/开始与 Agent 对话/)).toBeInTheDocument();
  });
});
