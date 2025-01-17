import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
import { useState, useEffect } from "react";
import { EditExhibitForm } from "@/components/forms/edit-exhibit-form";
import { EditPartForm } from "@/components/forms/edit-part-form";
import useGetExhibit from "@/hooks/data/queries/exhibits/useGetExhibit";
import useGetPart from "@/hooks/data/queries/parts/useGetPart";
import type { Part, Exhibit } from "@/types";

interface GenericEditDialogProps {
  id: string;
  type: "exhibit" | "part";
  isOpen: boolean;
  onClose: () => void;
}

export function GenericEditDialog({
  id,
  type,
  isOpen,
  onClose,
}: GenericEditDialogProps) {
  const [shouldFetch, setShouldFetch] = useState(false);
  const { data: exhibit, isLoading: isExhibitLoading } = useGetExhibit(id, {
    enabled: shouldFetch && type === "exhibit",
  });
  const { data: part, isLoading: isPartLoading } = useGetPart(id, {
    enabled: shouldFetch && type === "part",
  });

  useEffect(() => {
    if (isOpen) {
      setShouldFetch(true);
    } else {
      setShouldFetch(false);
    }
  }, [isOpen]);

  const isLoading = type === "exhibit" ? isExhibitLoading : isPartLoading;
  const data = type === "exhibit" ? exhibit : part;

  const handleDialogClose = () => {
    onClose();
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            Edit {type === "exhibit" ? "Exhibit" : "Part"}
          </DialogTitle>
          <DialogDescription>
            Update the details for this{" "}
            {type === "exhibit" ? "exhibit" : "part"}.
          </DialogDescription>
        </DialogHeader>
        {isLoading ? (
          <div>Loading...</div>
        ) : data ? (
          type === "exhibit" ? (
            <EditExhibitForm
              exhibit={data as Exhibit}
              onSuccess={handleDialogClose}
            />
          ) : (
            <EditPartForm part={data as Part} onSuccess={handleDialogClose} />
          )
        ) : null}
      </DialogContent>
    </Dialog>
  );
}
