"use client";

import { Button, Code, Loader, Paper, Text } from "@mantine/core";
import { useMutation } from "@tanstack/react-query";
import { useState } from "react";

export default function Home() {
  const [result, setResult] = useState<unknown>(null);

  const { mutate, isPending, error } = useMutation({
    mutationFn: async () => {
      const response = await fetch("/api/eu4/parse-data", { method: "POST" });
      const body = await response.json();
      if (!response.ok) {
        throw new Error(body.error ?? `HTTP ${response.status}`);
      }
      return body;
    },
    onSuccess: (data) => setResult(data),
  });

  return (
    <div className="flex flex-col flex-1 items-center p-8 gap-6">
      <h1 className="text-2xl font-bold">Map Painting Helper</h1>

      <Button onClick={() => mutate()} loading={isPending} disabled={isPending}>
        Parse data
      </Button>

      {isPending && (
        <div className="flex items-center gap-2">
          <Loader size="sm" />
          <Text>Parsing data...</Text>
        </div>
      )}

      {error ? (
        <Paper p="md" withBorder className="w-full max-w-3xl">
          <Text c="red">Error: {error.message}</Text>
        </Paper>
      ) : null}

      {result ? (
        <Paper p="md" withBorder className="w-full max-w-3xl overflow-auto">
          <Code block>{JSON.stringify(result, null, 2)}</Code>
        </Paper>
      ) : null}
    </div>
  );
}
