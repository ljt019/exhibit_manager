import { useState, useMemo } from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { ChevronDown, ChevronRight, Atom } from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";
import { cn } from "@/lib/utils";
import useGetParts from "@/hooks/data/queries/parts/useGetParts";
import { FilterSection } from "@/components/filter-section";
import { Part } from "@/types/types";
import { CreatePartDialog } from "@/components/create-part-dialog";
import { Error, Loading } from "@/components/loading-and-error";
import { NotesDialog } from "@/components/notes-dialog";
import { MoreActions } from "@/components/more-actions";
import { LinkDisplay } from "@/components/link-display";

export default function PartsInventory() {
  const { data: parts, isLoading, isError, error } = useGetParts();
  const [expandedRows, setExpandedRows] = useState<{ [key: string]: boolean }>(
    {}
  );
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

  const toggleRow = (id: string) => {
    setExpandedRows((prev) => ({
      ...prev,
      [id]: !prev[id],
    }));
  };

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
    return [...new Set((parts ?? []).flatMap((p) => p.exhibit_ids))];
  }, [parts]);

  const filterOptions = [
    {
      value: exhibitFilter,
      onChange: setExhibitFilter,
      options: uniqueExhibits,
      placeholder: "Filter by Exhibit",
    },
  ];

  return (
    <div className="container mx-auto p-4">
      <div className="flex justify-between w-full mb-6">
        <h1 className="text-2xl font-bold mt-[0.1rem]">Part Inventory</h1>
        <CreatePartDialog />
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
      {isLoading ? (
        <Loading />
      ) : isError || !parts ? (
        <Error error={error} name="parts" />
      ) : (
        <ScrollArea className="h-[calc(100vh-200px)]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-[30px]"></TableHead>
                <TableHead>Part Name</TableHead>
                <TableHead>Exhibits</TableHead>
                <TableHead>Link</TableHead>
                <TableHead>Notes</TableHead>
                <TableHead>Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {sortedParts.map((part) => (
                <AnimatePresence key={part.id}>
                  <TableRow
                    className="hover:bg-muted/50 transition-colors"
                    onClick={() => toggleRow(part.id)}
                  >
                    <TableCell className="w-[30px]">
                      <Button variant="ghost" size="icon" className="h-6 w-6">
                        {expandedRows[part.id] ? (
                          <ChevronDown className="h-4 w-4" />
                        ) : (
                          <ChevronRight className="h-4 w-4" />
                        )}
                      </Button>
                    </TableCell>
                    <TableCell className="font-medium">{part.name}</TableCell>
                    <TableCell>
                      <Badge variant="secondary">
                        <Atom className="w-4 h-4 mr-2" />
                        {part.exhibit_ids.length}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <LinkDisplay url={part.link} />
                    </TableCell>
                    <TableCell>
                      <NotesDialog partId={part.id} notes={part.notes} />
                    </TableCell>
                    <TableCell>
                      <MoreActions partId={part.id} />
                    </TableCell>
                  </TableRow>
                  {expandedRows[part.id] && (
                    <motion.tr
                      initial={{ opacity: 0, height: 0 }}
                      animate={{ opacity: 1, height: "auto" }}
                      exit={{ opacity: 0, height: 0 }}
                      transition={{ duration: 0.2 }}
                    >
                      <TableCell colSpan={6} className="bg-muted/50">
                        <motion.div
                          initial={{ opacity: 0, y: -10 }}
                          animate={{ opacity: 1, y: 0 }}
                          exit={{ opacity: 0, y: -10 }}
                          transition={{ duration: 0.2 }}
                        >
                          <div className="p-4">
                            <h4 className="text-sm font-semibold mb-2">
                              Connected Exhibits
                            </h4>
                            <ConnectedExhibitsDisplay
                              exhibitIds={part.exhibit_ids}
                            />
                          </div>
                        </motion.div>
                      </TableCell>
                    </motion.tr>
                  )}
                </AnimatePresence>
              ))}
            </TableBody>
          </Table>
        </ScrollArea>
      )}
      <div className="mt-4 text-sm text-muted-foreground">
        Showing {sortedParts.length} of {parts ? parts.length : 0} parts
      </div>
    </div>
  );
}

function ConnectedExhibitsDisplay({ exhibitIds }: { exhibitIds: string[] }) {
  return (
    <ul className="list-disc pl-4">
      {exhibitIds.map((exhibitId, index) => (
        <li key={index}>{exhibitId}</li>
      ))}
    </ul>
  );
}
