import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import type { Exhibit } from "@/components/exhibit-card";

interface ApiExhibit {
  id: number;
  name: string;
  cluster: string;
  location: string;
  status: "operational" | "needs repair" | "out of service";
  image_url: string;
}

function transformExhibit(apiExhibit: ApiExhibit): Exhibit {
  return {
    id: apiExhibit.id.toString(), // Convert to string as expected by Exhibit type
    name: apiExhibit.name,
    cluster: apiExhibit.cluster,
    location: apiExhibit.location,
    status: apiExhibit.status,
    imageUrl: apiExhibit.image_url, // Transform snake_case to camelCase
    parts: [], // Provide default empty array since API doesn't include parts
    notes: [], // Provide default empty array since API doesn't include notes
  };
}

async function getExhibits() {
  const response = await axios.get<ApiExhibit[]>(
    "http://localhost:3030/exhibits"
  );
  return response.data.map(transformExhibit);
}

export default function useGetExhibits() {
  return useQuery<Exhibit[]>({
    queryKey: ["exhibits"],
    queryFn: getExhibits,
  });
}
