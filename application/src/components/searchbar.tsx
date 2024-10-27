import { Search } from "lucide-react";
import { Input } from "@/components/ui/input";
import { debounce } from "lodash";
import { useCallback } from "react";

interface SearchBarProps {
  setSearchTerm: (term: string) => void;
  name: string;
}

export function SearchBar({ setSearchTerm, name }: SearchBarProps) {
  const debouncedSetSearchTerm = useCallback(
    debounce((value: string) => setSearchTerm(value), 300),
    []
  );

  return (
    <div className="flex-1 relative">
      <Search className="absolute left-2 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
      <Input
        type="text"
        placeholder={`Search ${name}...`}
        onChange={(e) => debouncedSetSearchTerm(e.target.value)}
        className="w-full pl-8"
      />
    </div>
  );
}
