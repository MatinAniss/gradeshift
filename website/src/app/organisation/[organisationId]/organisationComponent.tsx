"use client";

import { organisation } from "@/api/rest/organisations";
import { useSession } from "@/hooks/session";
import { useQuery } from "@tanstack/react-query";
import Link from "next/link";

export default function OrganisationComponenet({ id }: { id: string }) {
  const { session } = useSession();
  const { data } = useQuery({
    queryKey: ["organistion", id],
    queryFn: () => {
      return organisation(id);
    },
    enabled: session.token !== null
  });

  return (
    <>
      <h1 className="text-3xl">{data?.name}</h1>
      <div className="grid grid-cols-3 gap-4">
        {data?.subjects.map((s) => (
          <Link key={s.id} href={`/subject/${s.id}`} className="rounded-md shadow-md border border-border px-6 py-8">
            <div>{s.name}</div>
            <div>_</div>
          </Link>
        ))}
      </div>
    </>
  );
}
