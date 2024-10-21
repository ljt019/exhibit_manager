import { useGetUserProfile } from "@/hooks/useGetProfileInfo";

export default function Dashboard() {
  let { data: userProfile, isLoading, isError } = useGetUserProfile();

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (isError || !userProfile) {
    return <div>Error</div>;
  }

  return (
    <div className="flex h-screen justify-center items-center">
      <h1>Dashboard</h1>
    </div>
  );
}
