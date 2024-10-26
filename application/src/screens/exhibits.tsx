import { useState, useMemo, useCallback, useRef, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Button } from "@/components/ui/button";
import { Search, X, Filter } from "lucide-react";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { ScrollArea } from "@/components/ui/scroll-area";
import useGetExhibits from "@/hooks/useGetExhibits";
import { ExhibitCard } from "@/components/exhibit-card";
import { useVirtualizer } from "@tanstack/react-virtual";
import debounce from "lodash.debounce";

export default function Component() {
  const { data: exhibits, isLoading, isError, error } = useGetExhibits();

  const [searchTerm, setSearchTerm] = useState("");
  const [clusterFilter, setClusterFilter] = useState<string | null>(null);
  const [locationFilter, setLocationFilter] = useState<string | null>(null);
  const [statusFilter, setStatusFilter] = useState<string | null>(null);

  const debouncedSetSearchTerm = useCallback(
    debounce((value: string) => setSearchTerm(value), 300),
    []
  );

  const filteredExhibits = useMemo(() => {
    if (!exhibits) return [];

    return exhibits.filter((exhibit) => {
      if (
        searchTerm &&
        !exhibit.name.toLowerCase().includes(searchTerm.toLowerCase())
      ) {
        return false;
      }
      if (clusterFilter && exhibit.cluster !== clusterFilter) {
        return false;
      }
      if (locationFilter && exhibit.location !== locationFilter) {
        return false;
      }
      if (statusFilter && exhibit.status !== statusFilter) {
        return false;
      }
      return true;
    });
  }, [exhibits, searchTerm, clusterFilter, locationFilter, statusFilter]);

  const [showFilters, setShowFilters] = useState<boolean>(false);

  const parentRef = useRef<HTMLDivElement>(null);
  const [rowHeight, setRowHeight] = useState(300);

  const rowVirtualizer = useVirtualizer({
    count: Math.ceil(filteredExhibits.length / 3),
    getScrollElement: () => parentRef.current,
    estimateSize: () => rowHeight,
    overscan: 5,
  });

  useEffect(() => {
    const updateRowHeight = () => {
      if (parentRef.current) {
        const firstRow = parentRef.current.querySelector('[data-index="0"]');
        if (firstRow) {
          setRowHeight(firstRow.clientHeight);
        }
      }
    };

    updateRowHeight();
    window.addEventListener("resize", updateRowHeight);

    return () => {
      window.removeEventListener("resize", updateRowHeight);
    };
  }, [filteredExhibits]);

  useEffect(() => {
    rowVirtualizer.measure();
  }, [rowHeight, rowVirtualizer]);

  const clearFilters = () => {
    setClusterFilter(null);
    setLocationFilter(null);
    setStatusFilter(null);

    if (showFilters === true) {
      setShowFilters(false);
    }
  };

  const isFilterApplied =
    clusterFilter !== null || locationFilter !== null || statusFilter !== null;

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

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-6">Exhibit Inventory</h1>
      <div className="mb-4 flex flex-col md:flex-row gap-4">
        <div className="w-full md:w-[41.5rem]">
          <SearchBar setSearchTerm={debouncedSetSearchTerm} />
        </div>
        <Button
          onClick={() => {
            setShowFilters(!showFilters);
            clearFilters();
          }}
          className={`w-full md:w-auto ${
            showFilters
              ? "text-foreground outline outline-1 outline-foreground"
              : "text-muted-foreground"
          }`}
          variant="outline"
        >
          <Filter className="w-4 h-4" />
        </Button>
        <AnimatePresence>
          {showFilters && (
            <motion.div
              initial={{ opacity: 0, x: -20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              transition={{ duration: 0.3, ease: "easeInOut" }}
              className="flex flex-wrap gap-2 overflow-hidden"
            >
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
              <AnimatePresence>
                {isFilterApplied && (
                  <motion.div
                    initial={{ opacity: 0, x: -20 }}
                    animate={{ opacity: 1, x: 0 }}
                    exit={{ opacity: 0, x: -20 }}
                    transition={{ duration: 0.3, ease: "easeInOut" }}
                  >
                    <Button
                      variant="outline"
                      onClick={clearFilters}
                      className="w-full md:w-auto"
                    >
                      <X className="w-4 h-4 text-destructive" />
                    </Button>
                  </motion.div>
                )}
              </AnimatePresence>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
      <ScrollArea className="h-[calc(100vh-200px)]">
        <div
          ref={parentRef}
          style={{
            height: `${rowVirtualizer.getTotalSize()}px`,
            width: "100%",
            position: "relative",
          }}
        >
          {rowVirtualizer.getVirtualItems().map((virtualRow) => (
            <div
              key={virtualRow.key}
              data-index={virtualRow.index}
              ref={rowVirtualizer.measureElement}
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
                {[0, 1, 2].map((columnIndex) => {
                  const exhibitIndex = virtualRow.index * 3 + columnIndex;
                  const exhibit = filteredExhibits[exhibitIndex];
                  return exhibit ? (
                    <ExhibitCard key={exhibit.id} exhibit={exhibit} />
                  ) : null;
                })}
              </div>
            </div>
          ))}
        </div>
      </ScrollArea>
      <div className="mt-4 text-sm text-muted-foreground">
        Showing {filteredExhibits.length} of {exhibits.length} exhibits
      </div>
    </div>
  );
}

function SearchBar({
  setSearchTerm,
}: {
  setSearchTerm: (term: string) => void;
}) {
  return (
    <div className="flex-1 relative">
      <Search className="absolute left-2 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
      <Input
        type="text"
        placeholder="Search exhibits..."
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
