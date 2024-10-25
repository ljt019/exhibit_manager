import { invoke } from "@tauri-apps/api";

export interface UserProfile {
  id: string;
  name: string;
  given_name: string;
  family_name: string;
  picture: string | null;
}

export async function getUserProfile(): Promise<UserProfile> {
  const userProfile = await invoke<UserProfile>("get_user_info");
  return userProfile;
}
