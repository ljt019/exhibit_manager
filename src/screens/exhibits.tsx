import { useState, useMemo, useRef } from "react";
import { Button } from "@/components/ui/button";
import { Search, X } from "lucide-react";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useVirtualizer } from "@tanstack/react-virtual";
import useGetExhibits from "@/hooks/useGetExhibits";
import { ExhibitCard } from "@/components/exhibit-card";

export default function ExhibitInventory() {
  const { data: exhibits, isLoading, isError, error } = useGetExhibits();

  const [searchTerm, setSearchTerm] = useState("");
  const [clusterFilter, setClusterFilter] = useState<string | null>(null);
  const [locationFilter, setLocationFilter] = useState<string | null>(null);
  const [statusFilter, setStatusFilter] = useState<string | null>(null);

  const filteredExhibits = useMemo(() => {
    if (!exhibits) return [];

    return exhibits.filter((exhibit) => {
      const nameMatch = exhibit.name
        .toLowerCase()
        .includes(searchTerm.toLowerCase());
      const clusterMatch = !clusterFilter || exhibit.cluster === clusterFilter;
      const locationMatch =
        !locationFilter || exhibit.location === locationFilter;
      const statusMatch = !statusFilter || exhibit.status === statusFilter;
      return nameMatch && clusterMatch && locationMatch && statusMatch;
    });
  }, [exhibits, searchTerm, clusterFilter, locationFilter, statusFilter]);

  const parentRef = useRef<HTMLDivElement>(null);

  const rowVirtualizer = useVirtualizer({
    count: Math.ceil(filteredExhibits.length / 3),
    getScrollElement: () => parentRef.current,
    estimateSize: () => 300,
    overscan: 5,
  });

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (isError || !exhibits) {
    return (
      <div>
        Error fetching exhibits {error && <div>{error.toString()}</div>}
      </div>
    );
  }

  const uniqueClusters = [...new Set(exhibits.map((e) => e.cluster))];
  const uniqueLocations = [...new Set(exhibits.map((e) => e.location))];
  const uniqueStatuses = [...new Set(exhibits.map((e) => e.status))];

  const isFilterApplied =
    clusterFilter !== null || locationFilter !== null || statusFilter !== null;

  const clearFilters = () => {
    setClusterFilter(null);
    setLocationFilter(null);
    setStatusFilter(null);
  };

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-6">Exhibit Inventory</h1>
      <div className="mb-4 flex flex-col md:flex-row gap-4">
        <SearchBar searchTerm={searchTerm} setSearchTerm={setSearchTerm} />
        <FilterSelect
          value={clusterFilter}
          onChange={setClusterFilter}
          options={uniqueClusters}
          placeholder="Filter by Cluster"
        />
        <FilterSelect
          value={locationFilter}
          onChange={setLocationFilter}
          options={uniqueLocations}
          placeholder="Filter by Location"
        />
        <FilterSelect
          value={statusFilter}
          onChange={setStatusFilter}
          options={uniqueStatuses}
          placeholder="Filter by Status"
        />
        {isFilterApplied && (
          <Button
            variant="outline"
            onClick={clearFilters}
            className="w-full md:w-auto"
          >
            <X className="w-4 h-4 mr-2" />
            Clear Filters
          </Button>
        )}
      </div>
      <div ref={parentRef} className="h-[calc(100vh-200px)] overflow-auto">
        <div
          style={{
            height: `${rowVirtualizer.getTotalSize()}px`,
            width: "100%",
            position: "relative",
          }}
        >
          {rowVirtualizer.getVirtualItems().map((virtualRow) => (
            <div
              key={virtualRow.index}
              style={{
                position: "absolute",
                top: 0,
                left: 0,
                width: "100%",
                height: `${virtualRow.size}px`,
                transform: `translateY(${virtualRow.start}px)`,
              }}
            >
              <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                {filteredExhibits
                  .slice(virtualRow.index * 3, virtualRow.index * 3 + 3)
                  .map((exhibit) => (
                    <ExhibitCard key={exhibit.id} exhibit={exhibit} />
                  ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

function SearchBar({
  searchTerm,
  setSearchTerm,
}: {
  searchTerm: string;
  setSearchTerm: (term: string) => void;
}) {
  return (
    <div className="flex-1 relative">
      <Search className="absolute left-2 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
      <Input
        type="text"
        placeholder="Search exhibits..."
        value={searchTerm}
        onChange={(e) => setSearchTerm(e.target.value)}
        className="w-full pl-8"
      />
    </div>
  );
}

function FilterSelect({
  value,
  onChange,
  options,
  placeholder,
}: {
  value: string | null;
  onChange: (value: string | null) => void;
  options: string[];
  placeholder: string;
}) {
  return (
    <Select value={value || ""} onValueChange={(val) => onChange(val || null)}>
      <SelectTrigger className="w-full md:w-[180px]">
        <SelectValue placeholder={placeholder} />
      </SelectTrigger>
      <SelectContent>
        {options.map((option) => (
          <SelectItem key={option} value={option}>
            {option}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
