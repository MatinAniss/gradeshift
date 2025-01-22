import SubjectsComponent from "./subjectsComponent";

interface Params {
  subjectId: string;
}

export default async function Subjects({ params }: { params: Promise<Params> }) {
  const { subjectId } = await params;

  return (
    <div className="flex flex-col mx-auto max-w-screen-xl my-16 gap-6">
      <SubjectsComponent id={subjectId} />
    </div>
  );
}
