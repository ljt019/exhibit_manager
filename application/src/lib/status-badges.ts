export const exhibitStatusClasses: Record<string, string> = {
  Operational: "bg-green-500/10 text-green-500 hover:bg-green-500/20",
  "Needs Repair": "bg-purple-500/10 text-purple-500 hover:bg-purple-500/20",
  "Out of Service": "bg-red-500/10 text-red-500 hover:bg-red-500/20",
};

export function getExhibitStatusBadge(status: string): string {
  const key =
    status.charAt(0).toUpperCase() + status.slice(1).toLowerCase();
  return (
    exhibitStatusClasses[key as keyof typeof exhibitStatusClasses] ||
    exhibitStatusClasses["Operational"]
  );
}

export const jotformStatusClasses: Record<string, string> = {
  Open: "bg-green-500/10 text-green-500 hover:bg-green-500/20",
  InProgress: "bg-purple-500/10 text-purple-500 hover:bg-purple-500/20",
  Closed: "bg-red-500/10 text-red-500 hover:bg-red-500/20",
  Unplanned: "bg-gray-500/10 text-gray-500 hover:bg-gray-500/20",
};

export function getJotformStatusBadge(status: string): string {
  return (
    jotformStatusClasses[status as keyof typeof jotformStatusClasses] ||
    jotformStatusClasses.Open
  );
}
