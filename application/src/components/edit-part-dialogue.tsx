import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
import { useState, useEffect } from "react";
import useGetPart from "@/hooks/data/queries/parts/useGetPart";
import { EditPartForm } from "@/components/edit-part-form";

interface EditPartDialogProps {
  partId: string;
  isOpen: boolean;
  onClose: () => void;
}

export function EditPartDialog({
  partId,
  isOpen,
  onClose,
}: EditPartDialogProps) {
  const [shouldFetch, setShouldFetch] = useState(false);
  const { data: part, isLoading } = useGetPart(partId, {
    enabled: shouldFetch,
  });

  useEffect(() => {
    if (isOpen) {
      setShouldFetch(true);
    } else {
      setShouldFetch(false);
    }
  }, [isOpen]);

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Part</DialogTitle>
          <DialogDescription>
            Update the details for this part.
          </DialogDescription>
        </DialogHeader>
        {isLoading ? (
          <div>Loading...</div>
        ) : part ? (
          <EditPartForm part={part} onSuccess={onClose} />
        ) : null}
      </DialogContent>
    </Dialog>
  );
}
