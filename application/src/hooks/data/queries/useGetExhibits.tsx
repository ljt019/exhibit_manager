import { useQuery } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import type { Exhibit } from "@/types";

async function getExhibits() {
  const response = await axiosInstance.get<Exhibit[]>("/exhibits");
  return response.data;
}

export default function useGetExhibits() {
  return useQuery<Exhibit[]>({
    queryKey: ["exhibits"],
    queryFn: getExhibits,
    staleTime: 5 * 60 * 1000, // 5 minutes
    retry: 2,
  });
}
