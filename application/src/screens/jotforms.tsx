import useGetJotformList from "@/hooks/data/queries/jotforms/useGetJotformList";
import { Loading, Error } from "@/components/loading-and-error";
import { useState } from "react";
import { FilterSection } from "@/components/filter-section";
import { JotformsTable } from "@/components/tables/jotforms-table";

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
