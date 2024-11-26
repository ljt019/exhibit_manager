import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { Exhibit } from "@/types";
import { axiosInstance } from "@/api/axiosInstance";

async function createExhibit(exhibit: Exhibit) {
  exhibit.part_ids = [];
  exhibit.notes = [];
  axiosInstance.post("/exhibits", exhibit);
}

export default function useCreateExhibit() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["createExhibit"],
    mutationFn: createExhibit,
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
