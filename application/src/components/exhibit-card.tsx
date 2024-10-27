import { MapPin, Star, StickyNote, Boxes, MoreVertical } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { calculateTimeUntilExpiration } from "@/lib/date";
import useDeleteExhibit from "@/hooks/data/mutations/useDeleteExhibit";
import type { Exhibit, Sponsorship } from "@/types";
import { PartsButton } from "@/components/parts-dialog";

const statusColors: Record<Exhibit["status"], string> = {
  operational: "bg-green-500",
  "needs repair": "bg-yellow-500",
  "out of service": "bg-red-500",
};

export function ExhibitCard({ exhibit }: { exhibit: Exhibit }) {
  const deleteExhibitMutation = useDeleteExhibit();

  return (
    <Card className="flex flex-col h-full">
      <CardHeader className="p-4 pb-0">
        <div className="flex items-start space-x-4">
          <img
            src={exhibit.image_url}
            alt={exhibit.name}
            className="w-36 h-36 object-cover rounded-md"
          />
          <div className="flex-1">
            <div className="flex items-center justify-between w-full">
              <h3 className="font-semibold text-lg">{exhibit.name}</h3>
              <div className="flex items-center space-x-2">
                <Badge
                  className={`${
                    statusColors[exhibit.status]
                  } text-white text-nowrap`}
                >
                  {exhibit.status}
                </Badge>
                <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <Button variant="ghost" size="icon" className="h-8 w-8">
                      <MoreVertical className="h-4 w-4" />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="end">
                    <DropdownMenuItem>Edit</DropdownMenuItem>
                    <DropdownMenuItem
                      onClick={() => deleteExhibitMutation.mutate(exhibit.id)}
                    >
                      Delete
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>
            </div>
            <div className="text-sm text-muted-foreground mt-1 space-y-1">
              <div className="flex items-center gap-1">
                <Boxes className="w-3 h-3" />
                <span className="truncate">{exhibit.cluster}</span>
              </div>
              <div className="flex items-center gap-1">
                <MapPin className="w-3 h-3" />
                <span className="truncate">{exhibit.location}</span>
              </div>
            </div>
          </div>
        </div>
      </CardHeader>
      <CardContent className="p-4 pt-0">
        <div className="mt-4 space-y-2">
          <SponsorshipButton sponsorship={exhibit.sponsorship} />
          <PartsButton
            name={exhibit.name}
            parts={exhibit.part_ids}
            exhibitId={exhibit.id}
          />
          <NotesButton name={exhibit.name} notes={exhibit.notes} />
        </div>
      </CardContent>
    </Card>
  );
}

function SponsorshipButton({ sponsorship }: { sponsorship?: Sponsorship }) {
  if (!sponsorship) return null;
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline" size="sm" className="w-full">
          <Star className="w-4 h-4 mr-2" />
          Sponsored
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Sponsorship Details</DialogTitle>
        </DialogHeader>
        <div className="space-y-2">
          <p>
            <strong>Sponsor:</strong> {sponsorship.sponsorName}
          </p>
          <p>
            <strong>Start Date:</strong> {sponsorship.startDate}
          </p>
          <p>
            <strong>End Date:</strong> {sponsorship.endDate}
          </p>
          <p>
            <strong>Time until expiration:</strong>{" "}
            {calculateTimeUntilExpiration(sponsorship.endDate)}
          </p>
        </div>
      </DialogContent>
    </Dialog>
  );
}

function NotesButton({
  name,
  notes,
}: {
  name: string;
  notes: Array<{ timestamp: string; note: string }>;
}) {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline" size="sm" className="w-full">
          <StickyNote className="w-4 h-4 mr-2" />
          Notes ({notes.length})
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>{name} - Notes</DialogTitle>
        </DialogHeader>
        <ScrollArea className="h-[60vh] mt-4">
          <ul className="space-y-4">
            {notes.map((note, index) => (
              <li key={index} className="border-b pb-2">
                <span className="font-medium">{note.timestamp}:</span>{" "}
                {note.note}
              </li>
            ))}
          </ul>
        </ScrollArea>
      </DialogContent>
    </Dialog>
  );
}
