import { useState } from "react";
import {
  Hammer,
  ExternalLink,
  Loader2,
  AlertCircle,
  Plus,
  ChevronRight,
} from "lucide-react";
import useGetExhibitParts from "@/hooks/data/queries/useGetExhibitParts";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Badge } from "@/components/ui/badge";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { CreatePartDialog } from "@/components/create-part-dialog";
import { motion, AnimatePresence } from "framer-motion";
import type { Note, Part } from "@/types";

function PartsButton({
  name,
  parts,
  exhibitId,
}: {
  name: string;
  parts: string[];
  exhibitId: string;
}) {
  const [isOpen, setIsOpen] = useState(false);

  if (!parts || parts.length === 0) return null;

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button
          variant="outline"
          size="sm"
          className="w-full transition-all duration-200 hover:bg-primary hover:text-primary-foreground"
        >
          <Hammer className="w-4 h-4 mr-2" />
          Parts ({parts.length})
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-3xl max-h-[90vh] flex flex-col">
        <DialogHeader className="flex flex-row items-center justify-between">
          <DialogTitle className="text-2xl font-bold">
            {name} - Parts
          </DialogTitle>
          <CreatePartDialog exhibitId={exhibitId} />
        </DialogHeader>
        <PartsInnerDialog parts={parts} />
      </DialogContent>
    </Dialog>
  );
}

function PartsInnerDialog({ parts }: { parts: string[] }) {
  const { data, isLoading, isError } = useGetExhibitParts(parts);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-[60vh]">
        <Loader2 className="h-12 w-12 animate-spin text-primary" />
      </div>
    );
  }

  if (isError || !data) {
    return (
      <div className="flex flex-col items-center justify-center h-[60vh] text-destructive">
        <AlertCircle className="h-12 w-12 mb-4" />
        <p className="text-lg font-semibold">Error loading parts</p>
        <p className="text-sm text-muted-foreground">Please try again later</p>
      </div>
    );
  }

  return (
    <ScrollArea className="h-[50rem] rounded-md">
      <div className="space-y-2 pr-4">
        {data.map((part) => (
          <PartItem key={part.id} part={part} />
        ))}
      </div>
    </ScrollArea>
  );
}

function PartItem({ part }: { part: Part }) {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <motion.div
      initial={false}
      animate={isOpen ? "open" : "closed"}
      className="border rounded-md overflow-hidden"
    >
      <motion.button
        className="flex items-center justify-between w-full p-4 hover:bg-muted/50 transition-colors"
        onClick={() => setIsOpen(!isOpen)}
      >
        <div className="flex items-center space-x-4">
          <motion.div
            variants={{
              open: { rotate: 90 },
              closed: { rotate: 0 },
            }}
            transition={{ duration: 0.2 }}
          >
            <ChevronRight className="h-4 w-4" />
          </motion.div>
          <span className="font-medium">{part.name}</span>
        </div>
        <Badge variant="secondary">{part.exhibit_ids.length} exhibits</Badge>
      </motion.button>
      <AnimatePresence initial={false}>
        {isOpen && (
          <motion.div
            key="content"
            initial="collapsed"
            animate="open"
            exit="collapsed"
            variants={{
              open: { opacity: 1, height: "auto" },
              collapsed: { opacity: 0, height: 0 },
            }}
            transition={{ duration: 0.3, ease: [0.04, 0.62, 0.23, 0.98] }}
          >
            <div className="p-4 space-y-4">
              <div className="flex justify-between items-center">
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <Button variant="outline" size="sm" asChild>
                        <a
                          href={part.link}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="flex items-center"
                        >
                          <ExternalLink className="mr-2 h-4 w-4" />
                          View Part Details
                        </a>
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>Open part link in new tab</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
                <Button variant="outline" size="sm">
                  <Plus className="w-4 h-4 mr-2" /> Add Note
                </Button>
              </div>
              {part.notes.length > 0 && (
                <div className="space-y-2">
                  <h4 className="font-semibold text-sm">Notes:</h4>
                  <ScrollArea className="h-40 w-full rounded-md border p-4">
                    <ul className="space-y-2">
                      {part.notes.map((note: Note, index: number) => (
                        <li key={index} className="text-sm">
                          <span className="font-medium">{note.timestamp}:</span>{" "}
                          {note.note}
                        </li>
                      ))}
                    </ul>
                  </ScrollArea>
                </div>
              )}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
}

export { PartsButton };
