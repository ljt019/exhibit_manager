import {
  FormField,
  FormItem,
  FormLabel,
  FormDescription,
  FormMessage,
  FormControl,
} from "@/components/ui/form";
import { Textarea } from "@/components/ui/textarea";
import { UseFormReturn } from "react-hook-form";

export function DetailsStep({ form }: { form: UseFormReturn<any> }) {
  return (
    <FormField
      control={form.control}
      name="description"
      render={({ field }) => (
        <FormItem>
          <FormLabel>Description</FormLabel>
          <FormControl>
            <Textarea
              placeholder="Enter a detailed description of the exhibit"
              className="min-h-[100px]"
              {...field}
            />
          </FormControl>
          <FormDescription>
            Provide a detailed description of the exhibit (10-1000 characters).
          </FormDescription>
          <FormMessage />
        </FormItem>
      )}
    />
  );
}
