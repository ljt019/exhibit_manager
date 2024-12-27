import useGetJotformList from "@/hooks/data/queries/jotforms/useGetJotformList";
import { Loading, Error } from "@/components/loading-and-error";
import { Check } from "lucide-react";
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
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  ChevronDown,
  ChevronRight,
  CircleArrowUp,
  CircleArrowDown,
  CircleArrowRight,
} from "lucide-react";
import { cn } from "@/lib/utils";
import { motion, AnimatePresence } from "framer-motion";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import useChangeStatus from "@/hooks/data/mutations/jotforms/useChangeStatus";
import { FilterSection } from "@/components/filter-section";
import { Jotform } from "@/types";
import type {
  Status,
  NewStatusRequest,
} from "@/hooks/data/mutations/jotforms/useChangeStatus";

export default function Jotforms() {
  const { data: jotforms, isPending, isError, error } = useGetJotformList();
  const [searchTerm, setSearchTerm] = useState("");
  const [showFilters, setShowFilters] = useState(false);
  const [department, setDepartment] = useState<string | null>(null);
  const [status, setStatus] = useState<string | null>(null);

  const filteredJotforms =
    jotforms?.filter(
      (jotform) =>
        (searchTerm === "" ||
          Object.values(jotform).some((value) =>
            String(value).toLowerCase().includes(searchTerm.toLowerCase())
          )) &&
        (!department || jotform.department === department) &&
        (!status || jotform.status === status)
    ) || [];

  return (
    <div className="container mx-auto p-4 mt-[0.1rem]">
      <Header />
      {isPending ? (
        <Loading />
      ) : isError || !jotforms ? (
        <Error error={error} name="jotforms" />
      ) : (
        <>
          <FilterSection
            showFilters={showFilters}
            setShowFilters={setShowFilters}
            clearFilters={() => {
              setSearchTerm("");
              setDepartment(null);
              setStatus(null);
            }}
            isFilterApplied={!!searchTerm || !!department || !!status}
            setSearchTerm={setSearchTerm}
            filterOptions={[
              {
                value: department,
                onChange: setDepartment,
                options: ["Operations", "Exhibits", "N/A"],
                placeholder: "Department",
              },
              {
                value: status,
                onChange: setStatus,
                options: ["Open", "InProgress", "Closed", "Unplanned"],
                placeholder: "Status",
              },
            ]}
            searchBarName="Jotforms"
          />
          <JotformsTable jotforms={filteredJotforms} />
        </>
      )}
      <Footer
        totalJotforms={jotforms ? jotforms.length : 0}
        filteredJotforms={filteredJotforms.length}
      />
    </div>
  );
}

function Header() {
  return (
    <div className="flex justify-between items-center mb-6">
      <h1 className="text-2xl font-bold">Jotforms</h1>
    </div>
  );
}

interface FooterProps {
  totalJotforms: number;
  filteredJotforms: number;
}

function Footer({ totalJotforms, filteredJotforms }: FooterProps) {
  return (
    <div className="mt-4 text-sm text-muted-foreground">
      Showing {filteredJotforms} of {totalJotforms} jotforms
    </div>
  );
}

interface ExpandedState {
  [key: string]: boolean;
}

