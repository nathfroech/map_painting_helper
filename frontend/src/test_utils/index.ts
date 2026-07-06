import userEvent from "@testing-library/user-event";

// biome-ignore lint/style/noRestrictedImports: This is the place where I define the replacement.
export * from "@testing-library/react";
export { render } from "./render";
export { userEvent };
