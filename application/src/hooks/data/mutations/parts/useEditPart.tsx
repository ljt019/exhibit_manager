interface UpdatePartPayload {
  name: string;
  link: string;
  exhibitIds: number[];
}

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

async function editPart(id: string, updatePartPayload: UpdatePartPayload) {
  const response = await axiosInstance.put(`/parts/${id}`, updatePartPayload);

  if (response.status !== 200) {
    throw new Error("Failed to update part");
  }

  return response;
}

export default function useEditPart() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["editPart"],
    mutationFn: ({ id, payload }: { id: string; payload: UpdatePartPayload }) =>
      toast.promise(editPart(id, payload), {
        loading: "Editing part...",
        success: "Part edited successfully",
        error: "Failed to edit part",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["parts"] });
    },
  });
}
