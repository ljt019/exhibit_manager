import { useState, useMemo } from "react";
import type { Exhibit } from "@/types";

export function useExhibitFilters(exhibits: Exhibit[]) {
  const [searchTerm, setSearchTerm] = useState("");
  const [filters, setFilters] = useState<{
    cluster: string | null;
    location: string | null;
    status: string | null;
  }>({
    cluster: null,
    location: null,
    status: null,
  });
  const [showFilters, setShowFilters] = useState(false);

  const filteredExhibits = useFilteredExhibits(exhibits, searchTerm, filters);

  const clearFilters = () => {
    setFilters({ cluster: null, location: null, status: null });
    if (showFilters) setShowFilters(false);
  };

  const isFilterApplied = Object.values(filters).some(
    (filter) => filter !== null
  );

  const uniqueValues = useMemo(() => getUniqueValues(exhibits), [exhibits]);

  const filterOptions = [
    {
      value: filters.cluster,
      onChange: (value: string | null) =>
        setFilters((prev) => ({ ...prev, cluster: value })),
      options: uniqueValues.clusters,
      placeholder: "Filter by Cluster",
    },
    {
      value: filters.location,
      onChange: (value: string | null) =>
        setFilters((prev) => ({ ...prev, location: value })),
      options: uniqueValues.locations,
      placeholder: "Filter by Location",
    },
    {
      value: filters.status,
      onChange: (value: string | null) =>
        setFilters((prev) => ({ ...prev, status: value })),
      options: uniqueValues.statuses,
      placeholder: "Filter by Status",
    },
  ];

  return {
    searchTerm,
    setSearchTerm,
    filters,
    showFilters,
    setShowFilters,
    filteredExhibits,
    clearFilters,
    isFilterApplied,
    filterOptions,
  };
}

function useFilteredExhibits(
  exhibits: Exhibit[],
  searchTerm: string,
  filters: {
    cluster: string | null;
    location: string | null;
    status: string | null;
  }
) {
  return useMemo(() => {
    if (!exhibits) return [];

    return exhibits.filter((exhibit) => {
      if (
        searchTerm &&
        !exhibit.name.toLowerCase().includes(searchTerm.toLowerCase())
      ) {
        return false;
      }
      if (filters.cluster && exhibit.cluster !== filters.cluster) {
        return false;
      }
      if (filters.location && exhibit.location !== filters.location) {
        return false;
      }
      if (filters.status && exhibit.status !== filters.status) {
        return false;
      }
      return true;
    });
  }, [exhibits, searchTerm, filters]);
}

function getUniqueValues(exhibits: Exhibit[]) {
  return {
    clusters: [...new Set(exhibits.map((e) => e.cluster))],
    locations: [...new Set(exhibits.map((e) => e.location))],
    statuses: [...new Set(exhibits.map((e) => e.status))],
  };
}
