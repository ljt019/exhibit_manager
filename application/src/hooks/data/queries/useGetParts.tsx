import { useQuery } from "@tanstack/react-query";
import type { Part } from "@/types";
import { axiosInstance } from "@/api/axiosInstance";

async function getParts() {
  const response = await axiosInstance.get<Part[]>("/parts");
  return response.data;
}

export default function useGetParts() {
  return useQuery<Part[]>({
    queryKey: ["parts"],
    queryFn: getParts,
  });
}
