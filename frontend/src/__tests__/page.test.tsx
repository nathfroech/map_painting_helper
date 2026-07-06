import { delay, HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import { afterAll, afterEach, beforeAll, describe, expect, test, vi } from "vitest";
import Page from "@/app/page";
import { render, screen, userEvent, waitFor } from "@/test_utils";

const restHandlers = [
  http.post("/api/eu4/parse-data", async () => {
    await delay(50);
    return HttpResponse.json({ test: "data" });
  }),
];

const server = setupServer(...restHandlers);

describe("Home", () => {
  beforeAll(() => {
    server.listen({ onUnhandledRequest: "error" });
  });

  afterEach(() => {
    vi.clearAllMocks();
    server.resetHandlers();
  });

  afterAll(() => {
    server.close();
  });

  test("renders the page and parses data", async () => {
    render(<Page />);

    const parseButton = screen.getByRole("button", { name: /parse data/i });
    expect(parseButton).toBeDefined();
    expect(screen.queryByTestId("parsed-data")).toBeNull();

    await userEvent.click(parseButton);

    await waitFor(() => {
      expect(screen.getByText("Parsing data...")).toBeDefined();
    });
    await waitFor(() => {
      expect(screen.getByText(/"test": "data"/)).toBeDefined();
    });
  });

  test("shows error on failed request", async () => {
    server.use(
      http.post(
        "/api/eu4/parse-data",
        async () => {
          await delay(50);
          return HttpResponse.json({ error: "Parser module not available" }, { status: 503 });
        },
        { once: true },
      ),
    );
    render(<Page />);
    const parseButton = screen.getByRole("button", { name: /parse data/i });

    await userEvent.click(parseButton);

    await waitFor(() => {
      expect(screen.getByText("Parsing data...")).toBeDefined();
    });
    await waitFor(() => {
      expect(screen.getByText("Error: Parser module not available")).toBeDefined();
    });
  });

  test("shows generic error on failed request without error message", async () => {
    server.use(
      http.post(
        "/api/eu4/parse-data",
        async () => {
          await delay(50);
          return HttpResponse.json({}, { status: 500 });
        },
        { once: true },
      ),
    );
    render(<Page />);
    const parseButton = screen.getByRole("button", { name: /parse data/i });

    await userEvent.click(parseButton);

    await waitFor(() => {
      expect(screen.getByText("Parsing data...")).toBeDefined();
    });
    await waitFor(() => {
      expect(screen.getByText("Error: HTTP 500")).toBeDefined();
    });
  });
});
