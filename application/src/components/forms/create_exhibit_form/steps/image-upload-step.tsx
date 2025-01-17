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

export function ImageUploadStep({ form }: { form: UseFormReturn<any> }) {
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
    <FormField
      control={form.control}
      name="image_url"
      render={({}) => (
        <FormItem>
          <FormLabel>Image</FormLabel>
          <FormControl>
            <Input type="file" accept="image/*" onChange={handleImageUpload} />
          </FormControl>
          <FormDescription>
            Upload an image for the exhibit (optional).
          </FormDescription>
          <FormMessage />
        </FormItem>
      )}
    />
  );
}
