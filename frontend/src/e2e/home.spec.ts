import { expect, test } from "@playwright/test";

test("has heading", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("heading", { name: /to get started/i })).toBeVisible();
});
