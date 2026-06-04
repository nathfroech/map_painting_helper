import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import Page from "./page";

describe("Home", () => {
  it("renders the heading", () => {
    render(<Page />);
    expect(screen.getByRole("heading", { name: /to get started/i })).toBeDefined();
  });
});
