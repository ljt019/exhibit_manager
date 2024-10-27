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
import { Plus, X } from "lucide-react";
import { useState } from "react";
import useCreateExhibit from "@/hooks/data/useCreateExhibit";
import type { Exhibit } from "@/components/exhibit-card";
import { useEffect } from "react";
import { Separator } from "@/components/ui/separator";
import { ScrollArea } from "@/components/ui/scroll-area";

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
import { useGetUniqueClustersAndLocations } from "@/hooks/util/useGetUniqueClustersAndLocations";

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

interface CreateExhibitFormProps {
  onSuccess: () => void;
}

export function CreateExhibitForm({ onSuccess }: CreateExhibitFormProps) {
  const createExhibitMutation = useCreateExhibit();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const queryClient = useQueryClient();
  const { locations, clusters, isLoading, isError } =
    useGetUniqueClustersAndLocations();

  const [isNewCluster, setIsNewCluster] = useState(false);
  const [isNewLocation, setIsNewLocation] = useState(false);

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

  useEffect(() => {
    if (form.getValues("cluster") === "new") {
      setIsNewCluster(true);
      form.setValue("cluster", "");
    }
    if (form.getValues("location") === "new") {
      setIsNewLocation(true);
      form.setValue("location", "");
    }
  }, [form.getValues("cluster"), form.getValues("location")]);

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
      setIsNewCluster(false);
      setIsNewLocation(false);
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

  const CustomSelectContent = ({
    children,
    ...props
  }: React.ComponentProps<typeof SelectContent>) => (
    <SelectContent {...props}>
      <ScrollArea className="h-[200px]">{children}</ScrollArea>
      <Separator className="my-1" />
      <div className="bg-background p-1">
        <SelectItem value="new">Create new</SelectItem>
      </div>
    </SelectContent>
  );

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
                {isNewCluster || clusters.length === 0 ? (
                  <div className="flex items-center space-x-2">
                    <Input
                      placeholder="Enter new cluster name"
                      {...field}
                      onChange={(e) => {
                        field.onChange(e);
                        setIsNewCluster(true);
                      }}
                    />
                    <Button
                      type="button"
                      variant="ghost"
                      size="icon"
                      onClick={() => {
                        setIsNewCluster(false);
                        field.onChange(clusters[0] || "");
                      }}
                    >
                      <X className="h-4 w-4" />
                      <span className="sr-only">Cancel new cluster</span>
                    </Button>
                  </div>
                ) : (
                  <Select
                    onValueChange={(value) => {
                      if (value === "new") {
                        setIsNewCluster(true);
                        field.onChange("");
                      } else {
                        field.onChange(value);
                      }
                    }}
                    value={field.value}
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="Select a cluster" />
                    </SelectTrigger>
                    <CustomSelectContent>
                      {clusters.map((cluster) => (
                        <SelectItem key={cluster} value={cluster}>
                          {cluster}
                        </SelectItem>
                      ))}
                    </CustomSelectContent>
                  </Select>
                )}
              </FormControl>
              <FormDescription>
                {isNewCluster
                  ? "Enter a new cluster name"
                  : "Select an existing cluster or create a new one"}
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
                {isNewLocation || locations.length === 0 ? (
                  <div className="flex items-center space-x-2">
                    <Input
                      placeholder="Enter new location"
                      {...field}
                      onChange={(e) => {
                        field.onChange(e);
                        setIsNewLocation(true);
                      }}
                    />
                    <Button
                      type="button"
                      variant="ghost"
                      size="icon"
                      onClick={() => {
                        setIsNewLocation(false);
                        field.onChange(locations[0] || "");
                      }}
                    >
                      <X className="h-4 w-4" />
                      <span className="sr-only">Cancel new location</span>
                    </Button>
                  </div>
                ) : (
                  <Select
                    onValueChange={(value) => {
                      if (value === "new") {
                        setIsNewLocation(true);
                        field.onChange("");
                      } else {
                        field.onChange(value);
                      }
                    }}
                    value={field.value}
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="Select a location" />
                    </SelectTrigger>
                    <CustomSelectContent>
                      {locations.map((location) => (
                        <SelectItem key={location} value={location}>
                          {location}
                        </SelectItem>
                      ))}
                    </CustomSelectContent>
                  </Select>
                )}
              </FormControl>
              <FormDescription>
                {isNewLocation
                  ? "Enter a new location"
                  : "Select an existing location or create a new one"}
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
