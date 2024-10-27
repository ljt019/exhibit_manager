import { useMutation, useQueryClient } from "@tanstack/react-query";
import axios from "axios";

export interface NewPart {
  name: string;
  link: string;
  exhibit_ids?: Array<string>;
  notes?: Array<{ timestamp: string; note: string }>;
}

async function createPart(part: NewPart) {
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
