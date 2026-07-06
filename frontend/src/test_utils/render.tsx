// biome-ignore lint/style/noRestrictedImports: This is the place where I define the replacement.
import { render as testingLibraryRender } from "@testing-library/react";
import type React from "react";
import { Providers } from "@/app/providers";

export function render(ui: React.ReactNode) {
  return testingLibraryRender(ui, {
    wrapper: ({ children }: { children: React.ReactNode }) => (
      <Providers env="test">{children}</Providers>
    ),
  });
}
