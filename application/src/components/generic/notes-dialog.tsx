import { useState } from "react";
import { compareDesc, format } from "date-fns";
import {
  StickyNote,
  Loader2,
  Trash2,
  Calendar,
  User,
  Plus,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardHeader,
  CardFooter,
} from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import type { Note } from "@/types";
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
    note: { submitter: string; message: string };
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
          variant="outline"
          size="sm"
          className="transition-all duration-200 hover:bg-secondary"
        >
          <StickyNote className="w-4 h-4 mr-2" />
          <span>Notes</span>
          <Badge variant="secondary" className="ml-2">
            {notes.length}
          </Badge>
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle className="text-2xl font-semibold">
            {name || `${type.charAt(0).toUpperCase() + type.slice(1)} Notes`}
          </DialogTitle>
        </DialogHeader>
        <Separator className="my-4" />
        <div className="space-y-4">
          <NoteForm
            id={id}
            onSuccess={handleNoteAdded}
            createNote={createNote}
          />
          <NotesList
            notes={notes}
            handleDelete={handleDelete}
            isDeleting={isDeleting}
          />
        </div>
      </DialogContent>
    </Dialog>
  );
}

interface NotesListProps {
  notes: Note[];
  handleDelete: (noteId: string) => void;
  isDeleting: boolean;
}

function NotesList({ notes, handleDelete, isDeleting }: NotesListProps) {
  return (
    <ScrollArea className="h-[50vh] pr-4">
      {notes.length === 0 ? (
        <div className="flex flex-col items-center justify-center h-full text-center">
          <StickyNote className="w-12 h-12 text-muted-foreground mb-2" />
          <p className="text-muted-foreground">No notes yet</p>
          <p className="text-sm text-muted-foreground">
            Click the "Add Note" button to create one
          </p>
        </div>
      ) : (
        <div className="space-y-4">
          {notes
            .sort((a, b) =>
              compareDesc(
                new Date(a.timestamp.date + " " + a.timestamp.time),
                new Date(b.timestamp.date + " " + b.timestamp.time)
              )
            )
            .map((note) => (
              <NoteCard
                key={note.id}
                note={note}
                handleDelete={handleDelete}
                isDeleting={isDeleting}
              />
            ))}
        </div>
      )}
    </ScrollArea>
  );
}

interface NoteCardProps {
  note: Note;
  handleDelete: (noteId: string) => void;
  isDeleting: boolean;
}

function NoteCard({ note, handleDelete, isDeleting }: NoteCardProps) {
  const [showConfirm, setShowConfirm] = useState(false);
  const formattedDate = format(
    new Date(note.timestamp.date + " " + note.timestamp.time),
    "PPpp"
  );

  const onDelete = () => {
    setShowConfirm(true);
  };

  const confirmDelete = () => {
    handleDelete(note.id);
    setShowConfirm(false);
  };

  const cancelDelete = () => {
    setShowConfirm(false);
  };

  return (
    <Card className="w-full hover:shadow-md transition-shadow duration-300">
      <CardHeader className="pb-2">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <User className="w-4 h-4 text-primary" />
            <p className="text-sm font-medium">{note.submitter}</p>
          </div>
          <div className="flex items-center space-x-2">
            <Calendar className="w-4 h-4 text-muted-foreground" />
            <p className="text-xs text-muted-foreground">{formattedDate}</p>
          </div>
        </div>
      </CardHeader>
      <CardContent>
        <p className="text-sm leading-relaxed whitespace-pre-wrap">
          {note.message}
        </p>
      </CardContent>
      <CardFooter className="pt-2">
        <div className="flex justify-end w-full relative h-10 overflow-hidden">
          <div
            className={`
              absolute inset-y-0 right-0 flex items-center
              transition-all duration-500 ease-in-out
              ${
                showConfirm
                  ? "translate-y-full opacity-0"
                  : "translate-y-0 opacity-100"
              }
            `}
          >
            <Button
              variant="ghost"
              size="sm"
              onClick={onDelete}
              className="text-muted-foreground hover:text-destructive transition-colors duration-200"
            >
              <Trash2 className="w-4 h-4" />
              <span className="sr-only">Delete note</span>
            </Button>
          </div>
          <div
            className={`
              absolute inset-y-0 right-0 flex items-center space-x-2
              transition-all duration-500 ease-in-out
              ${
                showConfirm
                  ? "translate-y-0 opacity-100"
                  : "-translate-y-full opacity-0"
              }
            `}
          >
            <Button
              variant="destructive"
              size="sm"
              onClick={confirmDelete}
              disabled={isDeleting}
            >
              {isDeleting ? (
                <Loader2 className="w-4 h-4 animate-spin" />
              ) : (
                "Confirm"
              )}
            </Button>
            <Button variant="outline" size="sm" onClick={cancelDelete}>
              Cancel
            </Button>
          </div>
        </div>
      </CardFooter>
    </Card>
  );
}
