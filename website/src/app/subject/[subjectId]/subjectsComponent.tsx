"use client";

import { subject } from "@/api/rest/subjects";
import { useSession } from "@/hooks/session";
import { useQuery } from "@tanstack/react-query";
import Link from "next/link";

export default function SubjectsComponenet({ id }: { id: string }) {
  const { session } = useSession();
  const { data } = useQuery({
    queryKey: ["subjects", id],
    queryFn: () => {
      return subject(id);
    },
    enabled: session.token !== null
  });

  return (
    <>
      <h1 className="text-3xl">{data?.name}</h1>
      {data?.tasks.length === 0 ? (
        <div className="flex items-center justify-center h-32">No tasks available</div>
      ) : (
        <div className="grid grid-cols-3 gap-4">
          {data?.tasks.map((s) => (
            <Link key={s.id} href={`/task/${s.id}`} className="rounded-md shadow-md border border-border px-6 py-8">
              <div>{s.name}</div>
              <div>_</div>
            </Link>
          ))}
        </div>
      )}
    </>
  );
}
