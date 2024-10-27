import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { Part } from "@/types";
import axios from "axios";

async function createPart(part: Part) {
  part.notes = [];
  axios.post("http://localhost:3030/parts", part);
}

export default function useCreatePart() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["createPart"],
    mutationFn: createPart,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["parts"] });
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ["parts"] });
    },
    onMutate: () => {
      queryClient.invalidateQueries({ queryKey: ["parts"] });
    },
  });
}
