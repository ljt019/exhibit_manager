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

///
export default function useGetExhibitParts(partIds: string[]) {
  return useQuery<Part[]>({
    queryKey: ["parts", partIds],
    queryFn: () => getPartsByIds(partIds),
    enabled: partIds.length > 0, // Only run the query if partIds is not empty
    refetchInterval: 1000 * 60 * 1, // 1 minute
  });
}
