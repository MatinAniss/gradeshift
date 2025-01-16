import { Button } from "@/components/ui/button";
import Link from "next/link";

export default function NotFound() {
  return (
    <div className="flex flex-col items-center justify-center w-full h-full gap-4">
      <div className="flex flex-col items-center">
        <span className="text-6xl">404</span>
        <span className="text-2xl">Page Not Found</span>
      </div>
      <Link href="/">
        <Button>Go back Home</Button>
      </Link>
    </div>
  );
}
