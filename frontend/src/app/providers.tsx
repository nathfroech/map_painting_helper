"use client";

import { MantineProvider } from "@mantine/core";
import { QueryClient, type QueryClientConfig, QueryClientProvider } from "@tanstack/react-query";
import React from "react";

interface ProvidersProps {
  children: React.ReactNode;
  env?: "default" | "test" | undefined;
}

export function Providers({ children, env }: ProvidersProps) {
  const queryConfig: QueryClientConfig =
    env === "test"
      ? { defaultOptions: { queries: { retry: false }, mutations: { retry: false } } }
      : {};
  const [queryClient] = React.useState(() => new QueryClient(queryConfig));

  return (
    <QueryClientProvider client={queryClient}>
      <MantineProvider env={env}>{children}</MantineProvider>
    </QueryClientProvider>
  );
}
