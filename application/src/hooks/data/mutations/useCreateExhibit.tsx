import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { Exhibit } from "@/types";
import axios from "axios";

async function createExhibit(exhibit: Exhibit) {
  exhibit.part_ids = [];
  exhibit.notes = [];
  axios.post("http://localhost:3030/exhibits", exhibit);
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
