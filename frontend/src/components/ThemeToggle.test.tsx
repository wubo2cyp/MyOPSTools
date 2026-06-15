import { describe, it, expect, beforeEach } from "vitest";
import { act, render, screen, fireEvent } from "@testing-library/react";
import { ThemeToggle } from "./ThemeToggle";
import { useThemeStore } from "@/store/themeStore";

describe("ThemeToggle", () => {
  beforeEach(() => {
    useThemeStore.setState({ theme: "light" });
    document.documentElement.classList.remove("dark");
    localStorage.removeItem("agent-theme");
  });

  it("renders Moon icon when in light mode", () => {
    render(<ThemeToggle />);
    const btn = screen.getByRole("button");
    expect(btn.getAttribute("aria-label")).toMatch(/暗色/);
  });

  it("toggles theme and updates document class", () => {
    render(<ThemeToggle />);
    const btn = screen.getByRole("button");
    act(() => {
      fireEvent.click(btn);
    });
    expect(useThemeStore.getState().theme).toBe("dark");
    expect(document.documentElement.classList.contains("dark")).toBe(true);
    expect(localStorage.getItem("agent-theme")).toBe("dark");
  });

  it("toggles back to light and removes dark class", () => {
    useThemeStore.setState({ theme: "dark" });
    document.documentElement.classList.add("dark");
    render(<ThemeToggle />);
    const btn = screen.getByRole("button");
    act(() => {
      fireEvent.click(btn);
    });
    expect(useThemeStore.getState().theme).toBe("light");
    expect(document.documentElement.classList.contains("dark")).toBe(false);
  });
});
