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
import { CreateExhibitForm } from "@/components/forms/create_exhibit_form/create-exhibit-form";

interface CreateDialogProps {
  type: "exhibit" | "part";
  exhibitId?: string;
}

export function CreateDialog({ type, exhibitId }: CreateDialogProps) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  const handleDialogClose = () => {
    setIsDialogOpen(false);
  };

  const title = type === "exhibit" ? "Create New Exhibit" : "Create New Part";
  const description = `Fill in the details for the new ${type}.`;

  return (
    <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
      <DialogTrigger asChild>
        <Button variant="outline">
          <Plus className="w-4 h-4" />
          <span className="sr-only">{title}</span>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription>{description}</DialogDescription>
        </DialogHeader>
        {type === "exhibit" ? (
          <CreateExhibitForm onSuccess={handleDialogClose} />
        ) : (
          <CreatePartForm onSuccess={handleDialogClose} exhibitId={exhibitId} />
        )}
      </DialogContent>
    </Dialog>
  );
}
