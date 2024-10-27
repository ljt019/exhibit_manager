import useGetExhibits from "@/hooks/useGetExhibits";
import { ExhibitList } from "@/components/exhibit-list";
import { FilterSection } from "@/components/filter-section";
import { CreateExhibitDialog } from "@/components/create-exhibit-dialog";
import { Loading, Error } from "@/components/loading-and-error";
import { useExhibitFilters } from "@/hooks/filters/useExhibitFilters";

export default function ExhibitInventory() {
  const { data: exhibits, isLoading, isError, error } = useGetExhibits();
  const {
    setSearchTerm,
    showFilters,
    setShowFilters,
    filteredExhibits,
    clearFilters,
    isFilterApplied,
    filterOptions,
  } = useExhibitFilters(exhibits || []);

  if (isLoading) return <Loading />;
  if (isError || !exhibits) return <Error error={error} name="exhibits" />;

  return (
    <div className="container mx-auto p-4">
      <Header />
      <FilterSection
        showFilters={showFilters}
        setShowFilters={setShowFilters}
        clearFilters={clearFilters}
        isFilterApplied={isFilterApplied}
        setSearchTerm={setSearchTerm}
        filterOptions={filterOptions}
        searchBarName="exhibits"
      />
      <ExhibitList filteredExhibits={filteredExhibits} />
      <Footer
        totalExhibits={exhibits.length}
        filteredExhibits={filteredExhibits.length}
      />
    </div>
  );
}

function Header() {
  return (
    <div className="flex justify-between items-center mb-6">
      <h1 className="text-2xl font-bold">Exhibit Inventory</h1>
      <CreateExhibitDialog />
    </div>
  );
}

interface FooterProps {
  totalExhibits: number;
  filteredExhibits: number;
}

function Footer({ totalExhibits, filteredExhibits }: FooterProps) {
  return (
    <div className="mt-4 text-sm text-muted-foreground">
      Showing {filteredExhibits} of {totalExhibits} exhibits
    </div>
  );
}
