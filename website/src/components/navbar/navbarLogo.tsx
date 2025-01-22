"use client";

import { useSession } from "@/hooks/session";
import Link from "next/link";

export default function NavbarLogo() {
  const { session } = useSession();

  return (
    <Link href={session.user ? "/home" : "/"}>
      <span>GradeShift</span>
    </Link>
  );
}
