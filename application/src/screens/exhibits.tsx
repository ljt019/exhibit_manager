import { useState, useMemo } from "react";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";
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
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import useGetExhibits from "@/hooks/useGetExhibits";
import useCreateExhibit from "@/hooks/useCreateExhibit";
import type { Exhibit } from "@/components/exhibit-card";
import { ExhibitList } from "@/components/exhibit-list";
import { FilterSection } from "@/components/filter-section";

export default function ExhibitInventory() {
  const { data: exhibits, isLoading, isError, error } = useGetExhibits();
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

  const filteredExhibits = useFilteredExhibits(
    exhibits || [],
    searchTerm,
    filters
  );

  const clearFilters = () => {
    setFilters({ cluster: null, location: null, status: null });
    if (showFilters) setShowFilters(false);
  };

  const isFilterApplied = Object.values(filters).some(
    (filter) => filter !== null
  );

  if (isLoading) return <div>Loading...</div>;
  if (isError || !exhibits)
    return (
      <div>
        Error fetching exhibits {error && <div>{error.toString()}</div>}
      </div>
    );

  const uniqueValues = getUniqueValues(exhibits);

  const filterOptions = [
    {
      value: filters.cluster,
      onChange: (value: any) =>
        setFilters((prev) => ({ ...prev, cluster: value })),
      options: uniqueValues.clusters,
      placeholder: "Filter by Cluster",
    },
    {
      value: filters.location,
      onChange: (value: any) =>
        setFilters((prev) => ({ ...prev, location: value })),
      options: uniqueValues.locations,
      placeholder: "Filter by Location",
    },
    {
      value: filters.status,
      onChange: (value: any) =>
        setFilters((prev) => ({ ...prev, status: value })),
      options: uniqueValues.statuses,
      placeholder: "Filter by Status",
    },
  ];

  return (
    <div className="container mx-auto p-4">
      <Header />
      <FilterSection
        showFilters={showFilters}
        setShowFilters={setShowFilters}
        clearFilters={clearFilters}
        isFilterApplied={isFilterApplied}
        setSearchTerm={setSearchTerm}
        filterOptions={filterOptions}
        searchBarName="exhibits"
      />
      <ExhibitList filteredExhibits={filteredExhibits} />
      <Footer
        totalExhibits={exhibits.length}
        filteredExhibits={filteredExhibits.length}
      />
    </div>
  );
}

function Header() {
  return (
    <div className="flex justify-between items-center mb-6">
      <h1 className="text-2xl font-bold">Exhibit Inventory</h1>
      <CreateExhibitDialog />
    </div>
  );
}

function CreateExhibitDialog() {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const createExhibitMutation = useCreateExhibit();
  const [newExhibit, setNewExhibit] = useState<Partial<Exhibit>>({
    name: "",
    cluster: "",
    location: "",
    status: "operational",
    image_url: "",
  });

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

  return (
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
                  <SelectItem value="needs repair">Needs Repair</SelectItem>
                  <SelectItem value="out of service">Out of Service</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="image" className="text-right">
                Image
              </Label>
              <Input
                id="image"
                type="file"
                accept="image/*"
                onChange={handleImageUpload}
                className="col-span-3"
              />
            </div>
          </div>
          <DialogFooter>
            <Button type="submit">Create Exhibit</Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

interface FooterProps {
  totalExhibits: number;
  filteredExhibits: number;
}

function Footer({ totalExhibits, filteredExhibits }: FooterProps) {
  return (
    <div className="mt-4 text-sm text-muted-foreground">
      Showing {filteredExhibits} of {totalExhibits} exhibits
    </div>
  );
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
