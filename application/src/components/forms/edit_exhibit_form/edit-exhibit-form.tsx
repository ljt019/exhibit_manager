import { useState, useCallback } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import * as z from "zod";
import { toast } from "@/hooks/use-toast";
import { Form } from "@/components/ui/form";
import { Button } from "@/components/ui/button";
import { useQueryClient } from "@tanstack/react-query";
import { useGetUniqueClustersAndLocations } from "@/hooks/util/useGetUniqueClustersAndLocations";
import useEditExhibit from "@/hooks/data/mutations/exhibits/useEditExhibit";
import type { Exhibit } from "@/types";
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

interface EditExhibitFormProps {
  exhibit: Exhibit;
  onSuccess: () => void;
}

export function EditExhibitForm({ exhibit, onSuccess }: EditExhibitFormProps) {
  const [step, setStep] = useState(0);
  const editExhibitMutation = useEditExhibit();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const queryClient = useQueryClient();
  const { locations, clusters } = useGetUniqueClustersAndLocations();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: exhibit.name,
      cluster: exhibit.cluster,
      location: exhibit.location,
      description: exhibit.description,
      image_url: exhibit.image_url,
    },
  });

  const onSubmit = useCallback(
    async (values: z.infer<typeof formSchema>) => {
      setIsSubmitting(true);
      try {
        await editExhibitMutation.mutateAsync({
          id: exhibit.id,
          payload: values,
        });
        toast({
          title: "Exhibit updated",
          description: "Your exhibit has been successfully updated.",
        });
        onSuccess();
      } catch (error) {
        console.error("Failed to update exhibit:", error);
        toast({
          title: "Error",
          description: "Failed to update exhibit. Please try again.",
          variant: "destructive",
        });
      } finally {
        queryClient.invalidateQueries({ queryKey: ["exhibits"] });
        setIsSubmitting(false);
      }
    },
    [editExhibitMutation, exhibit.id, onSuccess, queryClient]
  );

  const handleNext = useCallback(() => {
    setStep((prevStep) => Math.min(prevStep + 1, 2));
  }, []);

  const handlePrevious = useCallback(() => {
    setStep((prevStep) => Math.max(prevStep - 1, 0));
  }, []);

  const handleSubmit = useCallback(
    (e: React.FormEvent<HTMLFormElement>) => {
      e.preventDefault();
      if (step === 2) {
        form.handleSubmit(onSubmit)(e);
      } else {
        handleNext();
      }
    },
    [step, form, onSubmit, handleNext]
  );

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
      <form onSubmit={handleSubmit} className="space-y-8">
        {steps[step]}
        <div className="flex justify-between">
          {step > 0 && (
            <Button type="button" onClick={handlePrevious}>
              Previous
            </Button>
          )}
          <Button type="submit">
            {step === 2
              ? isSubmitting
                ? "Updating..."
                : "Update Exhibit"
              : "Next"}
          </Button>
        </div>
      </form>
    </Form>
  );
}
