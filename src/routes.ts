import SignIn from "@/screens/sign-in";
import Parts from "@/screens/parts";
import Exhibits from "@/screens/exhibits";
import Issues from "@/screens/issues";

import { Frame, SquareTerminal, RabbitIcon } from "lucide-react";

export interface RouteConfig {
  url: string;
  screen: React.ComponentType;
  isActive: boolean;
  title: string;
  sidebar: boolean;
  icon?: React.ComponentType;
  items?: {
    title: string;
    url: string;
    isActive: boolean;
  }[];
}

export const routes: RouteConfig[] = [
  {
    url: "/",
    screen: SignIn,
    title: "Sign In",
    isActive: false,
    sidebar: false,
  },
  {
    url: "/exhibits",
    screen: Exhibits,
    title: "Exhibits",
    icon: SquareTerminal,
    isActive: true,
    sidebar: true,
  },
  {
    url: "/parts",
    screen: Parts,
    title: "Parts",
    icon: Frame,
    isActive: false,
    sidebar: true,
  },
  {
    url: "/issues",
    screen: Issues,
    title: "Issues",
    icon: RabbitIcon,
    isActive: false,
    sidebar: true,
  },
];
