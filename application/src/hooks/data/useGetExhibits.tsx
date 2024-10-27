import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import type { Exhibit } from "@/components/exhibit-card";

async function getExhibits() {
  const response = await axios.get<Exhibit[]>("http://localhost:3030/exhibits");
  return response.data;
}

export default function useGetExhibits() {
  return useQuery<Exhibit[]>({
    queryKey: ["exhibits"],
    queryFn: getExhibits,
  });
}
