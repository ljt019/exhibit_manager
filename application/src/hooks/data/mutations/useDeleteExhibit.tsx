import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";

async function deleteExhibit(exhibit_id: string) {
  axiosInstance.delete("/exhibits/" + exhibit_id);
}

export default function useDeleteExhibit() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["deleteExhibit"],
    mutationFn: deleteExhibit,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
    },
    onMutate: () => {
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
    },
  });
}
