export function Loading() {
  return <div>Loading...</div>;
}

export function Error({ error, name }: { error: Error | null; name: string }) {
  return (
    <div>
      Error fetching {name} {error && <div>{error.toString()}</div>}
    </div>
  );
}
