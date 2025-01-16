import { useQuery } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import type { Part } from "@/types";

/// This is the dumbest implementation for this probably ever
/// but i don't have the time or energy to care right now
/// please for the love of god come and fix this later with a real
/// backend endpoint
async function getPartById(id: string): Promise<Part> {
  const response = await axiosInstance.get<Part[]>("/parts");

  // Find the exhibit with the matching ID
  const part = response.data.find((part) => part.id === id);

  if (!part) {
    throw new Error(`Part with ID ${id} not found`);
  }

  return part;
}

interface UseGetPartOptions {
  enabled?: boolean;
}

export default function useGetPart(
  id: string,
  options: UseGetPartOptions = {}
) {
  return useQuery<Part>({
    queryKey: ["part", id],
    queryFn: () => getPartById(id),
    enabled: options.enabled,
    refetchInterval: 1000 * 60 * 1,
  });
}
