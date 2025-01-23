import { useState } from "react";
import { useForm } from "react-hook-form";
import { Loader2, Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useGetUserProfile } from "@/hooks/data/queries/useGetProfileInfo";

interface NoteFormProps {
  id: string;
  onSuccess: () => void;
  createNote: (data: {
    id: string;
    note: { submitter: string; message: string };
  }) => Promise<void>;
}

export function NoteForm({ id, onSuccess, createNote }: NoteFormProps) {
  const { data: userData, isLoading, isError } = useGetUserProfile();
  const { register, handleSubmit, reset } = useForm<{ message: string }>();
  const [isSubmitting, setIsSubmitting] = useState(false);

  const onSubmit = async (formData: { message: string }) => {
    if (isLoading || isError || !userData) {
      console.error("User data not available");
      return;
    }

    setIsSubmitting(true);
    try {
      await createNote({
        id,
        note: { submitter: userData.name, message: formData.message },
      });
      reset();
      onSuccess();
    } catch (error) {
      console.error("Error creating note:", error);
    } finally {
      setIsSubmitting(false);
    }
  };

  if (isLoading) {
    return <div>Loading user data...</div>;
  }

  if (isError) {
    return <div>Error loading user data. Please try again.</div>;
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="flex space-x-2">
      <Input
        {...register("message", { required: true })}
        placeholder="Add a new note..."
        className="flex-grow"
      />
      <Button
        type="submit"
        disabled={isSubmitting || !userData}
        variant="outline"
      >
        {isSubmitting ? (
          <Loader2 className="w-4 h-4 animate-spin" />
        ) : (
          <Plus className="h-4 w-4" />
        )}
      </Button>
    </form>
  );
}
