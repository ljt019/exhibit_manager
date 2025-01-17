import {
  FormField,
  FormItem,
  FormLabel,
  FormDescription,
  FormMessage,
  FormControl,
} from "@/components/ui/form";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
  SelectValue,
} from "@/components/ui/select";
import { UseFormReturn } from "react-hook-form";

export function DetailsStep({ form }: { form: UseFormReturn<any> }) {
  return (
    <>
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
              Provide a detailed description of the exhibit (10-1000
              characters).
            </FormDescription>
            <FormMessage />
          </FormItem>
        )}
      />
    </>
  );
}
