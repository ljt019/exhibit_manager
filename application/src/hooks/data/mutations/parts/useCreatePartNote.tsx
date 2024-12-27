import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

async function createPartNote(newPartNoteRequest: NewPartNoteRequest) {
  const response = await axiosInstance.post(
    `/parts/${newPartNoteRequest.partId}/notes`,
    newPartNoteRequest.note
  );

  if (response.status !== 200) {
    throw new Error("Failed to create note");
  }

  return response;
}

type NewPartNote = {
  message: string;
};

export interface NewPartNoteRequest {
  partId: string;
  note: NewPartNote;
}

export default function useCreatePartNote() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["createPartNote"],
    mutationFn: (newPartNoteRequest: NewPartNoteRequest) =>
      toast.promise(createPartNote(newPartNoteRequest), {
        loading: "Creating note...",
        success: "Note created successfully",
        error: "Failed to create note",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["parts"] });
    },
  });
}
