import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "react-router-dom";
import useListen from "@/hooks/useListen";

export default function SignIn() {
  const navigate = useNavigate();

  useListen({
    event: "sign_in_complete",
    callback: () => {
      navigate("/exhibits");
    },
  });

  async function signIn() {
    await invoke("sign_in");
  }

  return (
    <div className="h-screen flex flex-col justify-center items-center">
      <Button onClick={signIn}>Sign In with Google</Button>
    </div>
  );
}
