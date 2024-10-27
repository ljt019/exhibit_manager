import { useState, useMemo, useEffect } from "react";
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
  ColumnFiltersState,
  getFilteredRowModel,
} from "@tanstack/react-table";
import { Button } from "@/components/ui/button";
import { ArrowUpDown, ExternalLink, Atom, StickyNote } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import useGetParts from "@/hooks/useGetParts";
import { FilterSection } from "@/components/filter-section";

export type Part = {
  id: string;
  name: string;
  link: string;
  exhibit_ids: Array<string>;
  notes: Array<{ timestamp: string; note: string }>;
};

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
            <ul className="list-disc pl-4">
              {exhibit_ids && exhibit_ids.length > 0 ? (
                exhibit_ids.map((exhibitId, index) => (
                  <li key={index}>{exhibitId}</li>
                ))
              ) : (
                <li>No exhibits attached</li>
              )}
            </ul>
          </DialogContent>
        </Dialog>
      );
    },
  },
  {
    accessorKey: "notes",
    header: "Notes",
    cell: ({ row }) => {
      const notes = row.getValue("notes") as Array<{
        timestamp: string;
        note: string;
      }>;
      return (
        <Dialog>
          <DialogTrigger asChild>
            <Button variant="outline" size="sm">
              <StickyNote className="w-4 h-4 mr-2" />
              {notes.length}
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Notes</DialogTitle>
              <DialogDescription>Notes for this part:</DialogDescription>
            </DialogHeader>
            <ul className="list-disc pl-4">
              {notes.map((note, index) => (
                <li key={index} className="mb-2">
                  <span className="font-semibold">{note.timestamp}:</span>{" "}
                  {note.note}
                </li>
              ))}
            </ul>
          </DialogContent>
        </Dialog>
      );
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
  const { data: parts, isLoading, isError, error, refetch } = useGetParts();

  const [searchTerm, setSearchTerm] = useState("");
  const [exhibitFilter, setExhibitFilter] = useState<string | null>(null);
  const [showFilters, setShowFilters] = useState<boolean>(false);
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);

  useEffect(() => {
    // Refetch data when component mounts
    refetch();

    // Reset state
    setSearchTerm("");
    setExhibitFilter(null);
    setShowFilters(false);
    setSorting([]);
    setColumnFilters([]);
  }, [refetch]);

  const filteredParts = useMemo(() => {
    if (!parts) return [];

    return parts.filter((part) => {
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
    });
  }, [parts, searchTerm, exhibitFilter]);

  const table = useReactTable({
    data: filteredParts,
    columns,
    getCoreRowModel: getCoreRowModel(),
    onSortingChange: setSorting,
    getSortedRowModel: getSortedRowModel(),
    onColumnFiltersChange: setColumnFilters,
    getFilteredRowModel: getFilteredRowModel(),
    state: {
      sorting,
      columnFilters,
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

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (isError || !parts) {
    return (
      <div>Error fetching parts {error && <div>{error.toString()}</div>}</div>
    );
  }

  const uniqueExhibits = [...new Set(parts.flatMap((p) => p.exhibit_ids))];

  const filterOptions = [
    {
      value: exhibitFilter,
      onChange: setExhibitFilter,
      options: uniqueExhibits,
      placeholder: "Filter by Exhibit",
    },
  ];

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-6">Parts Inventory</h1>
      <FilterSection
        showFilters={showFilters}
        setShowFilters={setShowFilters}
        clearFilters={clearFilters}
        isFilterApplied={isFilterApplied}
        setSearchTerm={setSearchTerm}
        filterOptions={filterOptions}
        searchBarName="parts"
      />
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
      <div className="mt-4 text-sm text-muted-foreground">
        Showing {filteredParts.length} of {parts.length} parts
      </div>
    </div>
  );
}
