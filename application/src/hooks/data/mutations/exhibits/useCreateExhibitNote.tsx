import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

async function createExhibitNote(newExhibitNoteRequest: NewExhibitNoteRequest) {
  const response = await axiosInstance.post(
    `/exhibits/${newExhibitNoteRequest.exhibitId}/notes`,
    newExhibitNoteRequest.note
  );

  if (response.status !== 200) {
    throw new Error("Failed to create note");
  }

  return response;
}

type NewExhibitNote = {
  submitter: string;
  message: string;
};

export interface NewExhibitNoteRequest {
  exhibitId: string;
  note: NewExhibitNote;
}

export default function useCreateExhibitNote() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["createExhibitNote"],
    mutationFn: (newExhibitNoteRequest: NewExhibitNoteRequest) =>
      toast.promise(createExhibitNote(newExhibitNoteRequest), {
        loading: "Creating note...",
        success: "Note created successfully",
        error: "Failed to create note",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
    },
  });
}
