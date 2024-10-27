import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
  SelectValue,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";
import { useState } from "react";
import useCreateExhibit from "@/hooks/useCreateExhibit";
import type { Exhibit } from "@/components/exhibit-card";

export function CreateExhibitDialog() {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const createExhibitMutation = useCreateExhibit();
  const [newExhibit, setNewExhibit] = useState<Partial<Exhibit>>({
    name: "",
    cluster: "",
    location: "",
    status: "operational",
    image_url: "",
  });

  const handleCreateExhibit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createExhibitMutation.mutateAsync(newExhibit as Exhibit);
      setIsDialogOpen(false);
      setNewExhibit({
        name: "",
        cluster: "",
        location: "",
        status: "operational",
        image_url: "",
      });
    } catch (error) {
      console.error("Failed to create exhibit:", error);
    }
  };

  const handleImageUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onloadend = () => {
        setNewExhibit({ ...newExhibit, image_url: reader.result as string });
      };
      reader.readAsDataURL(file);
    }
  };

  return (
    <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
      <DialogTrigger asChild>
        <Button variant="outline">
          <Plus className="w-4 h-4" />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create New Exhibit</DialogTitle>
          <DialogDescription>
            Fill in the details for the new exhibit.
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={handleCreateExhibit}>
          <div className="grid gap-4 py-4">
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="name" className="text-right">
                Name
              </Label>
              <Input
                id="name"
                value={newExhibit.name}
                onChange={(e) =>
                  setNewExhibit({ ...newExhibit, name: e.target.value })
                }
                className="col-span-3"
              />
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="cluster" className="text-right">
                Cluster
              </Label>
              <Input
                id="cluster"
                value={newExhibit.cluster}
                onChange={(e) =>
                  setNewExhibit({
                    ...newExhibit,
                    cluster: e.target.value,
                  })
                }
                className="col-span-3"
              />
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="location" className="text-right">
                Location
              </Label>
              <Input
                id="location"
                value={newExhibit.location}
                onChange={(e) =>
                  setNewExhibit({
                    ...newExhibit,
                    location: e.target.value,
                  })
                }
                className="col-span-3"
              />
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="status" className="text-right">
                Status
              </Label>
              <Select
                value={newExhibit.status}
                onValueChange={(value) =>
                  setNewExhibit({
                    ...newExhibit,
                    status: value as Exhibit["status"],
                  })
                }
              >
                <SelectTrigger className="col-span-3">
                  <SelectValue placeholder="Select status" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="operational">Operational</SelectItem>
                  <SelectItem value="needs repair">Needs Repair</SelectItem>
                  <SelectItem value="out of service">Out of Service</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="image" className="text-right">
                Image
              </Label>
              <Input
                id="image"
                type="file"
                accept="image/*"
                onChange={handleImageUpload}
                className="col-span-3"
              />
            </div>
          </div>
          <DialogFooter>
            <Button type="submit">Create Exhibit</Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
