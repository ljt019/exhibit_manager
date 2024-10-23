import { useState, useMemo } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  MapPin,
  Hammer,
  StickyNote,
  ChevronDown,
  Search,
  X,
  Boxes,
} from "lucide-react";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

type Exhibit = {
  id: string;
  name: string;
  cluster: string;
  location: string;
  status: "operational" | "needs repair" | "out of service";
  parts: string[];
  notes: Array<{ timestamp: string; text: string }>;
};

const generateExhibits = (count: number): Exhibit[] => {
  const clusters = [
    "Biology",
    "Physics",
    "Chemistry",
    "Astronomy",
    "Geology",
    "Technology",
  ];
  const locations = [
    "Hall A",
    "Hall B",
    "Hall C",
    "Hall D",
    "Hall E",
    "Outdoor Area",
  ];
  const statuses: ("operational" | "needs repair" | "out of service")[] = [
    "operational",
    "needs repair",
    "out of service",
  ];

  return Array.from({ length: count }, (_, i) => ({
    id: (i + 1).toString(),
    name: `Exhibit ${i + 1}`,
    cluster: clusters[Math.floor(Math.random() * clusters.length)],
    location: locations[Math.floor(Math.random() * locations.length)],
    status: statuses[Math.floor(Math.random() * statuses.length)],
    parts: Array.from(
      { length: Math.floor(Math.random() * 10) + 3 },
      (_, j) => `Part ${j + 1}`
    ),
    notes: Array.from(
      { length: Math.floor(Math.random() * 20) + 1 },
      (_, j) => ({
        timestamp: new Date(Date.now() - j * 86400000)
          .toISOString()
          .split("T")[0],
        text: `Note ${j + 1} for Exhibit ${i + 1}`,
      })
    ),
  }));
};

const exhibits = generateExhibits(100);

const statusColors = {
  operational: "border-green-500 text-green-500",
  "needs repair": "border-yellow-500 text-yellow-500",
  "out of service": "border-red-500 text-red-500",
};

export default function ExhibitInventory() {
  const [searchTerm, setSearchTerm] = useState("");
  const [clusterFilter, setClusterFilter] = useState<string | null>(null);
  const [locationFilter, setLocationFilter] = useState<string | null>(null);
  const [statusFilter, setStatusFilter] = useState<string | null>(null);

  const filteredExhibits = useMemo(() => {
    return exhibits.filter((exhibit) => {
      const nameMatch = exhibit.name
        .toLowerCase()
        .includes(searchTerm.toLowerCase());
      const clusterMatch = clusterFilter
        ? exhibit.cluster === clusterFilter
        : true;
      const locationMatch = locationFilter
        ? exhibit.location === locationFilter
        : true;
      const statusMatch = statusFilter ? exhibit.status === statusFilter : true;
      return nameMatch && clusterMatch && locationMatch && statusMatch;
    });
  }, [searchTerm, clusterFilter, locationFilter, statusFilter]);

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
        <Select value={clusterFilter || ""} onValueChange={setClusterFilter}>
          <SelectTrigger className="w-full md:w-[180px]">
            <SelectValue placeholder="Filter by Cluster" />
          </SelectTrigger>
          <SelectContent>
            {uniqueClusters.map((cluster) => (
              <SelectItem key={cluster} value={cluster}>
                {cluster}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <Select value={locationFilter || ""} onValueChange={setLocationFilter}>
          <SelectTrigger className="w-full md:w-[180px]">
            <SelectValue placeholder="Filter by Location" />
          </SelectTrigger>
          <SelectContent>
            {uniqueLocations.map((location) => (
              <SelectItem key={location} value={location}>
                {location}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <Select value={statusFilter || ""} onValueChange={setStatusFilter}>
          <SelectTrigger className="w-full md:w-[180px]">
            <SelectValue placeholder="Filter by Status" />
          </SelectTrigger>
          <SelectContent>
            {uniqueStatuses.map((status) => (
              <SelectItem key={status} value={status}>
                {status}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
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
      <ScrollArea className="h-[calc(100vh-200px)]">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          {filteredExhibits.map((exhibit) => (
            <Card key={exhibit.id} className="w-full overflow-hidden">
              <CardHeader className="p-4 pb-2">
                <div className="flex justify-between items-start">
                  <CardTitle className="text-lg">{exhibit.name}</CardTitle>
                  <Badge
                    className={`${
                      statusColors[exhibit.status]
                    } bg-transparent border`}
                  >
                    {exhibit.status}
                  </Badge>
                </div>
              </CardHeader>
              <CardContent className="p-4 pt-0">
                <div className="space-y-2">
                  <div className="flex items-center justify-between text-sm text-muted-foreground">
                    <div className="flex items-center gap-1">
                      <Boxes className="w-3 h-3" />
                      <span>{exhibit.cluster}</span>
                    </div>
                    <div className="flex items-center gap-1">
                      <MapPin className="w-3 h-3" />
                      <span>{exhibit.location}</span>
                    </div>
                  </div>
                  <Collapsible>
                    <CollapsibleTrigger asChild>
                      <Button variant="outline" size="sm" className="w-full">
                        <Hammer className="w-3 h-3 mr-2" />
                        Parts ({exhibit.parts.length})
                        <ChevronDown className="w-3 h-3 ml-auto" />
                      </Button>
                    </CollapsibleTrigger>
                    <CollapsibleContent className="mt-2">
                      <ScrollArea className="h-24 w-full rounded-md border p-2">
                        <ul className="text-sm">
                          {exhibit.parts.map((part, index) => (
                            <li key={index}>{part}</li>
                          ))}
                        </ul>
                      </ScrollArea>
                    </CollapsibleContent>
                  </Collapsible>
                  <Dialog>
                    <DialogTrigger asChild>
                      <Button variant="outline" size="sm" className="w-full">
                        <StickyNote className="w-3 h-3 mr-2" />
                        View Notes ({exhibit.notes.length})
                      </Button>
                    </DialogTrigger>
                    <DialogContent className="max-w-2xl">
                      <DialogHeader>
                        <DialogTitle>{exhibit.name} - Notes</DialogTitle>
                      </DialogHeader>
                      <ScrollArea className="h-[60vh] mt-4">
                        <ul className="space-y-4">
                          {exhibit.notes.map((note, index) => (
                            <li key={index} className="border-b pb-2">
                              <span className="font-medium">
                                {note.timestamp}:
                              </span>{" "}
                              {note.text}
                            </li>
                          ))}
                        </ul>
                      </ScrollArea>
                    </DialogContent>
                  </Dialog>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </ScrollArea>
    </div>
  );
}
