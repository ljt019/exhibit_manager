import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { Atom } from "lucide-react";
import { MoreActions } from "@/components/generic/more-actions";
import { LinkDisplay } from "@/components/link-display";
import { NotesButton } from "@/components/generic/notes-button";
import type { Part } from "@/types";

interface PartTableProps {
  parts: Part[];
}

export function PartTable({ parts }: PartTableProps) {
  return (
    <ScrollArea className="h-[calc(100vh-200px)]">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Part Name</TableHead>
            <TableHead>Exhibits</TableHead>
            <TableHead>Link</TableHead>
            <TableHead>Notes</TableHead>
            <TableHead>Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {parts.map((part) => (
            <TableRow
              key={part.id}
              className="hover:bg-muted/50 transition-colors"
            >
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
          ))}
        </TableBody>
      </Table>
    </ScrollArea>
  );
}
