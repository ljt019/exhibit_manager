import {
  ChevronDown,
  Hammer,
  MapPin,
  Star,
  StickyNote,
  Boxes,
  MoreVertical,
} from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
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

type Sponsorship = {
  sponsorName: string;
  startDate: string;
  endDate: string;
};

export type Exhibit = {
  id: string;
  name: string;
  cluster: string;
  location: string;
  status: "operational" | "needs repair" | "out of service";
  part_ids: Array<string>;
  notes: Array<{ timestamp: string; text: string }>;
  image_url: string | undefined;
  sponsorship?: Sponsorship;
};

const statusColors: Record<Exhibit["status"], string> = {
  operational: "bg-green-500",
  "needs repair": "bg-yellow-500",
  "out of service": "bg-red-500",
};

export function ExhibitCard({ exhibit }: { exhibit: Exhibit }) {
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
                <Badge className={`${statusColors[exhibit.status]} text-white`}>
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
                    <DropdownMenuItem>Delete</DropdownMenuItem>
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
          <PartsButton parts={exhibit.part_ids} />
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

function PartsButton({ parts }: { parts: string[] }) {
  if (!parts) return null;

  return (
    <Collapsible>
      <CollapsibleTrigger asChild>
        <Button variant="outline" size="sm" className="w-full">
          <Hammer className="w-4 h-4 mr-2" />
          Parts ({parts.length})
          <ChevronDown className="w-4 h-4 ml-auto" />
        </Button>
      </CollapsibleTrigger>
      <CollapsibleContent className="mt-2 bg-background border rounded-md shadow-lg z-10">
        <ScrollArea className="h-32 w-full p-2">
          <ul className="text-sm space-y-1">
            {parts.map((part, index) => (
              <li key={index}>{part}</li>
            ))}
          </ul>
        </ScrollArea>
      </CollapsibleContent>
    </Collapsible>
  );
}

function NotesButton({
  name,
  notes,
}: {
  name: string;
  notes: Array<{ timestamp: string; text: string }>;
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
                {note.text}
              </li>
            ))}
          </ul>
        </ScrollArea>
      </DialogContent>
    </Dialog>
  );
}
