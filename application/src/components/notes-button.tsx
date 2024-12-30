import { StickyNote, Trash2, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { ScrollArea } from "@/components/ui/scroll-area";
import type { Note } from "@/types";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { format } from "date-fns";

import { Input } from "@/components/ui/input";
import useCreateExhibit from "@/hooks/data/mutations/exhibits/useCreateExhibitNote";
import useDeleteExhibitNote from "@/hooks/data/mutations/exhibits/useDeleteExhibitNote";

interface NotesButtonProps {
  exhibitId: string;
  name: string;
  notes: Array<Note>;
}

export function NotesButton({ exhibitId, name, notes }: NotesButtonProps) {
  const [isOpen, setIsOpen] = useState(false);
  const { register, handleSubmit } = useForm<{ message: string }>();
  const createNote = useCreateExhibit();
  const deleteNote = useDeleteExhibitNote();

  const onSubmit = async (data: { message: string }) => {
    try {
      await createNote.mutateAsync({
        exhibitId,
        note: { message: data.message },
      });
    } catch (error) {
      console.error("Error creating note:", error);
    }
  };

  const handleDelete = async (noteId: string) => {
    await deleteNote.mutateAsync({ exhibitId, noteId });
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
          <DialogTitle>{name}</DialogTitle>
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
