import { useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import * as z from "zod";
import {
  Form,
  FormField,
  FormItem,
  FormLabel,
  FormDescription,
  FormMessage,
  FormControl,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useQueryClient } from "@tanstack/react-query";
import useCreatePart from "@/hooks/data/mutations/parts/useCreatePart";
import type { NewPart } from "@/hooks/data/mutations/parts/useCreatePart";

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
  onCancel: () => void;
  exhibitId?: string;
}

export function CreatePartForm({
  onSuccess,
  onCancel,
  exhibitId,
}: CreatePartFormProps) {
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

      queryClient.invalidateQueries({ queryKey: ["parts", exhibitId] });
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
      onSuccess();
      form.reset();
    } catch (error) {
      console.error("Failed to create part:", error);
    } finally {
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
        <div className="flex justify-end space-x-2">
          <Button type="button" variant="outline" onClick={onCancel}>
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting ? "Creating..." : "Create Part"}
          </Button>
        </div>
      </form>
    </Form>
  );
}
