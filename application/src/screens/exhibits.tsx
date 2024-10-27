import React, {
  useState,
  useMemo,
  useCallback,
  useRef,
  useEffect,
} from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Button } from "@/components/ui/button";
import { Search, X, Filter, Plus, Upload } from "lucide-react";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogFooter,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import useGetExhibits from "@/hooks/useGetExhibits";
import useCreateExhibit from "@/hooks/useCreateExhibit";
import { ExhibitCard } from "@/components/exhibit-card";
import { useVirtualizer } from "@tanstack/react-virtual";
import debounce from "lodash.debounce";
import type { Exhibit } from "@/components/exhibit-card";

export default function ExhibitInventory() {
  const { data: exhibits, isLoading, isError, error } = useGetExhibits();
  const createExhibitMutation = useCreateExhibit();

  const [searchTerm, setSearchTerm] = useState("");
  const [clusterFilter, setClusterFilter] = useState<string | null>(null);
  const [locationFilter, setLocationFilter] = useState<string | null>(null);
  const [statusFilter, setStatusFilter] = useState<string | null>(null);
  const [showFilters, setShowFilters] = useState<boolean>(false);
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [newExhibit, setNewExhibit] = useState<Partial<Exhibit>>({
    name: "",
    cluster: "",
    location: "",
    status: "operational",
    image_url: "",
  });

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

  const handleCreateExhibit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createExhibitMutation.mutateAsync(newExhibit as Exhibit);
      setIsDialogOpen(false);
      setNewExhibit({
        name: "",
        cluster: "",
        location: "",
        status: "operational",
        image_url: "",
      });
    } catch (error) {
      console.error("Failed to create exhibit:", error);
    }
  };

  const handleImageUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onloadend = () => {
        setNewExhibit({ ...newExhibit, image_url: reader.result as string });
      };
      reader.readAsDataURL(file);
    }
  };

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
      <div className="flex justify-between items-center mb-6">
        <div className="flex justify-between w-full items-center">
          <h1 className="text-2xl font-bold">Exhibit Inventory</h1>
          <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
            <DialogTrigger asChild>
              <Button variant="outline">
                <Plus className="w-4 h-4" />
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Create New Exhibit</DialogTitle>
                <DialogDescription>
                  Fill in the details for the new exhibit.
                </DialogDescription>
              </DialogHeader>
              <form onSubmit={handleCreateExhibit}>
                <div className="grid gap-4 py-4">
                  <div className="grid grid-cols-4 items-center gap-4">
                    <Label htmlFor="name" className="text-right">
                      Name
                    </Label>
                    <Input
                      id="name"
                      value={newExhibit.name}
                      onChange={(e) =>
                        setNewExhibit({ ...newExhibit, name: e.target.value })
                      }
                      className="col-span-3"
                    />
                  </div>
                  <div className="grid grid-cols-4 items-center gap-4">
                    <Label htmlFor="cluster" className="text-right">
                      Cluster
                    </Label>
                    <Input
                      id="cluster"
                      value={newExhibit.cluster}
                      onChange={(e) =>
                        setNewExhibit({
                          ...newExhibit,
                          cluster: e.target.value,
                        })
                      }
                      className="col-span-3"
                    />
                  </div>
                  <div className="grid grid-cols-4 items-center gap-4">
                    <Label htmlFor="location" className="text-right">
                      Location
                    </Label>
                    <Input
                      id="location"
                      value={newExhibit.location}
                      onChange={(e) =>
                        setNewExhibit({
                          ...newExhibit,
                          location: e.target.value,
                        })
                      }
                      className="col-span-3"
                    />
                  </div>
                  <div className="grid grid-cols-4 items-center gap-4">
                    <Label htmlFor="status" className="text-right">
                      Status
                    </Label>
                    <Select
                      value={newExhibit.status}
                      onValueChange={(value) =>
                        setNewExhibit({
                          ...newExhibit,
                          status: value as Exhibit["status"],
                        })
                      }
                    >
                      <SelectTrigger className="col-span-3">
                        <SelectValue placeholder="Select status" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="operational">Operational</SelectItem>
                        <SelectItem value="needs repair">
                          Needs Repair
                        </SelectItem>
                        <SelectItem value="out of service">
                          Out of Service
                        </SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <div className="grid grid-cols-4 items-center gap-4">
                    <Label htmlFor="image" className="text-right">
                      Image
                    </Label>
                    <div className="col-span-3">
                      <div className="flex items-center justify-center w-full">
                        <label
                          htmlFor="dropzone-file"
                          className="flex flex-col items-center justify-center w-full h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
                        >
                          <div className="flex flex-col items-center justify-center pt-5 pb-6">
                            <Upload className="w-8 h-8 mb-4 text-gray-500 dark:text-gray-400" />
                            <p className="mb-2 text-sm text-gray-500 dark:text-gray-400">
                              <span className="font-semibold">
                                Click to upload
                              </span>{" "}
                              or drag and drop
                            </p>
                            <p className="text-xs text-gray-500 dark:text-gray-400">
                              SVG, PNG, JPG or GIF (MAX. 800x400px)
                            </p>
                          </div>
                          <input
                            id="dropzone-file"
                            type="file"
                            className="hidden"
                            accept="image/*"
                            onChange={handleImageUpload}
                          />
                        </label>
                      </div>
                      {newExhibit.image_url && (
                        <div className="mt-4">
                          <img
                            src={newExhibit.image_url}
                            alt="Uploaded exhibit"
                            className="max-w-full h-auto"
                          />
                        </div>
                      )}
                    </div>
                  </div>
                </div>
                <DialogFooter>
                  <Button type="submit">Create Exhibit</Button>
                </DialogFooter>
              </form>
            </DialogContent>
          </Dialog>
        </div>
      </div>
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
