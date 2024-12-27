import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";
import { useState } from "react";
import useCreatePart from "@/hooks/data/mutations/parts/useCreatePart";
import type { NewPart } from "@/hooks/data/mutations/parts/useCreatePart";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import * as z from "zod";
import { toast } from "@/hooks/use-toast";
import {
  Form,
  FormField,
  FormItem,
  FormLabel,
  FormDescription,
  FormMessage,
  FormControl,
} from "@/components/ui/form";
import { useQueryClient } from "@tanstack/react-query";

const formSchema = z.object({
  name: z.string().min(2, {
    message: "Name must be at least 2 characters.",
  }),
  link: z.string().url({
    message: "Please enter a valid URL.",
  }),
});

interface CreatePartFormProps {
  onSuccess: () => void;
  exhibitId?: string;
}

export function CreatePartDialog({ exhibitId }: { exhibitId?: string }) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  const handleDialogClose = () => {
    setIsDialogOpen(false);
  };

  return (
    <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
      <DialogTrigger asChild>
        <Button variant="outline">
          <Plus className="w-4 h-4" />
          <span className="sr-only">Create New Part</span>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create New Part</DialogTitle>
          <DialogDescription>
            Fill in the details for the new part.
          </DialogDescription>
        </DialogHeader>
        <CreatePartForm onSuccess={handleDialogClose} exhibitId={exhibitId} />
      </DialogContent>
    </Dialog>
  );
}

function CreatePartForm({ onSuccess, exhibitId }: CreatePartFormProps) {
  const createPartMutation = useCreatePart();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const queryClient = useQueryClient();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
      link: "",
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setIsSubmitting(true);
    try {
      const newPart: NewPart = {
        name: values.name,
        link: values.link,
        exhibit_ids: exhibitId ? [exhibitId] : [],
        notes: [],
      };
      await createPartMutation.mutateAsync(newPart);
      toast({
        title: "Part created",
        description: "Your new part has been successfully created.",
      });
      onSuccess();
      form.reset();
    } catch (error) {
      console.error("Failed to create part:", error);
      toast({
        title: "Error",
        description: "Failed to create part. Please try again.",
        variant: "destructive",
      });
    } finally {
      queryClient.invalidateQueries({ queryKey: ["parts"] });
      setIsSubmitting(false);
    }
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <FormField
          control={form.control}
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Name</FormLabel>
              <FormControl>
                <Input placeholder="Enter part name" {...field} />
              </FormControl>
              <FormDescription>The name of the new part.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="link"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Link</FormLabel>
              <FormControl>
                <Input placeholder="Enter part link" {...field} />
              </FormControl>
              <FormDescription>
                A link related to the part (e.g., manufacturer's website).
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit" disabled={isSubmitting}>
          {isSubmitting ? "Creating..." : "Create Part"}
        </Button>
      </form>
    </Form>
  );
}
