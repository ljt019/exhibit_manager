import SignIn from "@/screens/sign-in";
import Parts from "@/screens/parts";
import Exhibits from "@/screens/exhibits";
import Issues from "@/screens/issues";

import { Atom, Bolt, CircleDot } from "lucide-react";

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
  disabled?: boolean;
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
    icon: Atom,
    isActive: true,
    sidebar: true,
  },
  {
    url: "/parts",
    screen: Parts,
    title: "Parts",
    icon: Bolt,
    isActive: false,
    sidebar: true,
    disabled: true,
  },
  {
    url: "/issues",
    screen: Issues,
    title: "Issues",
    icon: CircleDot,
    isActive: false,
    sidebar: true,
    disabled: true,
  },
];
