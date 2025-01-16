"use client";

import Link from "next/link";
import { Button } from "../ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger } from "../ui/dropdown-menu";
import { useMutation } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { logout } from "@/api/rest/auth";
import { ChevronDown, LogOut, Settings } from "lucide-react";
import { useSession } from "@/hooks/session";

export default function NavbarInfo() {
  const router = useRouter();
  const { session, logout: sessionLogout } = useSession();
  const logoutMutation = useMutation({
    mutationFn: async () => {
      const res = await logout();
      sessionLogout();
      router.push("/");

      return res;
    }
  });

  return (
    <>
      {session.user ? (
        <DropdownMenu>
          <DropdownMenuTrigger>
            <div className="flex items-center gap-2">
              <span>
                {session.user.firstName} {session.user.lastName}
              </span>
              <ChevronDown className="h-5 w-5" />
            </div>
          </DropdownMenuTrigger>
          <DropdownMenuContent>
            <Link href="/settings">
              <DropdownMenuItem className="flex items-center gap-2">
                <Settings className="h-6 w-6" />
                Settings
              </DropdownMenuItem>
            </Link>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              className="flex items-center gap-2"
              onClick={() => {
                logoutMutation.mutate();
              }}
            >
              <LogOut className="h-6 w-6" />
              Log out
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      ) : (
        <Link href="/login">
          <Button>Login</Button>
        </Link>
      )}
    </>
  );
}
