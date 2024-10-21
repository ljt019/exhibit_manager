import { HashRouter as Router, Routes, Route } from "react-router-dom";
import { Index } from "@/screens/index";
import Dashboard from "@/screens/dashboard";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AppSidebar } from "@/components/app-sidebar";
import { SidebarProvider, SidebarInset } from "@/components/ui/sidebar";

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
        <AppSidebar />
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
        <Routes>
          <Route path="/" element={<Index />} />
          <Route
            path="/dashboard"
            element={
              <Layout>
                <Dashboard />
              </Layout>
            }
          />
        </Routes>
      </QueryClientProvider>
    </Router>
  );
}

export default App;
