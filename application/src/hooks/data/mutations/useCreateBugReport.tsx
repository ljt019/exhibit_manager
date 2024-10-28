import { useMutation } from "@tanstack/react-query";
import { useGetUserProfile } from "@/hooks/data/queries/useGetProfileInfo";
import { axiosInstance } from "@/api/axiosInstance";

interface ReportBugPayload {
  name?: string;
  title: string;
  description: string;
}

async function reportBug(bug_report: ReportBugPayload) {
  await axiosInstance.post("http://localhost:3030/report-bug", bug_report);
}

export default function useCreateBugReport() {
  const { data: profile, isLoading, isError } = useGetUserProfile();

  if (isLoading || isError || !profile) {
    return {
      mutate: () => {},
      isPending: true,
      isError,
      isSuccess: false,
    };
  }

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

      return await reportBug(enrichedBugReport);
    },
  });
}
