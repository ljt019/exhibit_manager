import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

async function deletePartNote(deletePartNoteRequest: DeletePartNoteRequest) {
  let response = await axiosInstance.delete(
    `/parts/${deletePartNoteRequest.partId}/notes/${deletePartNoteRequest.noteId}`
  );

  if (response.status !== 204) {
    throw new Error("Failed to delete note");
  }

  return response;
}

interface DeletePartNoteRequest {
  partId: string;
  noteId: string;
}

export default function useDeletePartNote() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["deletePartNote"],
    mutationFn: (deletePartNoteRequest: DeletePartNoteRequest) =>
      toast.promise(deletePartNote(deletePartNoteRequest), {
        loading: "Deleting note...",
        success: "Note deleted successfully",
        error: "Failed to delete note",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["parts"] });
    },
  });
}
