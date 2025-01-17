import { useState } from "react";
import { MoreVertical, Pencil, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import useDeletePart from "@/hooks/data/mutations/parts/useDeletePart";
import useDeleteExhibit from "@/hooks/data/mutations/exhibits/useDeleteExhibit";
import { GenericEditDialog } from "@/components/generic/edit-dialogue";

interface MoreActionsProps {
  id: string;
  type: "exhibit" | "part";
}

export function MoreActions({ id, type }: MoreActionsProps) {
  const deletePartMutation = useDeletePart();
  const deleteExhibitMutation = useDeleteExhibit();
  const [isEditDialogOpen, setIsEditDialogOpen] = useState(false);

  const handleEdit = () => {
    setIsEditDialogOpen(true);
  };

  const handleCloseEditDialog = () => {
    setIsEditDialogOpen(false);
  };

  const handleDelete = () => {
    if (type === "part") {
      deletePartMutation.mutate(id);
    } else {
      deleteExhibitMutation.mutate(id);
    }
  };

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <MoreVertical className="h-4 w-4" />
            <span className="sr-only">More actions</span>
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem onClick={handleEdit}>
            <Pencil className="mr-2 h-4 w-4" />
            Edit
          </DropdownMenuItem>
          <DropdownMenuItem onClick={handleDelete} className="text-destructive">
            <Trash2 className="mr-2 h-4 w-4" />
            Delete
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <GenericEditDialog
        id={id}
        type={type}
        isOpen={isEditDialogOpen}
        onClose={handleCloseEditDialog}
      />
    </>
  );
}
