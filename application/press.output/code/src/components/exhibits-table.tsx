import { useState, useMemo, useCallback } from "react";
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
import { ScrollArea } from "@/components/ui/scroll-area";
import { ChevronDown, ChevronRight } from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";
import { cn } from "@/lib/utils";
import type { Exhibit } from "@/types";
import { MoreActions } from "@/components/more-actions-exhibits";
import { NotesButton } from "@/components/notes-button";
import { PartsList } from "@/components/parts-list";
import { SponsorshipDisplay } from "@/components/sponsorship-display";

interface ExpandedState {
  [key: string]: boolean;
}

export function ExhibitsTable({ exhibits }: { exhibits: Exhibit[] }) {
  const [expandedRows, setExpandedRows] = useState<ExpandedState>({});

  const sortedExhibits = useMemo(() => {
    return [...exhibits].sort((a, b) => a.name.localeCompare(b.name));
  }, [exhibits]);

  const toggleRow = useCallback((id: string) => {
    setExpandedRows((prev) => ({
      ...prev,
      [id]: !prev[id],
    }));
  }, []);

  const getStatusBadge = useCallback((status: string) => {
    const variants = {
      Operational: "bg-green-500/10 text-green-500 hover:bg-green-500/20",
      "Needs Repair": "bg-yellow-500/10 text-yellow-500 hover:bg-yellow-500/20",
      "Out of Service": "bg-red-500/10 text-red-500 hover:bg-red-500/20",
    };
    return variants[status as keyof typeof variants] || variants["Operational"];
  }, []);

  return (
    <ScrollArea className="h-[calc(100vh-200px)] w-full">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[30px]"></TableHead>
            <TableHead>Name</TableHead>
            <TableHead>Location</TableHead>
            <TableHead>Cluster</TableHead>
            <TableHead>Status</TableHead>
            <TableHead>Notes</TableHead>
            <TableHead>Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {sortedExhibits.map((exhibit) => (
            <AnimatePresence key={exhibit.id}>
              <TableRow
                className="hover:bg-muted/50 transition-colors"
                onClick={() => toggleRow(exhibit.id)}
              >
                <TableCell className="w-[30px]">
                  <Button variant="ghost" size="icon" className="h-6 w-6">
                    {expandedRows[exhibit.id] ? (
                      <ChevronDown className="h-4 w-4" />
                    ) : (
                      <ChevronRight className="h-4 w-4" />
                    )}
                  </Button>
                </TableCell>
                <TableCell className="font-medium">{exhibit.name}</TableCell>
                <TableCell>{exhibit.location}</TableCell>
                <TableCell>{exhibit.cluster}</TableCell>
                <TableCell>
                  <Badge
                    variant="secondary"
                    className={cn(getStatusBadge(exhibit.status))}
                  >
                    {exhibit.status}
                  </Badge>
                </TableCell>
                <TableCell>
                  <NotesButton
                    exhibitId={exhibit.id}
                    name={exhibit.name}
                    notes={exhibit.notes}
                  />
                </TableCell>
                <TableCell>
                  <MoreActions exhibitId={exhibit.id} />
                </TableCell>
              </TableRow>
              {expandedRows[exhibit.id] && (
                <motion.tr
                  initial={{ opacity: 0, height: 0 }}
                  animate={{ opacity: 1, height: "auto" }}
                  exit={{ opacity: 0, height: 0 }}
                  transition={{ duration: 0.2 }}
                >
                  <TableCell colSpan={7} className="bg-muted/50">
                    <motion.div
                      initial={{ opacity: 0, y: -10 }}
                      animate={{ opacity: 1, y: 0 }}
                      exit={{ opacity: 0, y: -10 }}
                      transition={{ duration: 0.2 }}
                    >
                      <div className="p-4 flex space-x-4">
                        <img
                          src={exhibit.image_url}
                          alt={exhibit.name}
                          className="w-36 h-36 object-cover rounded-md"
                        />
                        <div className="flex-1">
                          <h4 className="text-sm font-semibold mb-2">
                            Description
                          </h4>
                          <p className="text-sm text-muted-foreground mb-4">
                            {exhibit.description || "No description available."}
                          </p>
                          <SponsorshipDisplay
                            sponsorship={exhibit.sponsorship}
                          />
                          <h4 className="text-sm font-semibold mb-2">Parts</h4>
                          <PartsList partIds={exhibit.part_ids} />
                        </div>
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
  );
}