import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { SessionSearchBar } from "./SessionSearchBar";

describe("SessionSearchBar", () => {
  it("renders input with current query", () => {
    render(
      <SessionSearchBar
        query="abc"
        onQueryChange={() => {}}
        sort="newest"
        onSortChange={() => {}}
      />,
    );
    const input = screen.getByPlaceholderText("搜索会话…") as HTMLInputElement;
    expect(input.value).toBe("abc");
  });

  it("emits onQueryChange when input changes", () => {
    const onQueryChange = vi.fn();
    render(
      <SessionSearchBar
        query=""
        onQueryChange={onQueryChange}
        sort="newest"
        onSortChange={() => {}}
      />,
    );
    fireEvent.change(screen.getByPlaceholderText("搜索会话…"), {
      target: { value: "hello" },
    });
    expect(onQueryChange).toHaveBeenCalledWith("hello");
  });

  it("shows clear button only when query is non-empty, and clears on click", () => {
    const onQueryChange = vi.fn();
    const { rerender } = render(
      <SessionSearchBar
        query=""
        onQueryChange={onQueryChange}
        sort="newest"
        onSortChange={() => {}}
      />,
    );
    expect(screen.queryByText("×")).toBeNull();
    rerender(
      <SessionSearchBar
        query="x"
        onQueryChange={onQueryChange}
        sort="newest"
        onSortChange={() => {}}
      />,
    );
    fireEvent.click(screen.getByText("×"));
    expect(onQueryChange).toHaveBeenCalledWith("");
  });

  it("emits onSortChange when select changes", () => {
    const onSortChange = vi.fn();
    render(
      <SessionSearchBar
        query=""
        onQueryChange={() => {}}
        sort="newest"
        onSortChange={onSortChange}
      />,
    );
    const select = screen.getByRole("combobox") as HTMLSelectElement;
    fireEvent.change(select, { target: { value: "az" } });
    expect(onSortChange).toHaveBeenCalledWith("az");
  });
});
