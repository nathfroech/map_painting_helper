import { expect, test } from "@playwright/test";

test("has parse button", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("button", { name: /parse data/i })).toBeVisible();
});
