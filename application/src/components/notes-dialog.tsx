import { useState } from "react";
import { useForm } from "react-hook-form";
import { format } from "date-fns";
import { StickyNote, Loader2, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import useCreatePartNote from "@/hooks/data/mutations/parts/useCreatePartNote";
import useDeletePartNote from "@/hooks/data/mutations/parts/useDeletePartNote";
import { Note } from "@/types";

interface NotesDialogProps {
  partId: string;
  notes: Note[];
}

export function NotesDialog({ partId, notes }: NotesDialogProps) {
  const [isOpen, setIsOpen] = useState(false);
  const { register, handleSubmit, reset } = useForm<{ message: string }>();
  const createNote = useCreatePartNote();
  const deleteNote = useDeletePartNote();

  const onSubmit = async (data: { message: string }) => {
    try {
      await createNote.mutateAsync({
        partId,
        note: { message: data.message },
      });
      reset();
    } catch (error) {
      console.error("Error creating note:", error);
    }
  };

  const handleDelete = async (noteId: string) => {
    try {
      await deleteNote.mutateAsync({ partId, noteId });
    } catch (error) {
      console.error("Error deleting note:", error);
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button variant="ghost" size="sm">
          <StickyNote className="w-4 h-4 mr-2" />
          {notes.length}
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>Notes</DialogTitle>
        </DialogHeader>
        <div className="mt-4 space-y-4">
          <form onSubmit={handleSubmit(onSubmit)} className="flex space-x-2">
            <Input
              {...register("message", { required: true })}
              placeholder="Add a new note..."
              className="flex-grow"
            />
            <Button type="submit" disabled={createNote.isPending}>
              {createNote.isPending ? (
                <Loader2 className="w-4 h-4 animate-spin" />
              ) : (
                "Add"
              )}
            </Button>
          </form>
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
                        disabled={deleteNote.isPending}
                      >
                        {deleteNote.isPending ? (
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
