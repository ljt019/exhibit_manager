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
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useQueryClient } from "@tanstack/react-query";
import useEditPart from "@/hooks/data/mutations/parts/useEditPart";
import type { Part } from "@/types";

const formSchema = z.object({
  name: z.string().min(2, {
    message: "Name must be at least 2 characters.",
  }),
  link: z.string().url({
    message: "Please enter a valid URL.",
  }),
});

interface EditPartFormProps {
  part: Part;
  onSuccess: () => void;
}

export function EditPartForm({ part, onSuccess }: EditPartFormProps) {
  const editPartMutation = useEditPart();
  const queryClient = useQueryClient();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: part.name,
      link: part.link,
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    try {
      await editPartMutation.mutateAsync({
        id: part.id,
        payload: values,
      });
      toast({
        title: "Part updated",
        description: "Your part has been successfully updated.",
      });
      onSuccess();
    } catch (error) {
      console.error("Failed to update part:", error);
      toast({
        title: "Error",
        description: "Failed to update part. Please try again.",
        variant: "destructive",
      });
    } finally {
      queryClient.invalidateQueries({ queryKey: ["parts"] });
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
                <Input {...field} />
              </FormControl>
              <FormDescription>The name of the part.</FormDescription>
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
                <Input {...field} type="url" />
              </FormControl>
              <FormDescription>The URL link for the part.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit" disabled={editPartMutation.isPending}>
          {editPartMutation.isPending ? "Updating..." : "Update Part"}
        </Button>
      </form>
    </Form>
  );
}
