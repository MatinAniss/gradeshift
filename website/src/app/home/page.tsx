import Organisations from "./organisations";

export default function Home() {
  return (
    <div className="flex flex-col mx-auto max-w-screen-xl my-16 gap-6">
      <h1 className="text-3xl">Home</h1>
      <div className="grid grid-cols-3 gap-4">
        <Organisations />
      </div>
    </div>
  );
}
