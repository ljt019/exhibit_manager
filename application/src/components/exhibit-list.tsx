import { ScrollArea } from "@/components/ui/scroll-area";
import { ExhibitCard } from "@/components/exhibit-card";
import type { Exhibit } from "@/types";

interface ExhibitListProps {
  filteredExhibits: Exhibit[];
}

export function ExhibitList({ filteredExhibits }: ExhibitListProps) {
  return (
    <ScrollArea className="h-[calc(100vh-200px)]">
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
        {filteredExhibits.map((exhibit) => (
          <ExhibitCard key={exhibit.id} exhibit={exhibit} />
        ))}
      </div>
    </ScrollArea>
  );
}
