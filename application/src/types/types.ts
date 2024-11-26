export interface UserProfile {
  id: string;
  name: string;
  given_name: string;
  family_name: string;
  picture: string | null;
}

export type Note = { timestamp: string; note: string };

export type Sponsorship = {
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
  notes: Array<Note>;
  image_url: string | undefined;
  sponsorship?: Sponsorship;
};

export type Part = {
  id: string;
  name: string;
  link: string;
  exhibit_ids: Array<string>;
  notes: Array<Note>;
};
