import { Card, CardContent } from "@/components/ui/card";
import { Star } from "lucide-react";
import { calculateTimeUntilExpiration } from "@/lib/date";
import type { Sponsorship } from "@/types";

interface SponsorshipDisplayProps {
  sponsorship?: Sponsorship;
}

export function SponsorshipDisplay({ sponsorship }: SponsorshipDisplayProps) {
  if (!sponsorship) {
    return null;
  }

  return (
    <Card className="mb-4">
      <CardContent className="p-4">
        <div className="flex items-center space-x-2 mb-2">
          <Star className="h-5 w-5 text-yellow-500" />
          <h4 className="text-sm font-semibold">Sponsorship</h4>
        </div>
        <div className="space-y-1 text-sm">
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
      </CardContent>
    </Card>
  );
}
