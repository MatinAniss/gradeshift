"use client";

import { AuthProvider } from "@/hooks/session";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { type ReactElement, useState } from "react";

export default function Providers({ children }: { children: ReactElement }) {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <AuthProvider>
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    </AuthProvider>
  );
}
