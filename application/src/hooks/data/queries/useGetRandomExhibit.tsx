import { useQuery } from "@tanstack/react-query";
import type { Exhibit } from "@/types";
import { axiosInstance } from "@/api/axiosInstance";

async function getRandomExhibit() {
  const response = await axiosInstance.get<Exhibit>("/exhibits/random");
  return response.data;
}

// Refetch every 10 seconds
export default function useGetRandomExhibit() {
  return useQuery<Exhibit>({
    queryKey: ["exhibits-random"],
    queryFn: getRandomExhibit,
    refetchInterval: 10000,
  });
}
