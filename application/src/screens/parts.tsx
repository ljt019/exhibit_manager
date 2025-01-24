import { useState, useMemo } from "react";
import useGetParts from "@/hooks/data/queries/parts/useGetParts";
import useGetExhibits from "@/hooks/data/queries/exhibits/useGetExhibits";
import { FilterSection } from "@/components/filter-section";
import { CreateDialog } from "@/components/generic/create-dialog";
import { Error, Loading } from "@/components/loading-and-error";
import { PartTable } from "@/components/tables/part-table";
import { FilterOption } from "@/components/filter-section";

export default function PartsInventory() {
  const {
    data: parts,
    isLoading: isLoadingParts,
    isError: isErrorParts,
    error: errorParts,
  } = useGetParts();
  const {
    data: exhibits,
    isLoading: isLoadingExhibits,
    isError: isErrorExhibits,
    error: errorExhibits,
  } = useGetExhibits();

  const [searchTerm, setSearchTerm] = useState("");
  const [exhibitFilter, setExhibitFilter] = useState<string | null>(null);
  const [connectedExhibitFilter, setConnectedExhibitFilter] = useState<
    string | null
  >(null);
  const [showFilters, setShowFilters] = useState<boolean>(false);

  const filteredParts = useMemo(() => {
    return (
      parts?.filter((part) => {
        if (
          searchTerm &&
          !part.name.toLowerCase().includes(searchTerm.toLowerCase())
        ) {
          return false;
        }
        if (exhibitFilter && !part.exhibit_ids.includes(exhibitFilter)) {
          return false;
        }
        if (
          connectedExhibitFilter &&
          !part.exhibit_ids.includes(connectedExhibitFilter)
        ) {
          return false;
        }
        return true;
      }) || []
    );
  }, [parts, searchTerm, exhibitFilter, connectedExhibitFilter]);

  const sortedParts = useMemo(() => {
    return [...filteredParts].sort((a, b) => a.name.localeCompare(b.name));
  }, [filteredParts]);

  const clearFilters = () => {
    setExhibitFilter(null);
    setConnectedExhibitFilter(null);
    setSearchTerm("");
  };

  const isFilterApplied =
    exhibitFilter !== null ||
    connectedExhibitFilter !== null ||
    searchTerm !== "";

  const uniqueExhibits = useMemo(() => {
    if (!parts || !exhibits) return [];
    const exhibitIds = [...new Set(parts.flatMap((p) => p.exhibit_ids))];
    return exhibitIds.map((id) => {
      const exhibit = exhibits.find((e) => e.id === id);
      return { id, name: exhibit ? exhibit.name : `Unknown (${id})` };
    });
  }, [parts, exhibits]);

  const filterOptions: FilterOption[] = [
    {
      value: exhibitFilter,
      onChange: setExhibitFilter,
      options: uniqueExhibits.map((e) => e.id),
      placeholder: "Filter by Exhibit",
      labelFunction: (value) =>
        uniqueExhibits.find((e) => e.id === value)?.name || value,
    },
  ];

  if (isLoadingParts || isLoadingExhibits) {
    return <Loading />;
  }

  if (isErrorParts || isErrorExhibits || !parts || !exhibits) {
    return <Error name="parts or exhibits" />;
  }

  return (
    <div className="container mx-auto p-4">
      <div className="flex justify-between w-full mb-6">
        <h1 className="text-2xl font-bold mt-[0.1rem]">Part Inventory</h1>
        <CreateDialog type="part" />
      </div>
      <FilterSection
        showFilters={showFilters}
        setShowFilters={setShowFilters}
        clearFilters={clearFilters}
        isFilterApplied={isFilterApplied}
        setSearchTerm={setSearchTerm}
        filterOptions={filterOptions}
        searchBarName="parts"
      />
      <PartTable parts={sortedParts} />
      <div className="mt-4 text-sm text-muted-foreground">
        Showing {sortedParts.length} of {parts.length} parts
      </div>
    </div>
  );
}
