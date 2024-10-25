export type Part = {
  id: string;
  name: string;
  link: string;
  exhibit_ids: Array<string>;
  notes: Array<{ timestamp: string; text: string }>;
};

export default function Parts() {
  return (
    <div className="flex h-screen justify-center items-center">
      <h1>Parts</h1>
    </div>
  );
}
