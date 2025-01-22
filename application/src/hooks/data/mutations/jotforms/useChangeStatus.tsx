import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

export type Status = "InProgress" | "Closed" | "Open" | "Unplanned";

export interface NewStatusRequest {
  jotformId: string;
  status: Status;
}

async function changeStatus(newStatusRequest: NewStatusRequest) {
  const response = await axiosInstance.post(
    `/jotforms/${newStatusRequest.jotformId}/status`,
    {
      new_status: newStatusRequest.status,
    }
  );

  if (response.status !== 200) {
    throw new Error("Failed to change status");
  }

  return response;
}

export default function useChangeStatus() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["changeStatus"],
    mutationFn: (newStatusRequest: NewStatusRequest) =>
      toast.promise(changeStatus(newStatusRequest), {
        loading: "Changing status...",
        success: "Status changed successfully",
        error: "Failed to change status",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["jotforms"] });
    },
  });
}
