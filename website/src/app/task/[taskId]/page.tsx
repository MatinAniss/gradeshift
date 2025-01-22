interface Params {
  taskId: string;
}

export default async function Organisation({ params }: { params: Promise<Params> }) {
  const { taskId } = await params;

  return <div className="flex flex-col mx-auto max-w-screen-xl my-16 gap-6"></div>;
}
