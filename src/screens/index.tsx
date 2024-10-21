import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

export function Index() {
  const navigate = useNavigate();

  useEffect(() => {
    let unlisten: UnlistenFn;

    const setupListener = async () => {
      unlisten = await listen("sign_in_complete", () => {
        navigate("/dashboard");
      });
    };
    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, [navigate]);

  async function signIn() {
    await invoke("sign_in");
  }

  return (
    <div className="h-screen flex flex-col justify-center items-center">
      <Button onClick={signIn}>Sign In with Google</Button>
    </div>
  );
}
