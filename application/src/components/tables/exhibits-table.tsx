import React, { useState, useMemo, useCallback } from "react";
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
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  SelectSeparator,
} from "@/components/ui/select";
import useGetParts from "@/hooks/data/queries/parts/useGetParts";
import useAddExistingPart from "@/hooks/data/mutations/exhibits/useAddExistingPart";
import useChangeExhibitStatus, {
  type ExhibitStatus,
} from "@/hooks/data/mutations/exhibits/useChangeExhibitStatus";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Check } from "lucide-react";

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
  const addExistingPartMutation = useAddExistingPart();
  const [expandedRows, setExpandedRows] = useState<ExpandedState>({});
  const [isAddPartModalOpen, setIsAddPartModalOpen] = useState(false);
  const [selectedExhibitId, setSelectedExhibitId] = useState<string | null>(
    null
  );
  const [selectedPartId, setSelectedPartId] = useState<string | null>(null);
  const { data: allParts } = useGetParts();
  const changeStatus = useChangeExhibitStatus();

  const sortedExhibits = useMemo(() => {
    return [...exhibits].sort((a, b) => a.name.localeCompare(b.name));
  }, [exhibits]);

  const toggleRow = useCallback((id: string) => {
    setExpandedRows((prev) => ({ ...prev, [id]: !prev[id] }));
  }, []);

  const getStatusBadge = useCallback((status: string) => {
    const variants = {
      operational: "bg-green-500/10 text-green-500 hover:bg-green-500/20",
      "needs repair": "bg-yellow-500/10 text-yellow-500 hover:bg-yellow-500/20",
      "out of service": "bg-red-500/10 text-red-500 hover:bg-red-500/20",
    };
    return (
      variants[status.toLowerCase() as keyof typeof variants] ||
      variants["operational"]
    );
  }, []);

  const handleAddPart = (exhibitId: string) => {
    setSelectedExhibitId(exhibitId);
    setIsAddPartModalOpen(true);
  };

  const handleAddPartSuccess = () => {
    setIsAddPartModalOpen(false);
    setSelectedExhibitId(null);
    setSelectedPartId(null);
    refetchExhibits();
  };

  const handleAddExistingPart = async () => {
    if (selectedExhibitId && selectedPartId) {
      try {
        await addExistingPartMutation.mutateAsync({
          exhibitId: selectedExhibitId,
          partId: selectedPartId,
        });
        handleAddPartSuccess();
      } catch (error) {
        console.error("Failed to add existing part:", error);
      }
    }
  };

  const handleStatusChange = (exhibitId: string, newStatus: ExhibitStatus) => {
    if (
      exhibits.find((exhibit) => exhibit.id === exhibitId)?.status === newStatus
    )
      return;

    changeStatus.mutate(
      { exhibitId, newStatus },
      {
        onSuccess: () => {
          refetchExhibits();
        },
      }
    );
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
              <React.Fragment key={exhibit.id}>
                <AnimatePresence>
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
                    <TableCell className="font-medium">
                      {exhibit.name}
                    </TableCell>
                    <TableCell>{exhibit.location}</TableCell>
                    <TableCell>{exhibit.cluster}</TableCell>
                    <TableCell>
                      <DropdownMenu>
                        <DropdownMenuTrigger>
                          <Badge
                            variant="secondary"
                            className={cn(
                              getStatusBadge(exhibit.status),
                              "hover:opacity-80 transition-opacity"
                            )}
                          >
                            {exhibit.status
                              .split(" ")
                              .map(
                                (word) =>
                                  word.charAt(0).toUpperCase() + word.slice(1)
                              )
                              .join(" ")}
                          </Badge>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent
                          className="w-40"
                          side="bottom"
                          align="start"
                        >
                          {[
                            "Operational",
                            "Needs Repair",
                            "Out of Service",
                          ].map((status) => (
                            <DropdownMenuItem
                              key={status}
                              onClick={(e) => {
                                e.stopPropagation();
                                handleStatusChange(
                                  exhibit.id,
                                  status.toLowerCase() as ExhibitStatus
                                );
                              }}
                              className={cn(
                                "flex items-center text-center justify-between rounded-none",
                                getStatusBadge(status.toLowerCase()),
                                "hover:opacity-80 transition-opacity"
                              )}
                            >
                              {status}
                              {exhibit.status.toLowerCase() ===
                                status.toLowerCase() && (
                                <Check className="h-4 w-4 text-current" />
                              )}
                            </DropdownMenuItem>
                          ))}
                        </DropdownMenuContent>
                      </DropdownMenu>
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
                </AnimatePresence>
                {expandedRows[exhibit.id] && (
                  <motion.tr
                    key={`${exhibit.id}-details`}
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
                            className="w-36 h-36 object-fill rounded-md"
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
                                <Plus className="h-4 w-4" />
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
              </React.Fragment>
            ))}
          </TableBody>
        </Table>
      </ScrollArea>
      <Dialog open={isAddPartModalOpen} onOpenChange={setIsAddPartModalOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Add Part to Exhibit</DialogTitle>
          </DialogHeader>
          <Select
            onValueChange={(value) => setSelectedPartId(value)}
            onOpenChange={(open) => {
              if (open) {
                setTimeout(() => {
                  const content = document.querySelector(".select-scroll-area");
                  if (content) {
                    content.addEventListener("wheel", (e) => {
                      e.stopPropagation();
                    });
                  }
                }, 0);
              }
            }}
          >
            <SelectTrigger>
              <SelectValue placeholder="Select a part" />
            </SelectTrigger>
            <SelectContent>
              <ScrollArea className="h-[200px] select-scroll-area">
                <SelectItem
                  value="new"
                  className="cursor-pointer hover:bg-muted/50 transition-colors"
                >
                  Create New Part
                </SelectItem>
                <SelectSeparator />
                {allParts?.map((part) => (
                  <SelectItem
                    key={part.id}
                    value={part.id}
                    className="cursor-pointer hover:bg-muted/50 transition-colors"
                  >
                    {part.name}
                  </SelectItem>
                ))}
              </ScrollArea>
            </SelectContent>
          </Select>
          {selectedPartId === "new" ? (
            <CreatePartForm
              onSuccess={handleAddPartSuccess}
              onCancel={() => setIsAddPartModalOpen(false)}
              exhibitId={selectedExhibitId || undefined}
            />
          ) : (
            <Button onClick={handleAddExistingPart} disabled={!selectedPartId}>
              Add Existing Part
            </Button>
          )}
        </DialogContent>
      </Dialog>
    </>
  );
}
