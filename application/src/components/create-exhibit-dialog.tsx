import { useState } from "react";
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";
import { CreateExhibitForm } from "@/components/forms/create-exhibit-form";

export function CreateExhibitDialog() {
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  const handleDialogClose = () => {
    setIsDialogOpen(false);
  };

  return (
    <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
      <DialogTrigger asChild>
        <Button variant="outline">
          <Plus className="w-4 h-4" />
          <span className="sr-only">Create New Exhibit</span>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create New Exhibit</DialogTitle>
          <DialogDescription>
            Fill in the details for the new exhibit.
          </DialogDescription>
        </DialogHeader>
        <CreateExhibitForm onSuccess={handleDialogClose} />
      </DialogContent>
    </Dialog>
  );
}
