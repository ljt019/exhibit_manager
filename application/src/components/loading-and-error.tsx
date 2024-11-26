import { Loader2 } from "lucide-react";

export function Loading() {
  return (
    <div className="h-screen flex justify-center items-center">
      <Loader2 className="animate-spin h-8 w-8" />
    </div>
  );
}

export function Error({ error, name }: { error: Error | null; name: string }) {
  return (
    <div>
      Error fetching {name} {error && <div>{error.toString()}</div>}
    </div>
  );
}
