import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

async function deleteExhibitNote(
  deleteExhibitNoteRequest: DeleteExhibitNoteRequest
) {
  let response = await axiosInstance.delete(
    `/exhibits/${deleteExhibitNoteRequest.exhibitId}/notes/${deleteExhibitNoteRequest.noteId}`
  );

  if (response.status !== 204) {
    throw new Error("Failed to delete note");
  }

  return response;
}

interface DeleteExhibitNoteRequest {
  exhibitId: string;
  noteId: string;
}

export default function useDeleteExhibitNote() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["deleteExhibitNote"],
    mutationFn: (deleteExhibitNoteRequest: DeleteExhibitNoteRequest) =>
      toast.promise(deleteExhibitNote(deleteExhibitNoteRequest), {
        loading: "Deleting note...",
        success: "Note deleted successfully",
        error: "Failed to delete note",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
    },
  });
}
