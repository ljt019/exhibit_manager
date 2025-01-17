import {
  FormField,
  FormItem,
  FormLabel,
  FormDescription,
  FormMessage,
  FormControl,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
  SelectValue,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { X } from "lucide-react";
import { useState } from "react";
import { UseFormReturn } from "react-hook-form";

export function BasicInfoStep({
  form,
  clusters,
  locations,
}: {
  form: UseFormReturn<any>;
  clusters: string[];
  locations: string[];
}) {
  const [isNewCluster, setIsNewCluster] = useState(false);
  const [isNewLocation, setIsNewLocation] = useState(false);

  return (
    <>
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
                  <SelectContent>
                    {clusters.map((cluster) => (
                      <SelectItem key={cluster} value={cluster}>
                        {cluster}
                      </SelectItem>
                    ))}
                    <SelectItem value="new">Create new</SelectItem>
                  </SelectContent>
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
                  <SelectContent>
                    {locations.map((location) => (
                      <SelectItem key={location} value={location}>
                        {location}
                      </SelectItem>
                    ))}
                    <SelectItem value="new">Create new</SelectItem>
                  </SelectContent>
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
    </>
  );
}
