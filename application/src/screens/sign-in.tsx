import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "react-router-dom";
import useListen from "@/hooks/useListen";
import { useEffect, useState } from "react";
import { FcGoogle } from "react-icons/fc";
import { motion } from "framer-motion";
import useGetRandomExhibit from "@/hooks/useGetRandomExhibit";

export default function SignIn() {
  const navigate = useNavigate();
  const [imageLoaded, setImageLoaded] = useState(false);

  const { data: randomExhibit, isLoading, isError } = useGetRandomExhibit();

  useListen({
    event: "sign_in_complete",
    callback: () => {
      navigate("/exhibits");
    },
  });

  async function signIn() {
    await invoke("sign_in");
  }

  async function checkAlreadySignedIn() {
    let isSignedIn = await invoke("check_if_signed_in");
    if (isSignedIn) {
      navigate("/exhibits");
    }
  }

  useEffect(() => {
    checkAlreadySignedIn();
  }, []);

  if (isError) {
    return <div>Error</div>;
  }

  return (
    <div className="min-h-screen flex bg-background">
      {/* Left side - Sign In */}
      <motion.div
        className="w-full md:w-1/2 flex flex-col justify-center items-center p-8 md:p-16"
        initial={{ opacity: 0, x: -50 }}
        animate={{ opacity: 1, x: 0 }}
        transition={{ duration: 0.5 }}
      >
        <div className="w-full max-w-md space-y-8">
          <div className="text-center">
            <motion.h1
              className="text-4xl font-bold tracking-tight"
              initial={{ opacity: 0, y: -20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.2, duration: 0.5 }}
            >
              Exhibit Manager
            </motion.h1>
            <motion.p
              className="mt-2 text-lg text-muted-foreground"
              initial={{ opacity: 0, y: -20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.3, duration: 0.5 }}
            >
              Curate, Manage, Inspire
            </motion.p>
          </div>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.4, duration: 0.5 }}
          >
            <Button
              onClick={signIn}
              variant="outline"
              className="w-full h-14 text-lg font-semibold transition-all duration-300 ease-in-out transform hover:scale-105 hover:shadow-lg"
            >
              <FcGoogle className="w-6 h-6 mr-2" />
              Sign in with Google
            </Button>
          </motion.div>
        </div>
      </motion.div>

      {/* Right side - Exhibit Image */}
      <motion.div
        className="hidden md:block w-1/2 bg-black relative overflow-hidden"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.5 }}
      >
        {!isLoading && (
          <motion.img
            src={randomExhibit?.image_url}
            alt="Featured Exhibit"
            className="absolute inset-0 w-full h-full object-cover"
            onLoad={() => setImageLoaded(true)}
            initial={{ scale: 1.1 }}
            animate={{ scale: imageLoaded ? 1 : 1.1 }}
            transition={{ duration: 10, ease: "easeInOut" }}
          />
        )}
        <div className="absolute inset-0 bg-gradient-to-r from-background via-background/50 to-transparent" />
        <motion.div
          className="absolute bottom-0 left-0 p-8 max-w-lg"
          initial={{ opacity: 0, y: 50 }}
          animate={{ opacity: imageLoaded ? 1 : 0, y: imageLoaded ? 0 : 50 }}
          transition={{ delay: 0.5, duration: 0.5 }}
        >
          <h2 className="text-3xl font-bold text-white mb-2">
            {randomExhibit?.name}
          </h2>
        </motion.div>
      </motion.div>
    </div>
  );
}
