import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import type { Exhibit } from "@/components/exhibit-card";

async function getRandomExhibit() {
  const response = await axios.get<Exhibit>(
    "http://localhost:3030/exhibits/random"
  );
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
