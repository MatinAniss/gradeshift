import OrganisationComponent from "./organisationComponent";

interface Params {
  organisationId: string;
}

export default async function Organisation({ params }: { params: Promise<Params> }) {
  const { organisationId } = await params;

  return (
    <div className="flex flex-col mx-auto max-w-screen-xl my-16 gap-6">
      <OrganisationComponent id={organisationId} />
    </div>
  );
}
