import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

async function deletePart(part_id: string) {
  const response = await axiosInstance.delete("/parts/" + part_id);

  if (response.status !== 204) {
    throw new Error("Failed to delete part");
  }

  return response;
}

export default function useDeletePart() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["deletePart"],
    mutationFn: (part_id: string) =>
      toast.promise(deletePart(part_id), {
        loading: "Deleting part...",
        success: "Part deleted successfully",
        error: "Failed to delete part",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["parts"] });
    },
  });
}
