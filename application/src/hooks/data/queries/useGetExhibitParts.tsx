import { useQuery } from "@tanstack/react-query";
import type { Part } from "@/types";
import { axiosInstance } from "@/api/axiosInstance";

export async function getPartsByIds(partIds: string[]): Promise<Part[]> {
  const response = await axiosInstance.post<Part[]>("/parts/batch", partIds, {
    headers: {
      "Content-Type": "application/json",
    },
  });
  return response.data;
}

export default function useGetExhibitParts(partIds: string[]) {
  return useQuery<Part[]>({
    queryKey: ["parts", partIds], // queryKey
    queryFn: () => getPartsByIds(partIds), // queryFn
    enabled: partIds.length > 0, // Only run the query if partIds is not empty
    staleTime: 5 * 60 * 1000, // 5 minutes
    retry: 2, // Retry failed requests up to 2 times
  });
}
