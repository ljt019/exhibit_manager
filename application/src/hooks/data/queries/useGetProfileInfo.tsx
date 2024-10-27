import { useQuery } from "@tanstack/react-query";

import { getUserProfile } from "@/api/get_user_profile";
import type { UserProfile } from "@/types";

export function useGetUserProfile() {
  return useQuery<UserProfile, Error>({
    queryKey: ["profile-info"],
    queryFn: getUserProfile,
  });
}
