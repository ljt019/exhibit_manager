import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

interface AddExistingPartPayload {
  exhibitId: string;
  partId: string;
}

async function addExistingPart({ exhibitId, partId }: AddExistingPartPayload) {
  const response = await axiosInstance.post(`/exhibits/${exhibitId}/add_part`, {
    part_id: partId,
  });

  if (response.status !== 200) {
    throw new Error("Failed to add part to exhibit");
  }

  return response;
}

export default function useAddExistingPart() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["addExistingPart"],
    mutationFn: addExistingPart,
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
      queryClient.invalidateQueries({
        queryKey: ["parts", variables.exhibitId],
      });
      toast.success("Part added to exhibit successfully");
    },
    onError: (error) => {
      console.error("Failed to add part to exhibit:", error);
      toast.error("Failed to add part to exhibit");
    },
  });
}
