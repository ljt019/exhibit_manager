import { invoke } from "@tauri-apps/api";
import { UserProfile } from "@/types/types";

export async function getUserProfile(): Promise<UserProfile> {
  const userProfile = await invoke<UserProfile>("get_user_info");
  return userProfile;
}
