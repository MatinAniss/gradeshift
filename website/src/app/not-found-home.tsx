"use client";

import { Button } from "@/components/ui/button";
import { useSession } from "@/hooks/session";
import Link from "next/link";

export default function NotFoundHome() {
  const { session } = useSession();

  return (
    <Link href={session.user ? "/home" : "/"}>
      <Button>Go back Home</Button>
    </Link>
  );
}
