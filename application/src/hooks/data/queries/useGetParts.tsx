import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import type { Part } from "@/types";

async function getParts() {
  const response = await axios.get<Part[]>("http://localhost:3030/parts");
  return response.data;
}

export default function useGetParts() {
  return useQuery<Part[]>({
    queryKey: ["parts"],
    queryFn: getParts,
  });
}
