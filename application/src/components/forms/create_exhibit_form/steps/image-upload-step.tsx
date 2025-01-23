import type React from "react";
import type { UseFormReturn } from "react-hook-form";
import {
  FormField,
  FormItem,
  FormLabel,
  FormControl,
  FormDescription,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";

interface ImageUploadStepProps {
  form: UseFormReturn<any>;
}

export const ImageUploadStep: React.FC<ImageUploadStepProps> = ({ form }) => {
  const handleImageUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    // Prevent the default form submission behavior
    e.preventDefault();

    const file = e.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onloadend = () => {
        // Use setValue with { shouldValidate: true } to trigger validation
        form.setValue("image_url", reader.result as string, {
          shouldValidate: true,
        });
      };
      reader.readAsDataURL(file);
    }
  };

  return (
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
              // Use onChangeCapture instead of onChange to have more control
              onChangeCapture={(e) => {
                e.preventDefault(); // Prevent default form submission
                handleImageUpload(e as React.ChangeEvent<HTMLInputElement>);
              }}
              onBlur={field.onBlur}
              name={field.name}
              ref={field.ref}
            />
          </FormControl>
          {form.watch("image_url") && (
            <div className="mt-2">
              <img
                src={form.watch("image_url") || "/placeholder.svg"}
                alt="Selected exhibit image"
                className="max-w-full h-auto"
              />
            </div>
          )}
          <FormDescription>
            Upload an image for the exhibit (optional).
          </FormDescription>
          <FormMessage />
        </FormItem>
      )}
    />
  );
};
