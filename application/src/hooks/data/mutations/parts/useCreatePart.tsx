import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

export interface NewPart {
  name: string;
  link: string;
  exhibit_ids?: Array<string>;
  notes?: Array<{ timestamp: string; note: string }>;
}

async function createPart(part: NewPart) {
  const response = await axiosInstance.post("/parts", part);

  if (response.status !== 200) {
    throw new Error("Failed to create part");
  }

  return response;
}

export default function useCreatePart() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["createPart"],
    mutationFn: (part: NewPart) =>
      toast.promise(createPart(part), {
        loading: "Creating part...",
        success: "Part created successfully",
        error: "Failed to create part",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["parts"] });
    },
  });
}
