import { useState } from "react";
import { format } from "date-fns";
import { StickyNote, Loader2, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Note } from "@/types";
import { NoteForm } from "@/components/forms/create-note-form";

interface NotesDialogProps<T extends "part" | "exhibit"> {
  id: string;
  type: T;
  name?: string;
  notes: Note[];
  onNoteAdded?: () => void;
  onNoteDeleted?: () => void;
  createNote: (data: {
    id: string;
    note: { message: string };
  }) => Promise<void>;
  deleteNote: (data: { id: string; noteId: string }) => Promise<void>;
}

export function NotesDialog<T extends "part" | "exhibit">({
  id,
  type,
  name,
  notes,
  onNoteAdded,
  onNoteDeleted,
  createNote,
  deleteNote,
}: NotesDialogProps<T>) {
  const [isOpen, setIsOpen] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const handleDelete = async (noteId: string) => {
    setIsDeleting(true);
    try {
      await deleteNote({ id, noteId });
      if (onNoteDeleted) {
        onNoteDeleted();
      }
    } catch (error) {
      console.error("Error deleting note:", error);
    } finally {
      setIsDeleting(false);
    }
  };

  const handleNoteAdded = () => {
    if (onNoteAdded) {
      onNoteAdded();
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button
          variant="ghost"
          size="sm"
          className="transition-all duration-200"
        >
          <StickyNote className="w-4 h-4 mr-2" />
          {notes.length}
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>
            {name || `${type.charAt(0).toUpperCase() + type.slice(1)} Notes`}
          </DialogTitle>
        </DialogHeader>
        <div className="mt-4 space-y-4">
          <NoteForm
            id={id as string}
            onSuccess={handleNoteAdded}
            createNote={createNote}
          />
          <ScrollArea className="h-[50vh]">
            {notes.length === 0 ? (
              <p className="text-center text-muted-foreground">No notes yet</p>
            ) : (
              <div className="space-y-4">
                {notes.map((note) => (
                  <Card key={note.id}>
                    <CardContent className="p-4 flex justify-between items-start">
                      <div>
                        <p className="text-sm text-muted-foreground mb-1">
                          {format(
                            new Date(
                              note.timestamp.date + " " + note.timestamp.time
                            ),
                            "PPpp"
                          )}
                        </p>
                        <p>{note.message}</p>
                      </div>
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => handleDelete(note.id)}
                        disabled={isDeleting}
                      >
                        {isDeleting ? (
                          <Loader2 className="w-4 h-4 animate-spin" />
                        ) : (
                          <Trash2 className="w-4 h-4" />
                        )}
                      </Button>
                    </CardContent>
                  </Card>
                ))}
              </div>
            )}
          </ScrollArea>
        </div>
      </DialogContent>
    </Dialog>
  );
}
