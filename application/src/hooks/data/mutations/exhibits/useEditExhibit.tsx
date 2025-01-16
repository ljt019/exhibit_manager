interface UpdateExhibitPayload {
  name?: string;
  cluster?: string;
  location?: string;
  description?: string;
  image_url?: string;
}

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

async function editExhibit(
  id: string,
  updateExhibitPayload: UpdateExhibitPayload
) {
  const response = await axiosInstance.put(
    `/exhibits/${id}`,
    updateExhibitPayload
  );

  if (response.status !== 200) {
    throw new Error("Failed to create exhibit");
  }

  return response;
}

export default function useEditExhibit() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["editExhibit"],
    mutationFn: ({
      id,
      payload,
    }: {
      id: string;
      payload: UpdateExhibitPayload;
    }) =>
      toast.promise(editExhibit(id, payload), {
        loading: "Editing exhibit...",
        success: "Exhibit edited successfully",
        error: "Failed to edit exhibit",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
    },
  });
}
