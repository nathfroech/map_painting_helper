// biome-ignore lint/style/noRestrictedImports: I need the original render for test here.
import { render as originalRender } from "@testing-library/react";
import { describe, expect, test } from "vitest";
import RootLayout from "@/app/layout";
import { screen } from "@/test_utils";

describe("RootLayout", () => {
  test("renders the parse button", () => {
    // This indirectly tests providers too
    originalRender(
      <RootLayout>
        <div>Test div</div>
      </RootLayout>,
    );

    expect(screen.getByText(/test div/i)).toBeInTheDocument();
  });
});
