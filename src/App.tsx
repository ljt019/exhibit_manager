import {
  HashRouter as Router,
  Routes as PrimitiveRoutes,
  Route,
} from "react-router-dom";
import Index from "@/screens/index";
import Parts from "@/screens/parts";
import Dashboard from "@/screens/exhibits";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AppSidebar } from "@/components/app-sidebar";
import { SidebarProvider, SidebarInset } from "@/components/ui/sidebar";
import { Frame, SquareTerminal } from "lucide-react";

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

const routes: RouteConfig[] = [
  {
    url: "/",
    screen: Index,
    title: "Sign In",
    isActive: false,
    sidebar: false,
  },
  {
    url: "/exhibits",
    screen: Dashboard,
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
];

function Routes() {
  return (
    <PrimitiveRoutes>
      {routes.map((route) =>
        route.sidebar ? (
          <Route
            key={route.url}
            path={route.url}
            element={
              <Layout>
                <route.screen />
              </Layout>
            }
          />
        ) : (
          <Route key={route.url} path={route.url} element={<route.screen />} />
        )
      )}
    </PrimitiveRoutes>
  );
}

function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="h-screen overflow-hidden">
      <SidebarProvider
        style={
          {
            "--sidebar-width": "19rem",
          } as React.CSSProperties
        }
      >
        <AppSidebar routeConfig={routes} />
        <SidebarInset>
          <div className="mt-[1rem]">{children}</div>
        </SidebarInset>
      </SidebarProvider>
    </div>
  );
}

function App() {
  let queryClient = new QueryClient();

  return (
    <Router>
      <QueryClientProvider client={queryClient}>
        <Routes />
      </QueryClientProvider>
    </Router>
  );
}

export default App;
