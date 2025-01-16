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
import { CreatePartForm } from "@/components/forms/create-part-form";

interface CreatePartDialogProps {
  exhibitId?: string;
}

export function CreatePartDialog({ exhibitId }: CreatePartDialogProps) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  const handleDialogClose = () => {
    setIsDialogOpen(false);
  };

  return (
    <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
      <DialogTrigger asChild>
        <Button variant="outline">
          <Plus className="w-4 h-4" />
          <span className="sr-only">Create New Part</span>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create New Part</DialogTitle>
          <DialogDescription>
            Fill in the details for the new part.
          </DialogDescription>
        </DialogHeader>
        <CreatePartForm onSuccess={handleDialogClose} exhibitId={exhibitId} />
      </DialogContent>
    </Dialog>
  );
}
