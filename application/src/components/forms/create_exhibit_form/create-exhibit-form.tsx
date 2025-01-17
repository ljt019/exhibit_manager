import { useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import * as z from "zod";
import { toast } from "@/hooks/use-toast";
import { Form } from "@/components/ui/form";
import { Button } from "@/components/ui/button";
import { useQueryClient } from "@tanstack/react-query";
import { useGetUniqueClustersAndLocations } from "@/hooks/util/useGetUniqueClustersAndLocations";
import useCreateExhibit from "@/hooks/data/mutations/exhibits/useCreateExhibit";
import { BasicInfoStep } from "./steps/basic-info-step";
import { DetailsStep } from "./steps/details-step";
import { ImageUploadStep } from "./steps/image-upload-step";

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
  description: z
    .string()
    .min(10, {
      message: "Description must be at least 10 characters.",
    })
    .max(1000, {
      message: "Description must not exceed 1000 characters.",
    }),
  image_url: z.string().url().optional(),
});

interface CreateExhibitFormProps {
  onSuccess: () => void;
}

export function CreateExhibitForm({ onSuccess }: CreateExhibitFormProps) {
  const [step, setStep] = useState(0);
  const createExhibitMutation = useCreateExhibit();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const queryClient = useQueryClient();
  const { locations, clusters } = useGetUniqueClustersAndLocations();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
      cluster: "",
      location: "",
      status: "operational",
      description: "",
      image_url: "",
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setIsSubmitting(true);
    try {
      await createExhibitMutation.mutateAsync(values);
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

  const steps = [
    <BasicInfoStep
      key="basic-info"
      form={form}
      clusters={clusters}
      locations={locations}
    />,
    <DetailsStep key="details" form={form} />,
    <ImageUploadStep key="image-upload" form={form} />,
  ];

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        {steps[step]}
        <div className="flex justify-between">
          {step > 0 && (
            <Button type="button" onClick={() => setStep((prev) => prev - 1)}>
              Previous
            </Button>
          )}
          {step < steps.length - 1 ? (
            <Button type="button" onClick={() => setStep((prev) => prev + 1)}>
              Next
            </Button>
          ) : (
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting ? "Creating..." : "Create Exhibit"}
            </Button>
          )}
        </div>
      </form>
    </Form>
  );
}
