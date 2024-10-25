import { invoke } from "@tauri-apps/api";

export function sign_in() {
  return invoke("sign_in");
}
