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
import { ChevronDown, ChevronRight, Plus } from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";
import { cn } from "@/lib/utils";
import type { Exhibit } from "@/types";
import { MoreActions } from "@/components/generic/more-actions";
import { NotesButton } from "@/components/generic/notes-button";
import { PartsList } from "@/components/parts-list";
import { SponsorshipDisplay } from "@/components/sponsorship-display";
import { CreatePartForm } from "@/components/forms/create-part-form";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";

interface ExpandedState {
  [key: string]: boolean;
}

export function ExhibitsTable({
  exhibits,
  refetchExhibits,
}: {
  exhibits: Exhibit[];
  refetchExhibits: () => void;
}) {
  const [expandedRows, setExpandedRows] = useState<ExpandedState>({});
  const [isAddPartModalOpen, setIsAddPartModalOpen] = useState(false);
  const [selectedExhibitId, setSelectedExhibitId] = useState<string | null>(
    null
  );

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

  const handleAddPart = (exhibitId: string) => {
    setSelectedExhibitId(exhibitId);
    setIsAddPartModalOpen(true);
  };

  const handleAddPartSuccess = () => {
    setIsAddPartModalOpen(false);
    setSelectedExhibitId(null);
  };

  return (
    <>
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
                <TableRow className="hover:bg-muted/50 transition-colors">
                  <TableCell className="w-[30px]">
                    <Button
                      variant="ghost"
                      size="icon"
                      className="h-6 w-6"
                      onClick={(e) => {
                        e.stopPropagation();
                        toggleRow(exhibit.id);
                      }}
                    >
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
                      id={exhibit.id}
                      type="exhibit"
                      name={exhibit.name}
                      notes={exhibit.notes}
                    />
                  </TableCell>
                  <TableCell>
                    <MoreActions id={exhibit.id} type="exhibit" />
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
                            src={exhibit.image_url || "/placeholder.svg"}
                            alt={exhibit.name}
                            className="w-36 h-36 object-cover rounded-md"
                          />
                          <div className="flex-1">
                            <h4 className="text-sm font-semibold mb-2">
                              Description
                            </h4>
                            <p className="text-sm text-muted-foreground mb-4">
                              {exhibit.description ||
                                "No description available."}
                            </p>
                            <SponsorshipDisplay
                              sponsorship={exhibit.sponsorship}
                            />
                            <div className="flex justify-between items-center mb-2">
                              <h4 className="text-sm font-semibold">Parts</h4>
                              <Button
                                variant="outline"
                                size="sm"
                                onClick={() => handleAddPart(exhibit.id)}
                              >
                                <Plus className="h-4 w-4 mr-2" />
                                Add Part
                              </Button>
                            </div>
                            <PartsList
                              partIds={exhibit.part_ids}
                              exhibitId={exhibit.id}
                              refetchPartIds={refetchExhibits}
                            />
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
      <Dialog open={isAddPartModalOpen} onOpenChange={setIsAddPartModalOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Add New Part</DialogTitle>
          </DialogHeader>
          <CreatePartForm
            onSuccess={handleAddPartSuccess}
            exhibitId={selectedExhibitId || undefined}
          />
        </DialogContent>
      </Dialog>
    </>
  );
}
