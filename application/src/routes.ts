import SignIn from "@/screens/sign-in";
import Exhibits from "@/screens/exhibits";
import Parts from "@/screens/parts";

import { Atom, Bolt } from "lucide-react";

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
  },
  /* 
  {
    url: "/issues",
    screen: Issues,
    title: "Issues",
    icon: CircleDot,
    isActive: false,
    sidebar: true,
  },
  {
    url: "/jotforms",
    screen: Jotforms,
    title: "Jotforms",
    icon: NotebookPen,
    isActive: false,
    sidebar: true,
  },
  */
];
