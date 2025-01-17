import { useState } from "react";
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
import { MoreActions } from "@/components/generic/more-actions";
import { LinkDisplay } from "@/components/link-display";
import { NotesButton } from "@/components/generic/notes-button";
import type { Part } from "@/types";

interface PartTableProps {
  parts: Part[];
}

export function PartTable({ parts }: PartTableProps) {
  const [expandedRows, setExpandedRows] = useState<{ [key: string]: boolean }>(
    {}
  );

  const toggleRow = (id: string) => {
    setExpandedRows((prev) => ({
      ...prev,
      [id]: !prev[id],
    }));
  };

  return (
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
          {parts.map((part) => (
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
                  <NotesButton
                    id={part.id}
                    type="part"
                    name={part.name}
                    notes={part.notes}
                  />
                </TableCell>
                <TableCell>
                  <MoreActions id={part.id} type="part" />
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
