"use client";

import { login } from "@/api/rest/auth";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useSession } from "@/hooks/session";
import { useMutation } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { useState } from "react";

export default function LoginCard() {
  const router = useRouter();
  const { setToken, setUser } = useSession();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const loginMutation = useMutation({
    mutationFn: async (email: string) => {
      const res = await login({ email, password });
      setToken({ token: res.token.token, createdAt: new Date(res.token.createdAt), expiresAt: new Date(res.token.expiresAt) });
      setUser(res.user);
      router.push("/");

      return res;
    }
  });

  return (
    <div className="flex flex-col gap-8 px-12 py-8 bg-primary-foreground rounded-md shadow-md">
      <div className="flex flex-col gap-2">
        <Input placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} />
        <Input placeholder="Password" type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
      </div>
      <Button
        disabled={email.length === 0 || password.length === 0}
        onClick={() => {
          if (email.length > 0 && password.length > 0) {
            loginMutation.mutate(email);
          }
        }}
      >
        Login
      </Button>
    </div>
  );
}
