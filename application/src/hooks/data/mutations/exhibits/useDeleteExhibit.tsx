import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

async function deleteExhibit(exhibit_id: string) {
  const response = await axiosInstance.delete("/exhibits/" + exhibit_id);

  if (response.status !== 204) {
    throw new Error("Failed to delete exhibit");
  }

  return response;
}

export default function useDeleteExhibit() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["deleteExhibit"],
    mutationFn: (exhibit_id: string) =>
      toast.promise(deleteExhibit(exhibit_id), {
        loading: "Deleting exhibit...",
        success: "Exhibit deleted successfully",
        error: "Failed to delete exhibit",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
    },
  });
}
