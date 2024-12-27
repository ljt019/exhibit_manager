import {
  MapPin,
  Star,
  StickyNote,
  Boxes,
  MoreVertical,
  Trash2,
  Loader2,
} from "lucide-react";
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
import useDeleteExhibit from "@/hooks/data/mutations/exhibits/useDeleteExhibit";
import type { Exhibit, Note, Sponsorship } from "@/types";
import { PartsButton } from "@/components/parts-dialog";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { format } from "date-fns";

import { Input } from "@/components/ui/input";
import useCreateExhibit from "@/hooks/data/mutations/exhibits/useCreateExhibitNote";
import useDeleteExhibitNote from "@/hooks/data/mutations/exhibits/useDeleteExhibitNote";
import { cn } from "@/lib/utils";

const statusColors: Record<Exhibit["status"], string> = {
  Operational: "bg-green-500/10 text-green-500 hover:bg-green-500/20",
  "Needs Repair": "bg-yellow-500/10 text-yellow-500 hover:bg-yellow-500/20",
  "Out of Service": "bg-red-500/10 text-red-500 hover:bg-red-500/20",
};

const getStatusColor = (status: Exhibit["status"]) => statusColors[status];

export function ExhibitCard({ exhibit }: { exhibit: Exhibit }) {
  const deleteExhibitMutation = useDeleteExhibit();

  return (
    <Card className="flex flex-col h-full">
      <CardHeader className="p-4 pb-0">
        <div className="flex items-start space-x-4">
          <img
            src={exhibit.image_url}
            alt={exhibit.name}
            className="w-36 h-36 object-fill rounded-md"
          />
          <div className="flex-1 flex flex-col">
            <div className="flex justify-between items-start">
              <h3 className="font-semibold text-lg break-words flex-1 mr-4">
                {exhibit.name}
              </h3>
              <div className="flex items-center space-x-2">
                <Badge
                  className={cn(getStatusColor(exhibit.status))}
                  variant="secondary"
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
                      className="text-destructive"
                    >
                      Delete
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>
            </div>
            {/* Details */}
            <div className="text-sm text-muted-foreground mt-2 space-y-1">
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
          <NotesButton
            exhibitId={exhibit.id}
            name={exhibit.name}
            notes={exhibit.notes}
          />
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

interface NotesButtonProps {
  exhibitId: string;
  name: string;
  notes: Array<Note>;
}

export function NotesButton({ exhibitId, name, notes }: NotesButtonProps) {
  const [isOpen, setIsOpen] = useState(false);
  const { register, handleSubmit } = useForm<{ message: string }>();
  const createNote = useCreateExhibit();
  const deleteNote = useDeleteExhibitNote();

  const onSubmit = async (data: { message: string }) => {
    try {
      await createNote.mutateAsync({
        exhibitId,
        note: { message: data.message },
      });
    } catch (error) {
      console.error("Error creating note:", error);
    }
  };

  const handleDelete = async (noteId: string) => {
    await deleteNote.mutateAsync({ exhibitId, noteId });
  };

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button variant="outline" size="sm" className="w-full">
          <StickyNote className="w-4 h-4 mr-2" />
          Notes ({notes.length})
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>{name}</DialogTitle>
        </DialogHeader>
        <div className="mt-4 space-y-4">
          <form onSubmit={handleSubmit(onSubmit)} className="flex space-x-2">
            <Input
              {...register("message", { required: true })}
              placeholder="Add a new note..."
              className="flex-grow"
            />
            <Button type="submit" disabled={createNote.isPending}>
              {createNote.isPending ? (
                <Loader2 className="w-4 h-4 animate-spin" />
              ) : (
                "Add"
              )}
            </Button>
          </form>
          <ScrollArea className="h-[50vh]">
            {notes.length === 0 ? (
              <p className="text-center text-muted-foreground">No notes yet</p>
            ) : (
              <div className="space-y-4">
                {notes.map((note) => (
                  <Card key={note.id}>
                    <CardContent className="p-4 flex justify-between items-start">
                      <div>
                        <p className="text-sm text-muted-foreground mb-1">
                          {format(
                            new Date(
                              note.timestamp.date + " " + note.timestamp.time
                            ),
                            "PPpp"
                          )}
                        </p>
                        <p>{note.message}</p>
                      </div>
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => handleDelete(note.id)}
                        disabled={deleteNote.isPending}
                      >
                        {deleteNote.isPending ? (
                          <Loader2 className="w-4 h-4 animate-spin" />
                        ) : (
                          <Trash2 className="w-4 h-4" />
                        )}
                      </Button>
                    </CardContent>
                  </Card>
                ))}
              </div>
            )}
          </ScrollArea>
        </div>
      </DialogContent>
    </Dialog>
  );
}
