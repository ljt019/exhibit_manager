import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { Exhibit } from "@/types";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

async function createExhibit(exhibit: Exhibit) {
  exhibit.part_ids = [];
  exhibit.notes = [];
  const response = await axiosInstance.post("/exhibits", exhibit);

  if (response.status !== 200) {
    throw new Error("Failed to create exhibit");
  }

  return response;
}

export default function useCreateExhibit() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["createExhibit"],
    mutationFn: (exhibit: Exhibit) =>
      toast.promise(createExhibit(exhibit), {
        loading: "Creating exhibit...",
        success: "Exhibit created successfully",
        error: "Failed to create exhibit",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
    },
  });
}
