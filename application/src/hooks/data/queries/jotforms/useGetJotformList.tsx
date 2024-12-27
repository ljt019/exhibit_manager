import { useQuery } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { Jotform } from "@/types";

async function getJotforms() {
  const response = await axiosInstance.get<Jotform[]>("/jotforms");
  return response.data;
}

export default function useGetJotformList() {
  return useQuery<Jotform[]>({
    queryKey: ["jotforms"],
    queryFn: getJotforms,
    refetchInterval: 1000 * 60 * 1, // 1 minute
  });
}
