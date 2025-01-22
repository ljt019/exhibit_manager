import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

export type ExhibitStatus = "Operational" | "Needs Repair" | "Out of Service";

export interface ChangeExhibitStatusRequest {
  exhibitId: string;
  newStatus: ExhibitStatus;
}

async function changeExhibitStatus(request: ChangeExhibitStatusRequest) {
  const response = await axiosInstance.post(
    `/exhibits/${request.exhibitId}/status`,
    {
      new_status: request.newStatus,
    }
  );

  if (response.status !== 200) {
    throw new Error("Failed to change exhibit status");
  }

  return response;
}

export default function useChangeExhibitStatus() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ["changeExhibitStatus"],
    mutationFn: (request: ChangeExhibitStatusRequest) =>
      toast.promise(changeExhibitStatus(request), {
        loading: "Changing exhibit status...",
        success: "Exhibit status changed successfully",
        error: "Failed to change exhibit status",
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
    },
  });
}
