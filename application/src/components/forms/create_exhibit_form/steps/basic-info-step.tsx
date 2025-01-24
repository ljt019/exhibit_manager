import { useState } from "react";
import type { UseFormReturn } from "react-hook-form";
import { X } from "lucide-react";
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
  SelectSeparator,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";

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

  const handleSelectChange = (
    value: string,
    field: any,
    setIsNew: (value: boolean) => void
  ) => {
    if (value === "new") {
      setIsNew(true);
      field.onChange("");
    } else {
      field.onChange(value);
    }
  };

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
              {isNewCluster ? (
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
                  onValueChange={(value) =>
                    handleSelectChange(value, field, setIsNewCluster)
                  }
                  value={field.value}
                  onOpenChange={(open) => {
                    if (open) {
                      setTimeout(() => {
                        const content = document.querySelector(
                          ".select-scroll-area"
                        );
                        if (content) {
                          content.addEventListener("wheel", (e) => {
                            e.stopPropagation();
                          });
                        }
                      }, 0);
                    }
                  }}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select a cluster" />
                  </SelectTrigger>
                  <SelectContent>
                    <ScrollArea className="h-[200px] select-scroll-area">
                      <SelectItem
                        value="new"
                        className="cursor-pointer hover:bg-muted/50 transition-colors"
                      >
                        Create New Cluster
                      </SelectItem>
                      <SelectSeparator />
                      {clusters.map((cluster) => (
                        <SelectItem
                          key={cluster}
                          value={cluster}
                          className="cursor-pointer hover:bg-muted/50 transition-colors"
                        >
                          {cluster}
                        </SelectItem>
                      ))}
                    </ScrollArea>
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
              {isNewLocation ? (
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
                  onValueChange={(value) =>
                    handleSelectChange(value, field, setIsNewLocation)
                  }
                  value={field.value}
                  onOpenChange={(open) => {
                    if (open) {
                      setTimeout(() => {
                        const content = document.querySelector(
                          ".select-scroll-area"
                        );
                        if (content) {
                          content.addEventListener("wheel", (e) => {
                            e.stopPropagation();
                          });
                        }
                      }, 0);
                    }
                  }}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select a location" />
                  </SelectTrigger>
                  <SelectContent>
                    <ScrollArea className="h-[200px] select-scroll-area">
                      <SelectItem
                        value="new"
                        className="cursor-pointer hover:bg-muted/50 transition-colors"
                      >
                        Create New Location
                      </SelectItem>
                      <SelectSeparator />
                      {locations.map((location) => (
                        <SelectItem
                          key={location}
                          value={location}
                          className="cursor-pointer hover:bg-muted/50 transition-colors"
                        >
                          {location}
                        </SelectItem>
                      ))}
                    </ScrollArea>
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