export function JotformsTable({ jotforms }: { jotforms: Array<Jotform> }) {
  const [expandedRows, setExpandedRows] = useState<ExpandedState>({});

  const changeStatus = useChangeStatus();

  const sortedData = jotforms.sort((a: Jotform, b: Jotform) => {
    if (a.status === "InProgress" && b.status !== "InProgress") return -1;
    if (a.status !== "InProgress" && b.status === "InProgress") return 1;
    if (a.status === "Unplanned" && b.status !== "Unplanned") return 1;
    if (a.status !== "Unplanned" && b.status === "Unplanned") return -1;
    return 0;
  });

  const toggleRow = (id: string) => {
    setExpandedRows((prev) => ({
      ...prev,
      [id]: !prev[id],
    }));
  };

  const handleStatusChange = (jotformId: string, newStatus: string) => {
    if (
      jotforms.find((jotform: Jotform) => jotform.id === jotformId)?.status ===
      newStatus
    )
      return;

    if (!["Open", "InProgress", "Closed", "Unplanned"].includes(newStatus))
      return;

    const newStatusRequest: NewStatusRequest = {
      jotformId,
      status: newStatus as Status,
    };

    changeStatus.mutate(newStatusRequest, {
      onSuccess: () => {
        // Manually update the local state to ensure immediate UI update
        setExpandedRows((prev) => ({
          ...prev,
          [jotformId]: false,
        }));
      },
    });
  };

  const getPriorityBadge = (priority: string) => {
    const variants = {
      High: {
        class: "bg-red-500/10 text-red-500 hover:bg-red-500/20",
        icon: <CircleArrowUp className="w-4 h-4" />,
      },
      Medium: {
        class: "bg-yellow-500/10 text-yellow-500 hover:bg-yellow-500/20",
        icon: <CircleArrowRight className="w-4 h-4" />,
      },
      Low: {
        class: "bg-green-500/10 text-green-500 hover:bg-green-500/20",
        icon: <CircleArrowDown className="w-4 h-4" />,
      },
    };
    return variants[priority as keyof typeof variants] || variants["Medium"];
  };

  const getStatusBadge = (status: string) => {
    const variants = {
      Open: "bg-green-500/10 text-green-500 hover:bg-green-500/20",
      InProgress: "bg-purple-500/10 text-purple-500 hover:bg-purple-500/20",
      Closed: "bg-red-500/10 text-red-500 hover:bg-red-500/20",
      Unplanned: "bg-gray-500/10 text-gray-500 hover:bg-gray-500/20",
    };
    return variants[status as keyof typeof variants] || variants["Open"];
  };

  const getDepartmentBadge = (department: string) => {
    const variants = {
      Operations: "bg-sky-500/10 text-sky-500 hover:bg-sky-500/20",
      Exhibits: "bg-orange-500/10 text-orange-500 hover:bg-orange-500/20 ",
      "N/A": "bg-gray-500/10 text-gray-500 hover:bg-gray-500/20",
    };
    return variants[department as keyof typeof variants] || variants["N/A"];
  };

  return (
    <ScrollArea className="h-[calc(100vh-200px)] w-full">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[30px]"></TableHead>
            <TableHead>Name</TableHead>
            <TableHead>Date</TableHead>
            <TableHead>Location</TableHead>
            <TableHead>Exhibit Name</TableHead>
            <TableHead>Priority</TableHead>
            <TableHead>Department</TableHead>
            <TableHead>Status</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {sortedData.map((jotform: Jotform) => (
            <AnimatePresence key={jotform.id}>
              <TableRow
                className={cn(
                  "hover:bg-muted/50 transition-colors",
                  jotform.status === "InProgress" &&
                    "bg-gradient-to-r from-purple-800/20 to-black/80"
                )}
                onClick={() => toggleRow(jotform.id)}
              >
                <TableCell className="w-[30px]">
                  <Button variant="ghost" size="icon" className="h-6 w-6">
                    {expandedRows[jotform.id] ? (
                      <ChevronDown className="h-4 w-4" />
                    ) : (
                      <ChevronRight className="h-4 w-4" />
                    )}
                  </Button>
                </TableCell>
                <TableCell className="font-medium">
                  {jotform.submitter_name.first}
                </TableCell>
                <TableCell className="text-muted-foreground">
                  {jotform.created_at.date}
                </TableCell>
                <TableCell>{jotform.location}</TableCell>
                <TableCell>{jotform.exhibit_name}</TableCell>
                <TableCell>
                  <Badge
                    variant="secondary"
                    className={cn(
                      "font-medium items-center gap-1",
                      getPriorityBadge(jotform.priority_level).class
                    )}
                  >
                    {jotform.priority_level}
                    {getPriorityBadge(jotform.priority_level).icon}
                  </Badge>
                </TableCell>
                <TableCell>
                  <Badge
                    variant="secondary"
                    className={cn(getDepartmentBadge(jotform.department))}
                  >
                    {jotform.department}
                  </Badge>
                </TableCell>
                <TableCell>
                  <DropdownMenu>
                    <DropdownMenuTrigger>
                      <Badge
                        variant="secondary"
                        className={cn(
                          getStatusBadge(jotform.status),
                          "hover:opacity-80 transition-opacity"
                        )}
                      >
                        {jotform.status}
                      </Badge>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent
                      className="w-20"
                      side="bottom"
                      align="start"
                    >
                      {["Open", "InProgress", "Closed", "Unplanned"].map(
                        (status) => (
                          <DropdownMenuItem
                            key={status}
                            onClick={(e) => {
                              e.stopPropagation();
                              handleStatusChange(jotform.id, status);
                            }}
                            className={cn(
                              "flex items-center text-center justify-between rounded-none",
                              getStatusBadge(status),
                              "hover:opacity-80 transition-opacity"
                            )}
                          >
                            {status}
                            {jotform.status === status && (
                              <Check className="h-4 w-4 text-current" />
                            )}
                          </DropdownMenuItem>
                        )
                      )}
                    </DropdownMenuContent>
                  </DropdownMenu>
                </TableCell>
              </TableRow>
              {expandedRows[jotform.id] && (
                <motion.tr
                  initial={{ opacity: 0, height: 0 }}
                  animate={{ opacity: 1, height: "auto" }}
                  exit={{ opacity: 0, height: 0 }}
                  transition={{ duration: 0.2 }}
                >
                  <TableCell colSpan={8} className="bg-muted/50">
                    <motion.div
                      initial={{ opacity: 0, y: -10 }}
                      animate={{ opacity: 1, y: 0 }}
                      exit={{ opacity: 0, y: -10 }}
                      transition={{ duration: 0.2 }}
                    >
                      <div className="p-4">
                        <h4 className="text-sm font-semibold mb-2">
                          Description
                        </h4>
                        <p className="text-sm text-muted-foreground">
                          {jotform.description}
                        </p>
                      </div>
                    </motion.div>
                  </TableCell>
                </motion.tr>
              )}
            </AnimatePresence>
          ))}
        </TableBody>
      </Table>
    </ScrollArea>
  );
}
