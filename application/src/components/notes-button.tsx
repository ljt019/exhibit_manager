import { NotesDialog } from "@/components/NotesDialog";
import useCreateExhibitNote from "@/hooks/data/mutations/exhibits/useCreateExhibitNote";
import useDeleteExhibitNote from "@/hooks/data/mutations/exhibits/useDeleteExhibitNote";
import type { Note } from "@/types";

interface ExhibitNotesButtonProps {
  exhibitId: string;
  name: string;
  notes: Array<Note>;
  onNoteAdded?: () => void;
  onNoteDeleted?: () => void;
}

export function ExhibitNotesButton({
  exhibitId,
  name,
  notes,
  onNoteAdded,
  onNoteDeleted,
}: ExhibitNotesButtonProps) {
  // We'll need to create a custom NoteForm for exhibits
  const ExhibitNoteForm = ({
    partId,
    onSuccess,
  }: {
    partId: string;
    onSuccess: () => void;
  }) => {
    const createNote = useCreateExhibitNote();

    const onSubmit = async (data: { message: string }) => {
      try {
        await createNote.mutateAsync({
          exhibitId: partId, // Use exhibitId instead of partId
          note: { message: data.message },
        });
        onSuccess();
      } catch (error) {
        console.error("Error creating note:", error);
      }
    };

    return <NoteForm partId={partId} onSuccess={onSuccess} />;
  };

  const deleteNote = useDeleteExhibitNote();

  const handleDelete = async (noteId: string) => {
    try {
      await deleteNote.mutateAsync({ exhibitId, noteId });
      if (onNoteDeleted) {
        onNoteDeleted();
      }
    } catch (error) {
      console.error("Error deleting note:", error);
    }
  };

  return (
    <NotesDialog
      partId={exhibitId}
      name={name}
      notes={notes}
      onNoteAdded={onNoteAdded}
      onNoteDeleted={onNoteDeleted}
    />
  );
}
