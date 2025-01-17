import { ScrollArea } from "@/components/ui/scroll-area";
import { Card, CardContent } from "@/components/ui/card";
import { Loader2, ExternalLink, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import useGetExhibitParts from "@/hooks/data/queries/exhibits/useGetExhibitParts";
import useEditPart from "@/hooks/data/mutations/parts/useEditPart";
import { useState } from "react";

interface PartsListProps {
  refetchPartIds: () => void;
  partIds: string[];
  exhibitId: string;
}

export function PartsList({
  partIds,
  exhibitId,
  refetchPartIds,
}: PartsListProps) {
  const {
    data: parts,
    isLoading,
    isError,
    refetch,
  } = useGetExhibitParts(partIds);
  const editPartMutation = useEditPart();
  const [removingPartId, setRemovingPartId] = useState<string | null>(null);

  const handleRemovePart = async (partId: string) => {
    setRemovingPartId(partId);
    try {
      const partToUpdate = parts?.find((part) => part.id === partId);
      if (partToUpdate) {
        await editPartMutation.mutateAsync({
          id: partId,
          payload: {
            name: partToUpdate.name,
            link: partToUpdate.link,
            exhibitIds: partToUpdate.exhibit_ids
              .filter((id) => id !== exhibitId)
              .map((id) => Number(id)), // Convert string IDs to numbers
          },
        });
        refetch();
        refetchPartIds();
      }
    } catch (error) {
      console.error("Failed to remove part from exhibit:", error);
    } finally {
      setRemovingPartId(null);
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-24">
        <Loader2 className="h-6 w-6 animate-spin text-primary" />
      </div>
    );
  }

  if (isError) {
    return (
      <div className="text-sm text-destructive">
        Error loading parts. Please try again.
      </div>
    );
  }

  if (!parts || parts.length === 0) {
    return (
      <Card>
        <CardContent className="p-4 text-center text-muted-foreground">
          No parts connected to this exhibit.
        </CardContent>
      </Card>
    );
  }

  return (
    <ScrollArea className="h-48 w-full">
      <div className="space-y-2">
        {parts.map((part) => (
          <Card key={part.id}>
            <CardContent className="p-3 flex justify-between items-center">
              <h5 className="font-medium">{part.name}</h5>
              <div className="flex items-center space-x-2">
                <Button variant="ghost" size="sm" asChild>
                  <a href={part.link} target="_blank" rel="noopener noreferrer">
                    <ExternalLink className="h-4 w-4 mr-2" />
                    Buy
                  </a>
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => handleRemovePart(part.id)}
                  disabled={removingPartId === part.id}
                >
                  {removingPartId === part.id ? (
                    <Loader2 className="h-4 w-4 animate-spin" />
                  ) : (
                    <X className="h-4 w-4" />
                  )}
                  <span className="sr-only">Remove</span>
                </Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </ScrollArea>
  );
}
