import { ScrollArea } from "@/components/ui/scroll-area";
import { Card, CardContent } from "@/components/ui/card";
import { Loader2, ExternalLink } from "lucide-react";
import { Button } from "@/components/ui/button";
import useGetExhibitParts from "@/hooks/data/queries/exhibits/useGetExhibitParts";

interface PartsListProps {
  partIds: string[];
}

export function PartsList({ partIds }: PartsListProps) {
  const { data: parts, isLoading, isError } = useGetExhibitParts(partIds);

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
              <Button variant="ghost" size="sm" asChild>
                <a href={part.link} target="_blank" rel="noopener noreferrer">
                  <ExternalLink className="h-4 w-4 mr-2" />
                  Buy
                </a>
              </Button>
            </CardContent>
          </Card>
        ))}
      </div>
    </ScrollArea>
  );
}
