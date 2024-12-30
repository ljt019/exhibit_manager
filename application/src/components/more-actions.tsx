import { MoreVertical, Pencil, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import useDeleteExhibit from "@/hooks/data/mutations/exhibits/useDeleteExhibit";

interface MoreActionsProps {
  exhibitId: string;
}

export function MoreActions({ exhibitId }: MoreActionsProps) {
  const deleteExhibitMutation = useDeleteExhibit();

  const handleEdit = () => {
    // Implement edit functionality
    console.log("Edit exhibit", exhibitId);
  };

  const handleDelete = () => {
    deleteExhibitMutation.mutate(exhibitId);
  };

  return (
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
  );
}
