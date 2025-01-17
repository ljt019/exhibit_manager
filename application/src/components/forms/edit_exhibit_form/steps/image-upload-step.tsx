import {
  FormField,
  FormItem,
  FormLabel,
  FormDescription,
  FormMessage,
  FormControl,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { UseFormReturn } from "react-hook-form";
import { useState } from "react";

export function ImageUploadStep({ form }: { form: UseFormReturn<any> }) {
  const [previewUrl, setPreviewUrl] = useState<string | null>(
    form.getValues("image_url") || null
  );

  const handleImageUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault(); // Prevent form submission
    const file = e.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onloadend = () => {
        const result = reader.result as string;
        form.setValue("image_url", result);
        setPreviewUrl(result);
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
            <Input type="file" accept="image/*" onChange={handleImageUpload} />
          </FormControl>
          <FormDescription>
            Upload a new image for the exhibit (optional).
          </FormDescription>
          <FormMessage />
          {previewUrl && (
            <div className="mt-4">
              <img
                src={previewUrl || "/placeholder.svg"}
                alt="Preview"
                className="max-w-full h-auto"
              />
            </div>
          )}
        </FormItem>
      )}
    />
  );
}
