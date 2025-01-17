import { NotesDialog } from "@/components/generic/notes-dialog";
import useCreateExhibitNote from "@/hooks/data/mutations/exhibits/useCreateExhibitNote";
import useDeleteExhibitNote from "@/hooks/data/mutations/exhibits/useDeleteExhibitNote";
import useCreatePartNote from "@/hooks/data/mutations/parts/useCreatePartNote";
import useDeletePartNote from "@/hooks/data/mutations/parts/useDeletePartNote";
import type { Note } from "@/types";

interface NotesButtonProps {
  id: string;
  type: "exhibit" | "part";
  name: string;
  notes: Array<Note>;
  onNoteAdded?: () => void;
  onNoteDeleted?: () => void;
}

export function NotesButton({
  id,
  type,
  name,
  notes,
  onNoteAdded,
  onNoteDeleted,
}: NotesButtonProps) {
  const createExhibitNoteMutation = useCreateExhibitNote();
  const deleteExhibitNoteMutation = useDeleteExhibitNote();
  const createPartNoteMutation = useCreatePartNote();
  const deletePartNoteMutation = useDeletePartNote();

  const createNote = async (data: {
    id: string;
    note: { message: string };
  }) => {
    if (type === "exhibit") {
      await createExhibitNoteMutation.mutateAsync({
        exhibitId: data.id,
        note: data.note,
      });
    } else {
      await createPartNoteMutation.mutateAsync({
        partId: data.id,
        note: data.note,
      });
    }
  };

  const deleteNote = async (data: { id: string; noteId: string }) => {
    if (type === "exhibit") {
      await deleteExhibitNoteMutation.mutateAsync({
        exhibitId: data.id,
        noteId: data.noteId,
      });
    } else {
      await deletePartNoteMutation.mutateAsync({
        partId: data.id,
        noteId: data.noteId,
      });
    }
  };

  return (
    <NotesDialog
      id={id}
      type={type}
      name={name}
      notes={notes}
      onNoteAdded={onNoteAdded}
      onNoteDeleted={onNoteDeleted}
      createNote={createNote}
      deleteNote={deleteNote}
    />
  );
}
