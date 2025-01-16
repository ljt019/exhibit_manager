import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
import { useState, useEffect } from "react";
import useGetExhibit from "@/hooks/data/queries/exhibits/useGetExhibit";
import { EditExhibitForm } from "@/components/edit-exhibit-form";

interface EditExhibitDialogProps {
  exhibitId: string;
  isOpen: boolean;
  onClose: () => void;
}

export function EditExhibitDialog({
  exhibitId,
  isOpen,
  onClose,
}: EditExhibitDialogProps) {
  const [shouldFetch, setShouldFetch] = useState(false);
  const { data: exhibit, isLoading } = useGetExhibit(exhibitId, {
    enabled: shouldFetch,
  });

  useEffect(() => {
    if (isOpen) {
      setShouldFetch(true);
    } else {
      setShouldFetch(false);
    }
  }, [isOpen]);

  const handleDialogClose = () => {
    onClose();
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Exhibit</DialogTitle>
          <DialogDescription>
            Update the details for this exhibit.
          </DialogDescription>
        </DialogHeader>
        {isLoading ? (
          <div>Loading...</div>
        ) : exhibit ? (
          <EditExhibitForm exhibit={exhibit} onSuccess={handleDialogClose} />
        ) : null}
      </DialogContent>
    </Dialog>
  );
}
