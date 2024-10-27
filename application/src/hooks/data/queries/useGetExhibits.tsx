import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import type { Exhibit } from "@/types";

async function getExhibits() {
  const response = await axios.get<Exhibit[]>("http://localhost:3030/exhibits");
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
