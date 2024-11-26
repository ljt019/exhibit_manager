import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api";

import type { UserProfile } from "@/types";

export async function getUserProfile(): Promise<UserProfile> {
  const userProfile = await invoke<UserProfile>("get_user_info");
  return userProfile;
}

export function useGetUserProfile() {
  return useQuery<UserProfile, Error>({
    queryKey: ["profile-info"],
    queryFn: getUserProfile,
  });
}
