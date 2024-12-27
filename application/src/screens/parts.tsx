import { useState } from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  useReactTable,
  SortingState,
  getSortedRowModel,
} from "@tanstack/react-table";
import { Button } from "@/components/ui/button";
import { ArrowUpDown, ExternalLink, Atom } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import useGetParts from "@/hooks/data/queries/parts/useGetParts";
import { FilterSection } from "@/components/filter-section";
import { Part } from "@/types/types";
import { CreatePartDialog } from "@/components/create-part-dialog";
import { Error, Loading } from "@/components/loading-and-error";
import { useForm } from "react-hook-form";
import { format } from "date-fns";
import { StickyNote, Loader2, Trash2 } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Card, CardContent } from "@/components/ui/card";
import useCreatePartNote from "@/hooks/data/mutations/parts/useCreatePartNote";
import useDeletePartNote from "@/hooks/data/mutations/parts/useDeletePartNote";
import { Note } from "@/types";

const columns: ColumnDef<Part>[] = [
  {
    accessorKey: "name",
    header: ({ column }) => {
      return (
        <Button
          variant="ghost"
          onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
        >
          Part Name
          <ArrowUpDown className="ml-2 h-4 w-4" />
        </Button>
      );
    },
    cell: ({ row }) => <div>{row.getValue("name")}</div>,
  },
  {
    accessorKey: "exhibit_ids",
    header: "Exhibits",
    cell: ({ row }) => {
      const exhibit_ids = row.getValue("exhibit_ids") as string[];
      return (
        <Dialog>
          <DialogTrigger asChild>
            <Button variant="outline" size="sm">
              <Atom className="w-4 h-4 mr-2" />
              {exhibit_ids ? exhibit_ids.length : 0}
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Attached Exhibits</DialogTitle>
              <DialogDescription>
                Exhibits this part is attached to:
              </DialogDescription>
            </DialogHeader>
            {exhibit_ids && (
              <ConnectedExhibitsDisplay exhibitIds={exhibit_ids} />
            )}
          </DialogContent>
        </Dialog>
      );
    },
  },
  {
    accessorKey: "notes",
    header: "Notes",
    cell: ({ row }) => {
      const part = row.original;
      return <NotesDialog partId={part.id} notes={part.notes} />;
    },
  },
  {
    accessorKey: "link",
    header: "Link",
    cell: ({ row }) => {
      const link = row.getValue("link") as string;
      return (
        <Button
          variant="ghost"
          size="sm"
          onClick={() => window.open(link, "_blank")}
        >
          <ExternalLink className="w-4 h-4 mr-2" />
          Open
        </Button>
      );
    },
  },
];

export default function PartsInventory() {
  const { data: parts, isLoading, isError, error } = useGetParts();

  const [searchTerm, setSearchTerm] = useState("");
  const [exhibitFilter, setExhibitFilter] = useState<string | null>(null);
  const [showFilters, setShowFilters] = useState<boolean>(false);
  const [sorting, setSorting] = useState<SortingState>([]);

  // Filter the parts data before passing it to the table
  const filteredParts =
    parts?.filter((part) => {
      if (
        searchTerm &&
        !part.name.toLowerCase().includes(searchTerm.toLowerCase())
      ) {
        return false;
      }
      if (exhibitFilter && !part.exhibit_ids.includes(exhibitFilter)) {
        return false;
      }
      return true;
    }) || [];

  // Pass the filtered data to the table
  const table = useReactTable({
    data: filteredParts,
    columns,
    getCoreRowModel: getCoreRowModel(),
    onSortingChange: setSorting,
    getSortedRowModel: getSortedRowModel(),
    state: {
      sorting,
    },
  });

  const clearFilters = () => {
    setExhibitFilter(null);
    setSearchTerm("");
    if (showFilters === true) {
      setShowFilters(false);
    }
  };

  const isFilterApplied = exhibitFilter !== null || searchTerm !== "";

  const uniqueExhibits = [
    ...new Set((parts ?? []).flatMap((p) => p.exhibit_ids)),
  ];

  const filterOptions = [
    {
      value: exhibitFilter,
      onChange: setExhibitFilter,
      options: uniqueExhibits,
      placeholder: "Filter by Exhibit",
    },
  ];

  const filteredPartsCount = filteredParts.length;

  return (
    <div className="container mx-auto p-4">
      <div className="flex justify-between w-full">
        <h1 className="text-2xl font-bold mb-6 mt-[0.1rem]">Part Inventory</h1>
        <CreatePartDialog />
      </div>
      <FilterSection
        showFilters={showFilters}
        setShowFilters={setShowFilters}
        clearFilters={clearFilters}
        isFilterApplied={isFilterApplied}
        setSearchTerm={setSearchTerm}
        filterOptions={filterOptions}
        searchBarName="parts"
      />
      {isLoading ? (
        <Loading />
      ) : isError || !parts ? (
        <Error error={error} name="exhibits" />
      ) : (
        <ScrollArea className="h-[calc(100vh-200px)]">
          <div className="rounded-md border">
            <Table>
              <TableHeader>
                {table.getHeaderGroups().map((headerGroup) => (
                  <TableRow key={headerGroup.id}>
                    {headerGroup.headers.map((header) => (
                      <TableHead key={header.id}>
                        {header.isPlaceholder
                          ? null
                          : flexRender(
                              header.column.columnDef.header,
                              header.getContext()
                            )}
                      </TableHead>
                    ))}
                  </TableRow>
                ))}
              </TableHeader>
              <TableBody>
                {table.getRowModel().rows?.length ? (
                  table.getRowModel().rows.map((row) => (
                    <TableRow
                      key={row.id}
                      data-state={row.getIsSelected() && "selected"}
                    >
                      {row.getVisibleCells().map((cell) => (
                        <TableCell key={cell.id}>
                          {flexRender(
                            cell.column.columnDef.cell,
                            cell.getContext()
                          )}
                        </TableCell>
                      ))}
                    </TableRow>
                  ))
                ) : (
                  <TableRow>
                    <TableCell
                      colSpan={columns.length}
                      className="h-24 text-center"
                    >
                      No results.
                    </TableCell>
                  </TableRow>
                )}
              </TableBody>
            </Table>
          </div>
        </ScrollArea>
      )}
      <div className="mt-4 text-sm text-muted-foreground">
        Showing {filteredPartsCount} of {parts ? parts.length : 0} parts
      </div>
    </div>
  );
}

function ConnectedExhibitsDisplay({ exhibitIds }: { exhibitIds: string[] }) {
  return (
    <ul className="list-disc pl-4">
      {exhibitIds.map((exhibitId, index) => (
        <li key={index}>{exhibitId}</li>
      ))}
    </ul>
  );
}

interface NotesDialogProps {
  partId: string;
  notes: Note[];
}

function NotesDialog({ partId, notes }: NotesDialogProps) {
  const [isOpen, setIsOpen] = useState(false);
  const { register, handleSubmit, reset } = useForm<{ message: string }>();
  const createNote = useCreatePartNote();
  const deleteNote = useDeletePartNote();

  const onSubmit = async (data: { message: string }) => {
    try {
      await createNote.mutateAsync({
        partId,
        note: { message: data.message },
      });
      reset();
    } catch (error) {
      console.error("Error creating note:", error);
    }
  };

  const handleDelete = async (noteId: string) => {
    try {
      await deleteNote.mutateAsync({ partId, noteId });
    } catch (error) {
      console.error("Error deleting note:", error);
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button variant="outline" size="sm">
          <StickyNote className="w-4 h-4 mr-2" />
          {notes.length}
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>Notes</DialogTitle>
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
