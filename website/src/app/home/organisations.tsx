"use client";

import { organisations } from "@/api/rest/organisations";
import { useSession } from "@/hooks/session";
import { useQuery } from "@tanstack/react-query";
import Link from "next/link";

export default function Organisations() {
  const { session } = useSession();
  const { data } = useQuery({
    queryKey: ["organistions"],
    queryFn: organisations,
    enabled: session.token !== null
  });

  return data?.organisations.map((o) => (
    <Link key={o.id} href={`/organisation/${o.id}`} className="rounded-md shadow-md border border-border px-6 py-8">
      <div>{o.name}</div>
      <div>_</div>
    </Link>
  ));
}
