import { useQuery } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import type { Exhibit } from "@/types";

/// This is the dumbest implementation for this probably ever
/// but i don't have the time or energy to care right now
/// please for the love of god come and fix this later with a real
/// backend endpoint
async function getExhibitById(id: string): Promise<Exhibit> {
  const response = await axiosInstance.get<Exhibit[]>("/exhibits");

  // Find the exhibit with the matching ID
  const exhibit = response.data.find((exhibit) => exhibit.id === id);

  if (!exhibit) {
    throw new Error(`Exhibit with ID ${id} not found`);
  }

  return exhibit;
}

interface UseGetExhibitOptions {
  enabled?: boolean;
}

export default function useGetExhibit(
  id: string,
  options: UseGetExhibitOptions = {}
) {
  return useQuery<Exhibit>({
    queryKey: ["exhibit", id],
    queryFn: () => getExhibitById(id),
    enabled: options.enabled,
    refetchInterval: 1000 * 60 * 1,
  });
}
