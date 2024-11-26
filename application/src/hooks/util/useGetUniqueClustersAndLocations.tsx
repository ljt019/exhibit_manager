import { useState, useEffect } from "react";
import useGetExhibits from "@/hooks/data/queries/useGetExhibits";

export function useGetUniqueClustersAndLocations() {
  const { data: exhibits, isLoading, isError, error } = useGetExhibits();

  const [clusters, setClusters] = useState<string[]>([]);
  const [locations, setLocations] = useState<string[]>([]);

  useEffect(() => {
    if (exhibits) {
      const uniqueClusters = Array.from(
        new Set(exhibits.map((exhibit) => exhibit.cluster))
      );
      setClusters(uniqueClusters);

      const uniqueLocations = Array.from(
        new Set(exhibits.map((exhibit) => exhibit.location))
      );
      setLocations(uniqueLocations);
    }
  }, [exhibits]);

  return { clusters, locations, isLoading, isError, error };
}
