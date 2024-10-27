import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
  SelectValue,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";
import { useState } from "react";
import useCreateExhibit from "@/hooks/data/useCreateExhibit";
import type { Exhibit } from "@/components/exhibit-card";

export function CreateExhibitDialog() {
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  const handleDialogClose = () => {
    setIsDialogOpen(false);
  };

  return (
    <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
      <DialogTrigger asChild>
        <Button variant="outline">
          <Plus className="w-4 h-4" />
          <span className="sr-only">Create New Exhibit</span>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create New Exhibit</DialogTitle>
          <DialogDescription>
            Fill in the details for the new exhibit.
          </DialogDescription>
        </DialogHeader>
        <CreateExhibitForm onSuccess={handleDialogClose} />
      </DialogContent>
    </Dialog>
  );
}

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
  cluster: z.string().min(2, {
    message: "Cluster must be at least 2 characters.",
  }),
  location: z.string().min(2, {
    message: "Location must be at least 2 characters.",
  }),
  status: z.enum(["operational", "needs repair", "out of service"]),
  image_url: z.string().url().optional(),
});

interface CreateExhibitFormProps {
  onSuccess: () => void;
}

export function CreateExhibitForm({ onSuccess }: CreateExhibitFormProps) {
  const createExhibitMutation = useCreateExhibit();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const queryClient = useQueryClient();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
      cluster: "",
      location: "",
      status: "operational",
      image_url: "",
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setIsSubmitting(true);
    try {
      await createExhibitMutation.mutateAsync(values as Exhibit);
      toast({
        title: "Exhibit created",
        description: "Your new exhibit has been successfully created.",
      });
      onSuccess();
      form.reset();
    } catch (error) {
      console.error("Failed to create exhibit:", error);
      toast({
        title: "Error",
        description: "Failed to create exhibit. Please try again.",
        variant: "destructive",
      });
    } finally {
      queryClient.invalidateQueries({ queryKey: ["exhibits"] });
      setIsSubmitting(false);
    }
  }

  const handleImageUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onloadend = () => {
        form.setValue("image_url", reader.result as string);
      };
      reader.readAsDataURL(file);
    }
  };

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
                <Input placeholder="Enter exhibit name" {...field} />
              </FormControl>
              <FormDescription>The name of the new exhibit.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="cluster"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Cluster</FormLabel>
              <FormControl>
                <Input placeholder="Enter cluster name" {...field} />
              </FormControl>
              <FormDescription>
                The cluster this exhibit belongs to.
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="location"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Location</FormLabel>
              <FormControl>
                <Input placeholder="Enter exhibit location" {...field} />
              </FormControl>
              <FormDescription>
                The specific location of the exhibit.
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="status"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Status</FormLabel>
              <Select onValueChange={field.onChange} defaultValue={field.value}>
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder="Select exhibit status" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  <SelectItem value="operational">Operational</SelectItem>
                  <SelectItem value="needs repair">Needs Repair</SelectItem>
                  <SelectItem value="out of service">Out of Service</SelectItem>
                </SelectContent>
              </Select>
              <FormDescription>
                The current operational status of the exhibit.
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="image_url"
          // @ts-ignore
          render={({ field }) => (
            <FormItem>
              <FormLabel>Image</FormLabel>
              <FormControl>
                <Input
                  type="file"
                  accept="image/*"
                  onChange={handleImageUpload}
                />
              </FormControl>
              <FormDescription>
                Upload an image for the exhibit (optional).
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit" disabled={isSubmitting}>
          {isSubmitting ? "Creating..." : "Create Exhibit"}
        </Button>
      </form>
    </Form>
  );
}
