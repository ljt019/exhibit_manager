import { useMutation } from "@tanstack/react-query";
import { useGetUserProfile } from "@/hooks/data/queries/useGetProfileInfo";
import { axiosInstance } from "@/api/axiosInstance";
import { toast } from "react-hot-toast";

interface ReportBugPayload {
  name?: string;
  title: string;
  description: string;
}

async function reportBug(bug_report: ReportBugPayload) {
  const response = await axiosInstance.post(
    "http://localhost:3030/report-bug",
    bug_report
  );

  if (response.status !== 200) {
    throw new Error("Failed to report bug");
  }

  return response;
}

export default function useCreateBugReport() {
  const { data: profile, isLoading, isError } = useGetUserProfile();

  return useMutation({
    mutationKey: ["reportBug"],
    mutationFn: async (bug_report: ReportBugPayload) => {
      if (isLoading) {
        throw new Error("Loading user profile. Please wait.");
      }

      if (isError || !profile) {
        throw new Error(
          "Failed to load user profile. Cannot submit bug report."
        );
      }

      // Attach user's first name to the bug report
      const enrichedBugReport = {
        ...bug_report,
        name: profile.given_name, // Assuming 'given_name' is available
      };

      return toast.promise(reportBug(enrichedBugReport), {
        loading: "Submitting bug report...",
        success: "Bug reported successfully",
        error: "Failed to report bug",
      });
    },
  });
}
